# Infrastructure Research Findings

**Date**: 2026-07-17
**Scope**: `/infra` directory analysis
**Status**: Complete

---

## Overview

The `/infra` directory contains a **multi-cloud, multi-region deployment system** for uno-app providing:

- Global distribution across 3 geographic regions (Americas, Europe, APAC - **no Africa coverage**)
- Automated infrastructure provisioning via Terraform
- Kubernetes orchestration with K3s
- PostgreSQL primary/replica topology
- Cloudflare CDN, WAF, and Zero Trust networking
- Comprehensive observability via Grafana Cloud

---

## Directory Structure

```
infra/
├── terraform/
│   ├── environments/
│   │   ├── americas/           # GCP (Primary)
│   │   │   ├── main.tf
│   │   │   ├── variables.tf
│   │   │   ├── terraform.tfvars.example
│   │   │   └── scripts/
│   │   │       ├── k3s-master.sh
│   │   │       ├── k3s-worker.sh
│   │   │       └── postgres-replica.sh
│   │   ├── europe/             # Hetzner Cloud (Secondary)
│   │   │   ├── main.tf
│   │   │   ├── variables.tf
│   │   │   └── scripts/
│   │   └── apac/               # DigitalOcean (Secondary)
│   │       ├── main.tf
│   │       ├── variables.tf
│   │       └── scripts/
│   ├── modules/
│   │   ├── k3s-cluster/main.tf
│   │   ├── database/main.tf
│   │   ├── networking/main.tf
│   │   └── monitoring/main.tf
│   └── global/
│       ├── cloudflare.tf       # CDN, WAF, LB, DNS, Tunnel
│       └── r2.tf               # Object storage (backups/images)
├── kubernetes/
│   ├── base/
│   │   ├── namespace.yaml
│   │   ├── deployment.yaml
│   │   ├── service.yaml
│   │   ├── serviceaccount.yaml
│   │   ├── hpa.yaml
│   │   ├── pdb.yaml
│   │   ├── networkpolicy.yaml
│   │   ├── cloudflared.yaml
│   │   ├── secrets.yaml
│   │   ├── monitoring.yaml
│   │   └── kustomization.yaml
│   └── overlays/
│       ├── americas/kustomization.yaml
│       ├── europe/kustomization.yaml
│       └── apac/kustomization.yaml
├── scripts/
│   ├── bootstrap.sh
│   ├── backup.sh
│   └── failover.sh
├── ansible/
│   ├── playbooks/
│   └── inventory/
└── RUNBOOK.md
```

---

## Technology Stack

### Infrastructure as Code
| Tool | Version | Purpose |
|------|---------|---------|
| Terraform | ~1.5.0+ | Primary IaC |
| hashicorp/google | ~5.0 | GCP provider |
| hetznercloud/hcloud | ~1.45 | Hetzner provider |
| cloudflare/cloudflare | ~4.20 | Cloudflare provider |

### Container Orchestration
| Tool | Version | Purpose |
|------|---------|---------|
| K3s | v1.29.0+ | Lightweight Kubernetes |
| Helm | 3.x | Package management and templating |
| Docker | - | Container images (ghcr.io) |

### Cloud Providers
| Provider | Region | Role |
|----------|--------|------|
| GCP | us-central1 | Americas (Primary) |
| Hetzner Cloud | nbg1 | Europe (Secondary) |
| DigitalOcean | sgp1 | APAC (Secondary) |

### Database
| Component | Details |
|-----------|---------|
| Engine | PostgreSQL 16 |
| Primary | Cloud SQL (GCP us-central1) |
| Replicas | Self-managed in Hetzner/DO |
| HA Mode | REGIONAL (failover within region) |
| Backup | Daily, 30-day retention, PITR |

### CDN & Security
| Service | Purpose |
|---------|---------|
| Cloudflare CDN | Static asset caching |
| Cloudflare WAF | SQL injection, bot blocking |
| Cloudflare LB | Geo-based load balancing |
| Cloudflare Tunnel | Zero Trust connectivity |
| Cloudflare R2 | S3-compatible storage |

### Monitoring
| Component | Purpose |
|-----------|---------|
| Grafana Cloud | Metrics, logs, alerting |
| Grafana Agent | DaemonSet collector |
| Prometheus | Metrics scraping |
| Loki | Log aggregation |

---

## Regional Architecture

### Americas (Primary)
- **Provider**: GCP
- **Region**: us-central1 (Iowa)
- **Compute**: 1x e2-small master + 2x e2-small workers
- **Database**: Cloud SQL PostgreSQL (writable primary)
- **Network**: 10.0.0.0/16 VPC

### Europe (Secondary)
- **Provider**: Hetzner Cloud
- **Region**: nbg1 (Nuremberg, Germany)
- **Compute**: 1x cx11 master + 2x cx11 workers
- **Database**: Self-managed PostgreSQL read replica
- **Storage**: 50GB ext4 volume

### APAC (Secondary)
- **Provider**: DigitalOcean
- **Region**: sgp1 (Singapore)
- **Compute**: Similar to Europe
- **Database**: Self-managed PostgreSQL read replica

---

## Kubernetes Configuration

### Deployment Specs
| Setting | Value |
|---------|-------|
| Replicas | 2-6 (HPA controlled) |
| Image | ghcr.io/unetwork/uno-app:latest |
| Port | 3000 |
| CPU | 100m-500m |
| Memory | 256Mi-512Mi |
| Health endpoint | /api/v1/health |

### Horizontal Pod Autoscaler
| Setting | Value |
|---------|-------|
| Min replicas | 2 |
| Max replicas | 6 |
| Scale-up | 100% every 15s |
| Scale-down | 10% every 60s (300s stabilization) |
| CPU threshold | 70% |
| Memory threshold | 80% |

### Pod Security
- Non-root user (UID 1000)
- Read-only root filesystem
- All capabilities dropped
- No privilege escalation
- Network policies (deny-by-default)

### Network Policy Rules
| Direction | Allow |
|-----------|-------|
| Ingress | cloudflared, traefik on port 3000 |
| Egress | DNS (53), PostgreSQL (5432), External HTTPS (443) |

---

## Cloudflare Configuration

### Load Balancer
| Setting | Value |
|---------|-------|
| Steering | GEO (geographic) |
| Session affinity | Cookie-based, 30 min TTL |
| Fallback pool | Americas |
| Health check | GET /api/v1/health, 60s interval |

### Regional Mapping
| Cloudflare Region | Pool |
|-------------------|------|
| WNAM, ENAM, SAM | Americas |
| WEU, EEU, ME | Europe |
| SAS, SEAS, NEAS, OC | APAC |

### Coverage Gap: Africa
**Issue**: No dedicated data center in Africa. Traffic from NSAF (North & South Africa) is routed to Europe pool (Hetzner nbg1, Germany).

**Impact**: African users experience ~150-300ms additional latency compared to having a regional presence.

**Location in code**: `infra/terraform/global/cloudflare.tf:252-255`

**Current workaround**: `values-africa.yaml` added to Helm chart for logical separation, but physically deploys to Europe infrastructure.

**Potential solutions**:
1. Add African region (e.g., AWS Cape Town af-south-1, GCP Johannesburg)
2. Use Cloudflare Workers for edge compute closer to African users
3. Evaluate if user base justifies additional infrastructure cost

### Cache Rules
| Path | TTL |
|------|-----|
| /*.wasm | 1 year |
| /pkg/*.{css,js} | 1 year |
| /assets/* | 1 month |
| /api/* | Bypass |
| / (GET) | 5 min edge, 1 min browser |

### WAF Rules
1. Rate limit: API 100 req/min per IP
2. SQL injection blocking (UNION/SELECT, DROP/TABLE)
3. Admin endpoints require x-client-id header
4. Block scanner user agents (sqlmap, nikto, nmap)
5. OWASP Core Rules enabled

### Security Headers
- X-Content-Type-Options: nosniff
- X-Frame-Options: DENY
- X-XSS-Protection: 1; mode=block
- Referrer-Policy: strict-origin-when-cross-origin
- Permissions-Policy: No geolocation/microphone/camera

---

## Monitoring & Alerting

### Alert Rules
| Alert | Condition |
|-------|-----------|
| HighErrorRate | >1% for 5m |
| HighLatency | P95 >2s |
| PodRestarts | >3 in 1h |
| HighCPUUsage | >80% for 10m |
| HighMemoryUsage | >85% for 10m |
| PodNotReady | Pod unhealthy |
| DeploymentReplicasMismatch | Desired != actual |

### Metrics Collected
- uno-app pods (custom metrics)
- Kubernetes nodes
- kube-state-metrics
- Container metrics

### Logs Collected
- uno-app application logs
- cloudflared tunnel logs

---

## Deployment Flow

### Infrastructure Provisioning
```
terraform init
terraform plan -out=plan
terraform apply plan
  ├── Create VPC and subnets
  ├── Create firewall rules
  ├── Create compute instances (K3s)
  ├── Create Cloud SQL primary DB
  ├── Create storage buckets
  └── Create Cloudflare resources
```

### Cluster Bootstrap (scripts/bootstrap.sh)
```
1. Verify prerequisites (kubectl, helm, kustomize)
2. Configure kubectl context
3. Create namespaces (uno-app, monitoring, cloudflared)
4. Create secrets from environment variables
5. Deploy cloudflared (tunnel)
6. Deploy monitoring (Grafana Agent)
7. Deploy application (kustomize)
8. Verify rollout status
```

### Application Deployment
```
1. Build Docker image
2. Push to ghcr.io
3. kubectl apply -k overlays/{region}
4. Rolling update: maxSurge=1, maxUnavailable=0
5. Health probes validate readiness
6. HPA monitors and scales
```

---

## Security Analysis

### Strengths
- Firewall whitelisting (Cloudflare IPs only)
- Private database (no public IP)
- Pod security (non-root, read-only FS, dropped capabilities)
- Network policies (deny-by-default)
- WAF rules (SQL injection, scanner blocking)
- Rate limiting (100 req/min per IP)
- PITR enabled (30-day retention)
- Sensitive variables marked in Terraform

### Weaknesses
| Issue | Risk |
|-------|------|
| Manual database failover | RTO ~1 hour |
| Self-managed replicas | Patching burden |
| Single admin secret | Cross-region compromise |
| No mTLS between pods | Internal traffic unencrypted |
| Grafana Agent privileged | Root access on nodes |

---

## Issues & Recommendations

### Critical (High Severity)

#### 1. Manual Database Failover
**Problem**: Database failover requires manual intervention
**Impact**: RTO ~1 hour, potential data loss
**Recommendation**: Implement automatic failover with PgBouncer/Stolon or managed HA replicas
**Files**: `terraform/environments/americas/main.tf:290-402`

#### 2. No Terraform State Locking
**Problem**: GCS backend without state locking
**Impact**: Concurrent applies can corrupt state
**Recommendation**: Enable locking (GCS + DynamoDB or Terraform Cloud)
**Files**: All `main.tf` backend configurations

### Important (Medium Severity)

#### 3. No Africa Region Coverage
**Problem**: No data center in Africa; NSAF traffic routed to Europe (Germany)
**Impact**: 150-300ms additional latency for African users
**Recommendation**: Add African region (AWS Cape Town or GCP Johannesburg) if user base justifies it
**File**: `terraform/global/cloudflare.tf:252-255`

#### 4. Self-Managed Database Replicas
**Problem**: Hetzner/DO replicas require manual patching
**Impact**: Security vulnerabilities, maintenance burden
**Recommendation**: Migrate to managed database services per region

#### 5. No GitOps Pipeline
**Problem**: Manual terraform apply, no CI/CD
**Impact**: Human error, no audit trail
**Recommendation**: Implement ArgoCD or Flux

#### 6. Cloudflare Tunnel Single Point
**Problem**: If tunnel fails, no direct access
**Impact**: Complete application outage
**Recommendation**: Add direct DNS failover if tunnel unhealthy

#### 7. R2 Bucket Access Control
**Problem**: Bucket policy needs verification
**Impact**: Potential data exposure
**Recommendation**: Ensure private access with signed URLs only

### Minor (Low Severity)

#### 8. No Canary Deployments
**Recommendation**: Add traffic splitting with Flagger/Argo Rollouts

#### 9. Hardcoded Version Pins
**Recommendation**: Use semantic versioning (v1.29.x not v1.29.0+k3s1)

#### 10. No Pod Security Standards
**Recommendation**: Implement Kubernetes Pod Security Standards

#### 11. Secrets in Environment Variables
**Recommendation**: Use ExternalSecrets or Sealed Secrets for GitOps

---

## Cost Optimization Opportunities

| Opportunity | Potential Savings |
|-------------|-------------------|
| Spot/Preemptible instances for workers | 30-70% |
| Reserved compute (1-3 year) | 25-55% |
| Managed regional SQL replicas | 20-30% |
| Increased cache TTLs | Reduced bandwidth |
| Database right-sizing | Variable |

---

## Key File Locations

| Purpose | Path |
|---------|------|
| Americas Terraform | `infra/terraform/environments/americas/main.tf` |
| Europe Terraform | `infra/terraform/environments/europe/main.tf` |
| APAC Terraform | `infra/terraform/environments/apac/main.tf` |
| Cloudflare Config | `infra/terraform/global/cloudflare.tf` |
| R2 Storage | `infra/terraform/global/r2.tf` |
| K3s Module | `infra/terraform/modules/k3s-cluster/main.tf` |
| Helm Chart | `infra/kubernetes/charts/uno-app/Chart.yaml` |
| Values (Base) | `infra/kubernetes/charts/uno-app/values.yaml` |
| Values (Americas) | `infra/kubernetes/charts/uno-app/values-americas.yaml` |
| Values (Europe) | `infra/kubernetes/charts/uno-app/values-europe.yaml` |
| Values (APAC) | `infra/kubernetes/charts/uno-app/values-apac.yaml` |
| Deployment Template | `infra/kubernetes/charts/uno-app/templates/deployment.yaml` |
| Network Policy | `infra/kubernetes/charts/uno-app/templates/networkpolicy.yaml` |
| HPA | `infra/kubernetes/charts/uno-app/templates/hpa.yaml` |
| Monitoring | `infra/kubernetes/charts/uno-app/templates/monitoring/` |
| Cloudflare Tunnel | `infra/kubernetes/charts/uno-app/templates/cloudflared/` |
| Bootstrap Script | `infra/scripts/bootstrap.sh` |
| Backup Script | `infra/scripts/backup.sh` |
| Failover Script | `infra/scripts/failover.sh` |
| Runbook | `infra/RUNBOOK.md` |

---

## Open Questions

- [ ] What is the actual RTO/RPO requirement for the database?
- [ ] Is the current HPA scaling (2-6 replicas) sufficient for expected load?
- [ ] Are there plans to add more regions?
- [ ] What percentage of users are from Africa? Does it justify a regional presence?
- [ ] What is the strategy for PostgreSQL major version upgrades?
- [ ] Is there a disaster recovery plan for complete region failure?
- [ ] What is the secrets rotation policy?

---

## Next Steps

1. **Prioritize**: Address manual failover and state locking first
2. **Plan**: Create implementation plan for GitOps pipeline
3. **Test**: Validate backup/restore procedures
4. **Document**: Update RUNBOOK.md with failover automation plans
5. **Monitor**: Review alert thresholds after load testing
