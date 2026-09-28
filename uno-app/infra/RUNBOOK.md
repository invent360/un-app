# UNO App Operations Runbook

This runbook provides procedures for operating the uno-app infrastructure.

## Table of Contents

1. [Quick Start (MVP)](#quick-start-mvp)
2. [Overall Deployment Flow](#overall-deployment-flow)
3. [Local Development](#local-development)
4. [Deployment Strategies](#deployment-strategies)
5. [Architecture Overview](#architecture-overview)
6. [Helm Chart Reference](#helm-chart-reference)
7. [Deployment Procedures](#deployment-procedures)
8. [Monitoring & Alerting](#monitoring--alerting)
9. [Incident Response](#incident-response)
10. [Troubleshooting](#troubleshooting)
11. [Database Operations](#database-operations)
12. [Disaster Recovery](#disaster-recovery)
13. [Routine Maintenance](#routine-maintenance)
14. [Useful Commands](#useful-commands)

---

## Quick Start (MVP)

**Recommended for MVP**: Deploy using the Europe strategy at ~$150/month.

### Using Make (Recommended)

```bash
# Show all available commands
make help

# Local development (minikube + integrated PostgreSQL)
make up                    # Setup minikube and deploy
make logs                  # Follow logs
make health                # Check health
make down                  # Cleanup

# Production deployment
export DATABASE_URL="postgres://..."
export ADMIN_API_KEY="..."
make k8s-secrets-prod ENV=europe
make helm-deploy ENV=europe
```

### Using Bootstrap Script

```bash
# 1. Set required environment variables
export DATABASE_URL="postgres://..."
export ADMIN_API_KEY="..."
export CLOUDFLARE_TUNNEL_TOKEN="..."
export GRAFANA_API_KEY="..."

# 2. Deploy Europe strategy
./infra/scripts/bootstrap.sh deploy-strategy europe

# 3. Verify deployment
./infra/scripts/bootstrap.sh verify
```

**Users from all regions can access the app** - traffic routes through Cloudflare to your single Europe deployment. Latency varies by distance:

| User Location | Latency |
|---------------|---------|
| Europe | 20-50ms |
| Middle East | 50-100ms |
| Africa | 100-200ms |
| Americas | 150-250ms |
| Asia-Pacific | 200-350ms |

When ready to scale, add more regions (see [Deployment Strategies](#deployment-strategies)).

---

## Overall Deployment Flow

### Components Deployed

| Component | Pod Name | Service Name | Description |
|-----------|----------|--------------|-------------|
| uno-app | uno-app-xxx | uno-app | Main application |
| PostgreSQL | uno-db-0 | uno-db, uno-db-hl | Database (when internal) |
| Cloudflared | cloudflared-xxx | - | Cloudflare tunnel (when enabled) |
| Grafana Agent | grafana-agent-xxx | - | Monitoring (when enabled) |

### Deployment Modes Summary

| Mode | Database | Command |
|------|----------|---------|
| **Local Dev** | In-cluster standalone (uno-db) | `helm upgrade --install uno-app . -f values.yaml -f values-strategy-local.yaml -n uno-app` |
| **Prod (External DB)** | Cloud SQL / RDS | `helm upgrade --install uno-app . -f values.yaml -f values-strategy-europe.yaml -n uno-app` |
| **Prod (Internal HA)** | In-cluster HA (uno-db + replicas) | `helm upgrade --install uno-app . -f values.yaml -f values-strategy-europe.yaml -f values-database-ha.yaml -n uno-app` |

### Step-by-Step Deployment

#### 1. Prerequisites

```bash
# Update Helm dependencies
cd infra/kubernetes/charts/uno-app
helm dependency update

# Create namespace
kubectl create namespace uno-app
```

#### 2. Create Secrets

```bash
# Admin credentials (always required)
kubectl create secret generic admin-credentials \
  --namespace uno-app \
  --from-literal=api_key="$ADMIN_API_KEY" \
  --from-literal=client_id="$ADMIN_CLIENT_ID" \
  --from-literal=secret_key="$ADMIN_SECRET_KEY"

# R2 storage (always required)
kubectl create secret generic r2-credentials \
  --namespace uno-app \
  --from-literal=access_key_id="$R2_ACCESS_KEY" \
  --from-literal=secret_access_key="$R2_SECRET_KEY" \
  --from-literal=endpoint="$R2_ENDPOINT" \
  --from-literal=bucket_name="$R2_BUCKET"

# Database credentials (for external DB mode only)
kubectl create secret generic db-credentials \
  --namespace uno-app \
  --from-literal=url="$DATABASE_URL"
```

#### 3. Deploy

```bash
# Local development
helm upgrade --install uno-app . \
  -f values.yaml \
  -f values-strategy-local.yaml \
  -n uno-app --wait

# Production (with external database)
helm upgrade --install uno-app . \
  -f values.yaml \
  -f values-strategy-europe.yaml \
  -n uno-app --wait
```

#### 4. Verify

```bash
# Check pods
kubectl get pods -n uno-app

# Check health
kubectl exec -n uno-app deploy/uno-app -- curl -s http://localhost:3000/api/v1/health
# Expected: {"status":"ok","version":"0.1.0","database":"connected"}

# Check services
kubectl get svc -n uno-app
```

---

## Local Development

Test the full Kubernetes deployment locally before deploying to production.

### Prerequisites

Install one of the following local Kubernetes options:

| Tool | Install Command | Best For |
|------|-----------------|----------|
| **minikube** | `brew install minikube` | Full-featured, multiple drivers |
| **k3d** | `brew install k3d` | Lightweight, fast startup |
| **kind** | `brew install kind` | CI/CD, Docker-based |
| **Docker Desktop** | Enable in Docker settings | macOS/Windows, simple setup |

### Option 1: minikube (Recommended)

```bash
# Using Make
make minikube-start

# Or manually
minikube start --cpus=4 --memory=8192 --driver=docker

# Enable ingress addon (optional)
minikube addons enable ingress

# Verify cluster is running
kubectl get nodes
```

### Option 2: k3d (Lightweight)

```bash
# Create cluster with port mapping
k3d cluster create uno-local \
  --port "30000:30000@server:0" \
  --port "8080:80@loadbalancer"

# Verify cluster
kubectl get nodes
```

### Option 3: kind

```bash
# Create cluster with port mapping
cat <<EOF | kind create cluster --config=-
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
nodes:
- role: control-plane
  extraPortMappings:
  - containerPort: 30000
    hostPort: 30000
    protocol: TCP
EOF

# Verify cluster
kubectl get nodes
```

### Deploy Locally (Recommended: Integrated PostgreSQL)

The Helm chart includes PostgreSQL as a sub-chart. For local development, it deploys a single-node PostgreSQL instance automatically.

#### Quick Start with Make

```bash
# One command to setup everything
make up

# Or step by step
make dev-setup      # Start minikube, update deps, create secrets
make dev-deploy     # Deploy uno-app + PostgreSQL
make health         # Verify deployment
```

#### Manual Deployment

```bash
cd infra/kubernetes/charts/uno-app

# Update Helm dependencies (downloads PostgreSQL chart)
helm dependency update

# Create namespace
kubectl create namespace uno-app

# Create admin secrets (required)
kubectl create secret generic admin-credentials \
  --namespace uno-app \
  --from-literal=api_key="local-dev-key" \
  --from-literal=client_id="local-admin" \
  --from-literal=secret_key="local-secret"

kubectl create secret generic r2-credentials \
  --namespace uno-app \
  --from-literal=access_key_id="local" \
  --from-literal=secret_access_key="local" \
  --from-literal=endpoint="http://localhost:9000" \
  --from-literal=bucket_name="local"

# Deploy with local values (includes PostgreSQL)
helm upgrade --install uno-app . \
  -f values.yaml \
  -f values-strategy-local.yaml \
  --namespace uno-app \
  --wait --timeout 5m

# Verify deployment
kubectl get pods -n uno-app
```

**Expected output:**
```
NAME                       READY   STATUS    AGE
uno-app-xxxxxxxxx-xxxxx    1/1     Running   1m
uno-db-0                   1/1     Running   1m
```

### Verify Database Connection

```bash
# Check health endpoint
kubectl exec -n uno-app deploy/uno-app -- curl -s http://localhost:3000/api/v1/health

# Expected: {"status":"ok","version":"0.1.0","database":"connected"}

# If database shows "disconnected", restart the app pod
kubectl delete pod -l app=uno-app -n uno-app
```

### Access the Application

```bash
# Option 1: NodePort (configured in values-strategy-local.yaml)
# Access at http://localhost:30000

# Option 2: Port forward
kubectl port-forward -n uno-app svc/uno-app 3000:3000
# Access at http://localhost:3000

# Option 3: minikube service (minikube only)
minikube service uno-app -n uno-app
```

### Database Access

```bash
# Connect to PostgreSQL
kubectl exec -it -n uno-app uno-db-0 -- bash -c 'PGPASSWORD="$POSTGRES_PASSWORD" psql -U postgres -d uno'

# Run SQL queries
\dt                    # List tables
SELECT * FROM users;   # Query data
\q                     # Exit
```

### Local Development Workflow

#### Using Make (Recommended)

```bash
# 1. Make code changes

# 2. Rebuild and redeploy
make dev-rebuild            # Build in minikube + restart pods

# 3. Watch logs
make logs                   # Follow logs
make health                 # Check health
```

#### Manual Workflow

```bash
# 1. Make code changes

# 2. Build and push image to local registry (minikube)
eval $(minikube docker-env)
docker build -t katson360/uno-app:local .

# 3. Update deployment
helm upgrade uno-app . \
  -f values.yaml \
  -f values-strategy-local.yaml \
  --namespace uno-app

# 4. Watch logs
kubectl logs -f -l app=uno-app -n uno-app
```

### Alternative: External PostgreSQL (Docker)

If you prefer running PostgreSQL outside Kubernetes:

```bash
# Start PostgreSQL container on host
docker run -d \
  --name postgres-local \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=uno \
  -p 5432:5432 \
  postgres:15

# Deploy WITHOUT internal PostgreSQL
helm upgrade --install uno-app . \
  -f values.yaml \
  -f values-strategy-local.yaml \
  --set database.internal.enabled=false \
  --set database.external.enabled=true \
  --namespace uno-app

# Create db-credentials secret pointing to host
kubectl create secret generic db-credentials \
  --namespace uno-app \
  --from-literal=url="postgres://postgres:postgres@host.minikube.internal:5432/uno"

# Restart uno-app to pick up new secret
kubectl rollout restart deployment/uno-app -n uno-app
```

### Cleanup

#### Using Make

```bash
make down           # Full cleanup: uninstall + delete namespace + stop minikube
# Or individually
make clean          # Uninstall Helm + delete PVCs + delete namespace
make minikube-stop  # Stop minikube
```

#### Manual Cleanup

```bash
# Delete the deployment (includes PostgreSQL)
helm uninstall uno-app -n uno-app

# Delete PVCs (database data)
kubectl delete pvc -n uno-app --all

# Delete namespace
kubectl delete namespace uno-app

# Stop local cluster
minikube stop      # or
k3d cluster delete uno-local  # or
kind delete cluster

# Stop external PostgreSQL (if used)
docker stop postgres-local && docker rm postgres-local
```

### Local vs Production Differences

| Feature | Local | Production |
|---------|-------|------------|
| Replicas | 1 | 2-8 |
| Database | In-cluster (uno-db) | External managed or HA internal |
| DB Architecture | Standalone | Replication (primary + replicas) |
| Cloudflared | Disabled | Enabled |
| Monitoring | Disabled | Enabled |
| Network Policies | Disabled | Enabled |
| Service Type | NodePort | ClusterIP |
| Resources | Minimal | Full |

---

## Deployment Strategies

Choose a deployment strategy based on your budget and target audience.

### Strategy Comparison

| Strategy | Monthly Cost | Regions Served | Best For |
|----------|-------------|----------------|----------|
| **Europe** | ~$150 | EU, ME, Africa | MVP, lowest cost |
| **India** | ~$250 | South Asia, Southeast Asia | India, Pakistan, Philippines |
| **India + Europe** | ~$320 | South Asia, SE Asia, EU, ME, Africa | India + EU + Africa |
| **Americas** | ~$294 | North/South America | US-focused |
| **Full Global** | ~$336 | All regions | Production scale |

---

### Strategy: Europe (~$150/month)

**Provider**: Hetzner Cloud (nbg1, Nuremberg)

Serves Western Europe, Eastern Europe, Middle East, and Africa.

#### Cloudflare Regions Served

| Code | Region |
|------|--------|
| WEU | Western Europe |
| EEU | Eastern Europe |
| ME | Middle East |
| NSAF | North & South Africa |

#### Latency

| Location | Latency |
|----------|---------|
| Europe | 20-50ms |
| Middle East | 50-100ms |
| Africa | 100-200ms |

#### Cost Breakdown

| Component | Spec | Monthly Cost |
|-----------|------|--------------|
| K3s Master | cx21 (2 vCPU, 4GB) | $7 |
| K3s Workers | 2x cx21 | $14 |
| PostgreSQL | cx31 (2 vCPU, 8GB) | $12 |
| Volume Storage | 100GB | $4 |
| Bandwidth | Included | $0 |
| **Hetzner Subtotal** | | **$37** |
| Cloudflare Business | CDN, LB, WAF | $100 |
| R2 Storage | ~50GB | $1 |
| **Total** | | **~$138-150** |

#### Deploy

```bash
./infra/scripts/bootstrap.sh deploy-strategy europe
```

#### Cloudflare Configuration

```hcl
fallback_pool_id = cloudflare_load_balancer_pool.europe.id
default_pool_ids = [cloudflare_load_balancer_pool.europe.id]
```

---

### Strategy: India (~$250/month)

**Provider**: AWS (ap-south-1, Mumbai)

Serves South Asia and Southeast Asia with optimal latency for India, Pakistan, Sri Lanka, Nepal, and Philippines.

#### Cloudflare Regions Served

| Code | Region |
|------|--------|
| SAS | South Asia (India, Pakistan, Sri Lanka, Nepal, Bangladesh) |
| SEAS | Southeast Asia (Philippines, Indonesia, Malaysia, Thailand) |

#### Latency

| Country | Latency |
|---------|---------|
| India | 10-30ms |
| Sri Lanka | 20-40ms |
| Bangladesh | 20-40ms |
| Pakistan | 30-50ms |
| Nepal | 30-50ms |
| Malaysia | 50-70ms |
| Thailand | 50-70ms |
| Indonesia | 60-80ms |
| Philippines | 80-100ms |

#### Cost Breakdown

| Component | Spec | Monthly Cost |
|-----------|------|--------------|
| K3s Master | t3.medium (2 vCPU, 4GB) | $30 |
| K3s Workers | 2x t3.medium | $60 |
| PostgreSQL | t3.medium + 100GB EBS | $40 |
| EBS Storage | 100GB gp3 | $8 |
| Bandwidth | ~500GB | $45 |
| **AWS Subtotal** | | **$183** |
| Cloudflare Business | CDN, LB, WAF | $100 |
| R2 Storage | ~50GB | $1 |
| **Total** | | **~$250-290** |

#### Deploy

```bash
./infra/scripts/bootstrap.sh deploy-strategy india
```

#### Cloudflare Configuration

```hcl
fallback_pool_id = cloudflare_load_balancer_pool.india.id
default_pool_ids = [cloudflare_load_balancer_pool.india.id]

region_pools {
  region   = "SAS"
  pool_ids = [cloudflare_load_balancer_pool.india.id]
}

region_pools {
  region   = "SEAS"
  pool_ids = [cloudflare_load_balancer_pool.india.id]
}
```

---

### Combined: India + Europe (~$320/month)

Deploy both strategies to cover South Asia, Southeast Asia, Europe, Middle East, and Africa.

#### Traffic Routing

| Traffic From | Routes To | Latency |
|--------------|-----------|---------|
| India, Pakistan, Nepal, Sri Lanka | Mumbai | 10-50ms |
| Philippines, Indonesia, Malaysia | Mumbai | 50-100ms |
| Europe | Nuremberg | 20-50ms |
| Middle East | Nuremberg | 50-100ms |
| Africa | Nuremberg | 100-200ms |

#### Cost Breakdown

| Component | Cost |
|-----------|------|
| India (AWS Mumbai) | $183 |
| Europe (Hetzner) | $37 |
| Cloudflare (shared) | $101 |
| **Total** | **~$320/month** |

#### Deploy

```bash
# 1. Deploy India strategy (primary database)
./infra/scripts/bootstrap.sh deploy-strategy india

# 2. Switch to Europe cluster
export KUBECONFIG=/path/to/europe-kubeconfig

# 3. Deploy Europe strategy
./infra/scripts/bootstrap.sh deploy-strategy europe
```

#### Cloudflare Configuration

```hcl
resource "cloudflare_load_balancer" "main" {
  fallback_pool_id = cloudflare_load_balancer_pool.india.id
  default_pool_ids = [cloudflare_load_balancer_pool.india.id]

  # South Asia → India
  region_pools {
    region   = "SAS"
    pool_ids = [cloudflare_load_balancer_pool.india.id]
  }

  # Southeast Asia → India
  region_pools {
    region   = "SEAS"
    pool_ids = [cloudflare_load_balancer_pool.india.id]
  }

  # Western Europe → Europe
  region_pools {
    region   = "WEU"
    pool_ids = [cloudflare_load_balancer_pool.europe.id]
  }

  # Eastern Europe → Europe
  region_pools {
    region   = "EEU"
    pool_ids = [cloudflare_load_balancer_pool.europe.id]
  }

  # Middle East → Europe
  region_pools {
    region   = "ME"
    pool_ids = [cloudflare_load_balancer_pool.europe.id]
  }

  # Africa → Europe
  region_pools {
    region   = "NSAF"
    pool_ids = [cloudflare_load_balancer_pool.europe.id]
  }
}
```

---

### Scaling Path

```text
India ($250/month)
    │
    └── Add Europe → $320/month (India + Europe)
            │
            └── Add Americas → $490/month

Europe ($150/month)
    │
    └── Add India → $320/month (India + Europe)
            │
            └── Add Americas → $490/month
```

### Scaling Down

To reduce costs:

1. Remove Americas (route to nearest region)
2. Remove India (route Asia to Europe, higher latency)
3. Single Europe region = lowest cost ($150/month)

---

## Architecture Overview

### Strategy: Europe (Single Region)

When using the Europe strategy, all traffic routes to a single Hetzner cluster:

```
User (anywhere) → Cloudflare Edge → Europe Pool (Hetzner) → uno-app
```

### Strategy: Full Global (Multi-Region)

| Region | Provider | Location | Database Role | HPA Max |
|--------|----------|----------|---------------|---------|
| Americas | GCP | us-central1 | Primary (writes) | 6 |
| Europe | Hetzner | nbg1 (Nuremberg) | Read Replica | 4 |
| APAC | DigitalOcean | sgp1 (Singapore) | Read Replica | 4 |

*Africa has no dedicated data center. Traffic routes to Europe via Cloudflare geo-routing.*

### Traffic Flow

```
User Request
    │
    ▼
Cloudflare Edge (CDN/WAF)
    │
    ▼
Geo-routing Load Balancer
    │
    ├──► Americas Pool (GCP)
    ├──► Europe Pool (Hetzner)
    └──► APAC Pool (DigitalOcean)
           │
           ▼
    Cloudflare Tunnel (cloudflared)
           │
           ▼
    K3s Cluster → uno-app pods
           │
           ▼
    PostgreSQL (Primary or Replica)
```

### Key Components

| Component | Purpose | Namespace |
|-----------|---------|-----------|
| uno-app | Main application | uno-app |
| cloudflared | Zero Trust tunnel | cloudflared |
| grafana-agent | Metrics & logs collection | monitoring |
| PostgreSQL | Database | External (Cloud SQL / self-managed) |

### Network Policies

- Default deny all in uno-app namespace
- Ingress allowed from cloudflared and traefik on port 3000
- Egress allowed to DNS (53), PostgreSQL (5432), external HTTPS (443)

---

## Helm Chart Reference

### Chart Location

```
infra/kubernetes/charts/uno-app/
├── Chart.yaml                    # Chart metadata + PostgreSQL dependency
├── charts/                       # Downloaded dependencies (postgresql)
├── values.yaml                   # Base configuration
├── values-strategy-local.yaml    # Local development (minikube/k3d/kind)
├── values-strategy-europe.yaml   # Europe strategy (MVP)
├── values-strategy-india.yaml    # India strategy (South Asia)
├── values-strategy-americas.yaml # Americas strategy
├── values-strategy-afroasia.yaml # AFRO-ASIA strategy
├── values-database-ha.yaml       # PostgreSQL HA overlay (production)
├── values-americas.yaml          # Full global: Americas
├── values-europe.yaml            # Full global: Europe
├── values-apac.yaml              # Full global: APAC
└── templates/
    ├── _helpers.tpl              # Template helpers
    ├── deployment.yaml           # Main application
    ├── service.yaml              # ClusterIP service
    ├── hpa.yaml                  # Horizontal Pod Autoscaler
    ├── pdb.yaml                  # Pod Disruption Budget
    ├── networkpolicy.yaml        # Network policies
    ├── postgresql-secret.yaml    # PostgreSQL credentials (placeholder)
    ├── cloudflared/              # Tunnel components
    └── monitoring/               # Grafana Agent DaemonSet
```

### Chart Dependencies

The chart includes Bitnami PostgreSQL as a sub-chart:

```yaml
# Chart.yaml
dependencies:
  - name: postgresql
    alias: db
    version: "16.4.1"
    repository: "https://charts.bitnami.com/bitnami"
    condition: database.internal.enabled
```

Run `helm dependency update` after cloning to download the PostgreSQL chart.

### Key Values

| Value | Default | Description |
|-------|---------|-------------|
| `app.replicaCount` | 2 | Initial replica count |
| `app.image.tag` | latest | Container image tag |
| `app.resources.limits.memory` | 512Mi | Memory limit |
| `app.resources.limits.cpu` | 500m | CPU limit |
| `hpa.minReplicas` | 2 | Minimum replicas |
| `hpa.maxReplicas` | 6 | Maximum replicas |
| `hpa.targetCPU` | 70 | CPU scaling threshold (%) |
| `hpa.targetMemory` | 80 | Memory scaling threshold (%) |
| `region` | americas | Deployment region |
| `provider` | gcp | Cloud provider |
| `databaseRole` | primary | Database role (primary/replica) |
| `cloudflared.enabled` | true | Enable Cloudflare tunnel |
| `monitoring.enabled` | true | Enable Grafana Agent |
| `database.internal.enabled` | false | Deploy PostgreSQL in-cluster |
| `database.external.enabled` | true | Use external managed database |

### Database Configuration

The chart supports three database modes:

#### Mode 1: External Managed Database (Default for Production)

Uses Cloud SQL, RDS, or other managed PostgreSQL service.

```yaml
# values.yaml (default)
database:
  internal:
    enabled: false
  external:
    enabled: true
```

Requires `db-credentials` secret with `url` key.

#### Mode 2: Internal Standalone (Local Development)

Deploys single-node PostgreSQL in the cluster.

```yaml
# values-strategy-local.yaml
database:
  internal:
    enabled: true
  external:
    enabled: false

db:
  fullnameOverride: uno-db
  architecture: standalone
  image:
    tag: latest
  auth:
    username: postgres
    password: postgres
    database: uno
```

Creates pods: `uno-db-0`
Creates services: `uno-db`, `uno-db-hl`

#### Mode 3: Internal HA (Self-Managed Production)

Deploys PostgreSQL with primary + read replicas.

```yaml
# values-database-ha.yaml (use with prod strategy)
database:
  internal:
    enabled: true
  external:
    enabled: false

db:
  fullnameOverride: uno-db
  architecture: replication
  auth:
    existingSecret: "postgresql-credentials"
  readReplicas:
    replicaCount: 2
```

Creates pods: `uno-db-0` (primary), `uno-db-read-0`, `uno-db-read-1`
Creates services: `uno-db`, `uno-db-hl`, `uno-db-read`, `uno-db-read-hl`

### Database Deployment Examples

```bash
# Local development (single-node PostgreSQL)
helm upgrade --install uno-app . \
  -f values.yaml \
  -f values-strategy-local.yaml \
  -n uno-app

# Production with external managed database (default)
helm upgrade --install uno-app . \
  -f values.yaml \
  -f values-strategy-europe.yaml \
  -n uno-app
# Requires: kubectl create secret generic db-credentials --from-literal=url="postgres://..."

# Production with self-managed HA PostgreSQL
helm upgrade --install uno-app . \
  -f values.yaml \
  -f values-strategy-europe.yaml \
  -f values-database-ha.yaml \
  -n uno-app
# Requires: kubectl create secret generic postgresql-credentials \
#   --from-literal=postgres-password=xxx \
#   --from-literal=password=xxx \
#   --from-literal=replication-password=xxx
```

### Region-Specific Overrides

**Americas (values-americas.yaml)**
```yaml
region: americas
provider: gcp
databaseRole: primary
hpa:
  maxReplicas: 6
```

**Europe (values-europe.yaml)**
```yaml
region: europe
provider: hetzner
databaseRole: replica
hpa:
  maxReplicas: 4
```

**APAC (values-apac.yaml)**
```yaml
region: apac
provider: digitalocean
databaseRole: replica
app:
  resources:
    limits:
      memory: 384Mi
hpa:
  maxReplicas: 4
```

**Africa (values-africa.yaml)**
```yaml
region: africa
provider: hetzner  # No dedicated DC, routes to Europe
databaseRole: replica
app:
  nodeSelector:
    region: europe  # Deploys to Europe nodes
hpa:
  maxReplicas: 4
```

### Required Secrets

Before deployment, these secrets must be created based on your database mode:

#### Always Required

| Secret | Namespace | Keys | Description |
|--------|-----------|------|-------------|
| admin-credentials | uno-app | `api_key`, `client_id`, `secret_key` | Admin API authentication |
| r2-credentials | uno-app | `access_key_id`, `secret_access_key`, `endpoint`, `bucket_name` | R2 storage |
| cloudflare-tunnel | cloudflared | `token` | Cloudflare tunnel token |
| grafana-credentials | monitoring | `prometheus_url`, `loki_url`, `user`, `api_key` | Grafana Cloud |

#### Database Mode: External (Production Default)

| Secret | Namespace | Keys | Description |
|--------|-----------|------|-------------|
| db-credentials | uno-app | `url`, `read_replica_url` | Managed database connection strings |

```bash
kubectl create secret generic db-credentials \
  --namespace uno-app \
  --from-literal=url="postgres://user:pass@host:5432/uno" \
  --from-literal=read_replica_url="postgres://user:pass@replica:5432/uno"
```

#### Database Mode: Internal Standalone (Local)

No database secrets required - credentials are set in `values-strategy-local.yaml`.

#### Database Mode: Internal HA (Self-Managed Production)

| Secret | Namespace | Keys | Description |
|--------|-----------|------|-------------|
| postgresql-credentials | uno-app | `postgres-password`, `password`, `replication-password` | PostgreSQL auth |

```bash
kubectl create secret generic postgresql-credentials \
  --namespace uno-app \
  --from-literal=postgres-password="admin-password" \
  --from-literal=password="app-password" \
  --from-literal=replication-password="replication-password"
```

---

## Deployment Procedures

### Strategy Deployment (Recommended)

Use strategy deployment for cost-optimized single-region rollouts:

```bash
# Deploy Europe strategy (MVP - $150/month)
./infra/scripts/bootstrap.sh deploy-strategy europe

# Deploy India strategy ($250/month) - India, Pakistan, Sri Lanka, Nepal, Philippines
./infra/scripts/bootstrap.sh deploy-strategy india

# Deploy AFRO-ASIA strategy ($230/month) - Singapore-based
./infra/scripts/bootstrap.sh deploy-strategy afroasia

# Deploy Americas strategy ($294/month)
./infra/scripts/bootstrap.sh deploy-strategy americas

# Verify deployment
./infra/scripts/bootstrap.sh verify

# Show help
./infra/scripts/bootstrap.sh help
```

### Full Global Deployment

For multi-region deployment with read replicas (production scale):

```bash
# Deploy to Americas (primary database)
./infra/scripts/bootstrap.sh deploy americas

# Deploy to Europe (read replica)
./infra/scripts/bootstrap.sh deploy europe

# Deploy to APAC (read replica)
./infra/scripts/bootstrap.sh deploy apac
```

### Manual Helm Deployment

```bash
# Strategy deployment (single region)
helm upgrade --install uno-app ./infra/kubernetes/charts/uno-app \
  -f ./infra/kubernetes/charts/uno-app/values.yaml \
  -f ./infra/kubernetes/charts/uno-app/values-strategy-europe.yaml \
  --namespace uno-app \
  --create-namespace \
  --wait \
  --timeout 10m

# Check rollout status
kubectl rollout status deployment/uno-app -n uno-app

# Verify all components
kubectl get pods -n uno-app
kubectl get pods -n cloudflared
kubectl get pods -n monitoring
```

### Dry Run / Preview Changes

```bash
# Template without applying
helm template uno-app ./infra/kubernetes/charts/uno-app \
  -f ./infra/kubernetes/charts/uno-app/values.yaml \
  -f ./infra/kubernetes/charts/uno-app/values-americas.yaml

# Diff changes (requires helm-diff plugin)
helm diff upgrade uno-app ./infra/kubernetes/charts/uno-app \
  -f ./infra/kubernetes/charts/uno-app/values.yaml \
  -f ./infra/kubernetes/charts/uno-app/values-americas.yaml \
  -n uno-app
```

### Rollback Procedure

```bash
# View release history
helm history uno-app -n uno-app

# Rollback to previous version
helm rollback uno-app -n uno-app

# Rollback to specific revision
helm rollback uno-app 3 -n uno-app

# Or use bootstrap script
./infra/scripts/bootstrap.sh rollback 3
```

### Uninstall

```bash
# Using bootstrap script
./infra/scripts/bootstrap.sh uninstall

# Manual uninstall
helm uninstall uno-app -n uno-app
kubectl delete namespace uno-app monitoring cloudflared
```

---

## Monitoring & Alerting

### Dashboards

Access Grafana Cloud dashboards:
- **Application Health**: Request rate, error rate, latency percentiles
- **Infrastructure**: CPU, memory, disk usage per node
- **Database**: Query latency, connections, replication lag
- **Kubernetes**: Pod status, HPA metrics, resource utilization

### Key Metrics

| Metric | Warning | Critical | Action |
|--------|---------|----------|--------|
| Error Rate | > 0.5% | > 1% | Check logs, recent deployments |
| P95 Latency | > 1s | > 2s | Check DB, external calls |
| CPU Usage | > 70% | > 90% | Scale up, optimize code |
| Memory Usage | > 80% | > 90% | Check leaks, increase limits |
| Replication Lag | > 30s | > 60s | Check network, disk I/O |
| Pod Restarts | > 1/hour | > 3/hour | Check OOM, crash loops |

### Alert Rules (Configured in Helm Chart)

| Alert | Condition | Duration | Severity |
|-------|-----------|----------|----------|
| HighErrorRate | > 1% errors | 5m | critical |
| HighLatency | P95 > 2s | 5m | warning |
| PodRestarts | > 3 restarts | 1h | warning |
| HighCPUUsage | > 80% | 10m | warning |
| HighMemoryUsage | > 85% | 10m | warning |
| PodNotReady | Not ready | 5m | critical |
| DeploymentReplicasMismatch | Mismatch | 10m | warning |

---

## Incident Response

### Severity Levels

| Level | Response Time | Description | Example |
|-------|--------------|-------------|---------|
| P1 | 15 min | Complete outage | All regions down |
| P2 | 1 hour | Degraded (>5% affected) | One region down |
| P3 | 4 hours | Minor (<5% affected) | Elevated latency |
| P4 | 24 hours | Non-urgent | Dashboard issue |

### P1 Incident Checklist

- [ ] Acknowledge alert in PagerDuty
- [ ] Join incident channel
- [ ] Check Cloudflare status page
- [ ] Check regional cluster health
- [ ] Check database connectivity
- [ ] Identify root cause
- [ ] Implement fix or failover
- [ ] Verify recovery
- [ ] Communicate resolution
- [ ] Schedule postmortem

### Region Failure Response

1. **Automatic**: Cloudflare routes traffic to healthy regions
2. **Verify**: Check traffic is being served elsewhere
3. **Investigate**: Check cluster, nodes, pods, network
4. **Fix**: Apply fix or rebuild region
5. **Recover**: Cloudflare resumes traffic when healthy

---

## Troubleshooting

### Pod Not Starting

```bash
# Check pod status
kubectl get pods -n uno-app

# Check pod events
kubectl describe pod <pod-name> -n uno-app

# Check container logs
kubectl logs <pod-name> -n uno-app

# Check previous container logs (crash loop)
kubectl logs <pod-name> -n uno-app --previous

# Check resource constraints
kubectl top pods -n uno-app
```

**Common Causes:**
- Image pull failure → Check image name/tag, registry credentials
- OOMKilled → Increase memory limits in values.yaml
- CrashLoopBackOff → Check application logs, environment variables
- Pending → Check node resources, node selector, tolerations

### Application Errors

```bash
# Stream logs from all pods
kubectl logs -n uno-app -l app=uno-app -f

# Search logs for errors
kubectl logs -n uno-app -l app=uno-app | grep -i error

# Check application metrics
kubectl port-forward -n uno-app svc/uno-app 3000:3000
curl http://localhost:3000/metrics
```

### Database Connection Issues

```bash
# Test connectivity from pod
kubectl exec -it -n uno-app <pod> -- /bin/sh
nc -zv <db-host> 5432

# Check secret is mounted correctly
kubectl exec -it -n uno-app <pod> -- env | grep DATABASE

# Check network policy allows egress
kubectl get networkpolicy -n uno-app -o yaml
```

### Cloudflared Tunnel Issues

```bash
# Check tunnel pod status
kubectl get pods -n cloudflared

# Check tunnel logs
kubectl logs -n cloudflared -l app=cloudflared

# Verify tunnel token secret
kubectl get secret cloudflare-tunnel -n cloudflared

# Restart tunnel
kubectl rollout restart deployment/cloudflared -n cloudflared
```

### HPA Not Scaling

```bash
# Check HPA status
kubectl get hpa -n uno-app

# Describe HPA for events
kubectl describe hpa uno-app-hpa -n uno-app

# Check metrics server
kubectl top pods -n uno-app

# Check current resource usage
kubectl get pods -n uno-app -o jsonpath='{.items[*].spec.containers[*].resources}'
```

**Common Causes:**
- Metrics server not running
- Resource requests not set
- Already at max replicas
- Scaling cooldown period

### Network Policy Issues

```bash
# List network policies
kubectl get networkpolicy -n uno-app

# Test connectivity to external service
kubectl exec -it -n uno-app <pod> -- curl -v https://api.example.com

# Check DNS resolution
kubectl exec -it -n uno-app <pod> -- nslookup api.example.com

# Temporarily disable network policy (debugging only)
kubectl delete networkpolicy default-deny-all -n uno-app
```

### Helm Deployment Issues

```bash
# Check release status
helm status uno-app -n uno-app

# Get release history
helm history uno-app -n uno-app

# Get deployed values
helm get values uno-app -n uno-app

# Get all deployed manifests
helm get manifest uno-app -n uno-app

# Debug template rendering
helm template uno-app ./infra/kubernetes/charts/uno-app \
  -f ./infra/kubernetes/charts/uno-app/values-americas.yaml \
  --debug
```

### Grafana Agent Issues

```bash
# Check agent status
kubectl get pods -n monitoring

# Check agent logs
kubectl logs -n monitoring -l app=grafana-agent

# Verify credentials secret
kubectl get secret grafana-credentials -n monitoring

# Check agent config
kubectl get configmap grafana-agent-config -n monitoring -o yaml
```

---

## Database Operations

### Internal PostgreSQL (In-Cluster)

When using `database.internal.enabled=true`, PostgreSQL runs as a StatefulSet in the cluster.

#### Access Database

```bash
# Connect to PostgreSQL CLI
kubectl exec -it -n uno-app uno-db-0 -- bash -c 'PGPASSWORD="$POSTGRES_PASSWORD" psql -U postgres -d uno'

# Run a single query
kubectl exec -n uno-app uno-db-0 -- bash -c 'PGPASSWORD="$POSTGRES_PASSWORD" psql -U postgres -d uno -c "SELECT 1;"'

# Check pod status
kubectl get pods -n uno-app -l app.kubernetes.io/name=db

# Check PVC (persistent storage)
kubectl get pvc -n uno-app -l app.kubernetes.io/name=db
```

#### HA Mode Operations

```bash
# Check primary
kubectl exec -n uno-app uno-db-0 -- bash -c 'PGPASSWORD="$POSTGRES_PASSWORD" psql -U postgres -c "SELECT pg_is_in_recovery();"'
# Expected: f (false = primary)

# Check replicas
kubectl exec -n uno-app uno-db-read-0 -- bash -c 'PGPASSWORD="$POSTGRES_PASSWORD" psql -U postgres -c "SELECT pg_is_in_recovery();"'
# Expected: t (true = replica)

# Check replication status
kubectl exec -n uno-app uno-db-0 -- bash -c 'PGPASSWORD="$POSTGRES_PASSWORD" psql -U postgres -c "SELECT * FROM pg_stat_replication;"'

# Check replication lag (on replica)
kubectl exec -n uno-app uno-db-read-0 -- bash -c 'PGPASSWORD="$POSTGRES_PASSWORD" psql -U postgres -c "SELECT EXTRACT(EPOCH FROM (now() - pg_last_xact_replay_timestamp()));"'
```

#### Scaling Replicas

```bash
# Scale read replicas (HA mode only)
helm upgrade uno-app . \
  -f values.yaml \
  -f values-strategy-europe.yaml \
  -f values-database-ha.yaml \
  --set db.readReplicas.replicaCount=3 \
  -n uno-app
```

### External PostgreSQL (Managed)

When using `database.external.enabled=true`, connect to Cloud SQL, RDS, etc.

### Health Checks

```bash
# Check primary status
psql $PRIMARY_URL -c "SELECT pg_is_in_recovery();"
# Expected: f (false = primary)

# Check replica status
psql $REPLICA_URL -c "SELECT pg_is_in_recovery();"
# Expected: t (true = replica)

# Check replication lag (seconds)
psql $REPLICA_URL -c "SELECT EXTRACT(EPOCH FROM (now() - pg_last_xact_replay_timestamp()));"

# Check active connections
psql $PRIMARY_URL -c "SELECT count(*) FROM pg_stat_activity WHERE state = 'active';"
```

### Backup Procedures

```bash
# Run manual backup
./infra/scripts/backup.sh backup

# List backups in R2
./infra/scripts/backup.sh list

# Verify backup integrity
./infra/scripts/backup.sh verify /path/to/backup.sql.gz
```

### Restore Procedures

```bash
# Restore from backup (DESTRUCTIVE)
./infra/scripts/backup.sh restore /path/to/backup.sql.gz
```

### Failover Procedures

**WARNING: Automatic failover is NOT configured. Manual intervention required.**

```bash
# Check cluster health
./infra/scripts/failover.sh check

# Initiate failover (promotes replica)
./infra/scripts/failover.sh failover
```

**Post-Failover Steps:**
1. Update DATABASE_URL in secrets to point to new primary
2. Rebuild old primary as new replica
3. Update Terraform state
4. Document incident

---

## Disaster Recovery

### Recovery Time Objectives

| Scenario | RTO | RPO |
|----------|-----|-----|
| Single pod failure | 30 seconds | 0 |
| Single region failure | 5 minutes | 0 |
| Database primary failure | 1 hour | 5 minutes |
| Complete disaster | 4 hours | 5 minutes |

### Complete Region Rebuild

```bash
# 1. Provision infrastructure
cd infra/terraform/environments/<region>
terraform init
terraform apply

# 2. Set environment variables for secrets
export DATABASE_URL="..."
export ADMIN_API_KEY="..."
export CLOUDFLARE_TUNNEL_TOKEN="..."
export GRAFANA_API_KEY="..."

# 3. Bootstrap and deploy
./infra/scripts/bootstrap.sh deploy <region>

# 4. Verify
kubectl get pods -A
curl https://<region>.uno-app.example.com/api/v1/health
```

### Complete Disaster Recovery

1. **Database**: Restore from latest R2 backup
2. **Infrastructure**: Run Terraform in all regions
3. **Cloudflare**: Verify DNS and tunnel configuration
4. **Deploy**: Run bootstrap.sh for each region
5. **Verify**: Health checks in all regions
6. **DNS**: Update if necessary

---

## Routine Maintenance

### Daily Tasks

- [ ] Review Grafana dashboards for anomalies
- [ ] Check database replication lag < 30s
- [ ] Verify backups completed successfully
- [ ] Review error rate trends

### Weekly Tasks

- [ ] Review cloud provider cost reports
- [ ] Check security advisories (Rust, K3s, dependencies)
- [ ] Review error logs for recurring patterns
- [ ] Verify Helm chart versions are current

### Monthly Tasks

- [ ] Test disaster recovery procedures
- [ ] Rotate credentials and API keys
- [ ] Update dependencies (Rust, K3s, Cloudflared)
- [ ] Review and optimize slow database queries
- [ ] Capacity planning review

### Quarterly Tasks

- [ ] Security audit and penetration testing
- [ ] Load testing and capacity validation
- [ ] Chaos engineering tests
- [ ] Documentation review and updates
- [ ] Review and update runbook

---

## Useful Commands

### Make Commands Reference

All operational commands are available via Make. Run `make help` for the full list.

#### Quick Reference

| Command | Description |
|---------|-------------|
| `make help` | Show all available commands |
| `make up` | Setup minikube + deploy locally |
| `make down` | Full cleanup |
| `make logs` | Follow application logs |
| `make ps` | List pods |
| `make status` | Show all resources + health |
| `make health` | Check app health endpoint |

#### Docker Commands

| Command | Description |
|---------|-------------|
| `make docker-build` | Build Docker image |
| `make docker-push` | Push to registry |
| `make docker-build-push` | Build and push |
| `make docker-build-minikube` | Build in minikube's Docker |

#### Helm Commands

| Command | Description |
|---------|-------------|
| `make helm-deps` | Update chart dependencies |
| `make helm-deploy ENV=local` | Deploy to local |
| `make helm-deploy ENV=europe` | Deploy Europe strategy |
| `make helm-deploy-ha ENV=europe` | Deploy with HA PostgreSQL |
| `make helm-status` | Show release status |
| `make helm-rollback` | Rollback to previous |
| `make helm-uninstall` | Uninstall release |

#### Kubernetes Commands

| Command | Description |
|---------|-------------|
| `make k8s-pods` | List pods |
| `make k8s-logs` | Show app logs |
| `make k8s-logs-follow` | Follow app logs |
| `make k8s-shell` | Shell into app pod |
| `make k8s-psql` | Connect to PostgreSQL |
| `make k8s-restart` | Restart app |
| `make k8s-port-forward` | Forward port 3000 |

#### Development Workflows

| Command | Description |
|---------|-------------|
| `make dev-setup` | Setup local environment |
| `make dev-deploy` | Deploy locally |
| `make dev-rebuild` | Rebuild + restart |
| `make dev-clean` | Full cleanup |

### Helm

```bash
# List releases
helm list -n uno-app

# Get release status
helm status uno-app -n uno-app

# Get current values
helm get values uno-app -n uno-app

# Get all values (including defaults)
helm get values uno-app -n uno-app --all

# Template chart locally
helm template uno-app ./infra/kubernetes/charts/uno-app \
  -f ./infra/kubernetes/charts/uno-app/values-americas.yaml

# Lint chart
helm lint ./infra/kubernetes/charts/uno-app

# Package chart
helm package ./infra/kubernetes/charts/uno-app
```

### Kubernetes

```bash
# Get all resources
kubectl get all -n uno-app

# Get pods with node info
kubectl get pods -n uno-app -o wide

# Exec into pod
kubectl exec -it -n uno-app <pod> -- /bin/sh

# Port forward for local debugging
kubectl port-forward -n uno-app svc/uno-app 3000:3000

# Stream logs from all pods
kubectl logs -n uno-app -l app=uno-app -f --tail=100

# Scale deployment manually
kubectl scale deployment/uno-app -n uno-app --replicas=4

# Force pod restart
kubectl rollout restart deployment/uno-app -n uno-app

# Check resource usage
kubectl top pods -n uno-app
kubectl top nodes
```

### Cloudflare

```bash
# Purge entire cache
curl -X POST "https://api.cloudflare.com/client/v4/zones/{zone_id}/purge_cache" \
  -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
  -H "Content-Type: application/json" \
  --data '{"purge_everything":true}'

# Purge specific URLs
curl -X POST "https://api.cloudflare.com/client/v4/zones/{zone_id}/purge_cache" \
  -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
  -H "Content-Type: application/json" \
  --data '{"files":["https://example.com/path"]}'
```

### Database

#### Internal PostgreSQL (uno-db)

```bash
# Connect to database
kubectl exec -it -n uno-app uno-db-0 -- bash -c 'PGPASSWORD="$POSTGRES_PASSWORD" psql -U postgres -d uno'

# Check database connection from app
kubectl exec -n uno-app deploy/uno-app -- curl -s http://localhost:3000/api/v1/health

# View database logs
kubectl logs -n uno-app uno-db-0

# Check storage usage
kubectl exec -n uno-app uno-db-0 -- df -h /bitnami/postgresql

# Restart PostgreSQL
kubectl delete pod -n uno-app uno-db-0
# StatefulSet will recreate the pod, data persists in PVC
```

#### External PostgreSQL

```bash
# Connect to primary
psql $DATABASE_URL

# Active connections
SELECT count(*) FROM pg_stat_activity WHERE state = 'active';

# Connection by state
SELECT state, count(*) FROM pg_stat_activity GROUP BY state;

# Slow queries (requires pg_stat_statements)
SELECT query, calls, mean_exec_time, total_exec_time
FROM pg_stat_statements
ORDER BY mean_exec_time DESC
LIMIT 10;

# Table sizes
SELECT relname, pg_size_pretty(pg_total_relation_size(relid))
FROM pg_catalog.pg_statio_user_tables
ORDER BY pg_total_relation_size(relid) DESC;

# Kill long-running query
SELECT pg_terminate_backend(pid)
FROM pg_stat_activity
WHERE duration > interval '5 minutes';
```

---

## Contacts

| Role | Contact |
|------|---------|
| Primary On-Call | See PagerDuty |
| Secondary On-Call | See PagerDuty |
| Infrastructure Lead | TBD |
| Security Team | TBD |
| Database Admin | TBD |

---

## Change Log

| Date | Change | Author |
|------|--------|--------|
| 2026-07-17 | Added Makefile with all operational commands | Claude |
| 2026-07-17 | Added PostgreSQL Helm sub-chart with dev/prod modes, renamed to uno-db | Claude |
| 2026-07-17 | Restructured for strategy-first deployment, added Quick Start MVP section | Claude |
| 2026-07-17 | Migrated from Kustomize to Helm, added troubleshooting section | Claude |
| 2024-XX-XX | Initial runbook | Claude |
