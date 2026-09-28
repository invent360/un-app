#!/bin/bash
# Bootstrap script for uno-app infrastructure
# Run this after initial Terraform apply to configure the cluster

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INFRA_DIR="$(dirname "$SCRIPT_DIR")"
CHART_DIR="$INFRA_DIR/kubernetes/charts/uno-app"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."

    command -v kubectl >/dev/null 2>&1 || { log_error "kubectl is required but not installed."; exit 1; }
    command -v helm >/dev/null 2>&1 || { log_error "helm is required but not installed."; exit 1; }

    # Verify Helm chart exists
    if [ ! -f "$CHART_DIR/Chart.yaml" ]; then
        log_error "Helm chart not found at $CHART_DIR"
        exit 1
    fi

    log_info "All prerequisites met."
}

# Configure kubectl context
configure_kubectl() {
    local region=$1
    log_info "Configuring kubectl for $region..."

    case $region in
        local)
            # Local development - check for available local clusters
            if command -v minikube >/dev/null 2>&1 && minikube status >/dev/null 2>&1; then
                log_info "Using minikube context"
                kubectl config use-context minikube || true
            elif command -v k3d >/dev/null 2>&1; then
                log_info "Using k3d context (if available)"
                kubectl config use-context k3d-uno-local 2>/dev/null || true
            elif command -v kind >/dev/null 2>&1; then
                log_info "Using kind context (if available)"
                kubectl config use-context kind-kind 2>/dev/null || true
            else
                log_warn "No local cluster detected. Please start minikube, k3d, or kind first."
                log_warn "See: infra/RUNBOOK.md for local development setup"
            fi
            ;;
        americas)
            # GCP - use gcloud to get credentials
            if command -v gcloud >/dev/null 2>&1; then
                gcloud container clusters get-credentials uno-app-k3s --region us-central1 || true
            fi
            ;;
        europe)
            # Hetzner - use kubeconfig from Terraform output
            log_warn "Please ensure KUBECONFIG is set for Europe cluster"
            ;;
        apac)
            # DigitalOcean - use doctl or kubeconfig
            if command -v doctl >/dev/null 2>&1; then
                doctl kubernetes cluster kubeconfig save uno-app-sgp1 || true
            fi
            ;;
        africa)
            # Africa routes to Europe (Hetzner) - no dedicated DC
            log_warn "Africa region uses Europe infrastructure (Hetzner)"
            log_warn "Please ensure KUBECONFIG is set for Europe cluster"
            ;;
        india)
            # AWS Mumbai - use aws cli or kubeconfig
            if command -v aws >/dev/null 2>&1; then
                aws eks update-kubeconfig --region ap-south-1 --name uno-app-mumbai || true
            else
                log_warn "Please ensure KUBECONFIG is set for India cluster (AWS Mumbai)"
            fi
            ;;
    esac
}

# Create secrets from environment variables (before Helm install)
create_secrets() {
    local region=$1
    log_info "Creating secrets for $region..."

    # Create namespaces with Helm labels (so Helm can adopt them)
    for ns in uno-app monitoring cloudflared; do
        kubectl create namespace "$ns" --dry-run=client -o yaml | \
            kubectl label --local -f - \
                app.kubernetes.io/managed-by=Helm \
                --dry-run=client -o yaml | \
            kubectl annotate --local -f - \
                meta.helm.sh/release-name=uno-app \
                meta.helm.sh/release-namespace=uno-app \
                --dry-run=client -o yaml | \
            kubectl apply -f -
    done

    # For local development, skip pre-creating secrets (Helm will create placeholders)
    # We'll update them after Helm install
    if [ "$region" = "local" ]; then
        log_info "Local development: secrets will be created by Helm, then updated"
        return
    fi

    # Database credentials
    if [ -n "$DATABASE_URL" ]; then
        kubectl create secret generic db-credentials \
            --namespace uno-app \
            --from-literal=url="$DATABASE_URL" \
            --from-literal=read_replica_url="${DATABASE_READ_REPLICA_URL:-$DATABASE_URL}" \
            --dry-run=client -o yaml | kubectl apply -f -
    else
        log_warn "DATABASE_URL not set, skipping db-credentials secret"
    fi

    # Admin credentials
    if [ -n "$ADMIN_API_KEY" ]; then
        kubectl create secret generic admin-credentials \
            --namespace uno-app \
            --from-literal=api_key="$ADMIN_API_KEY" \
            --from-literal=client_id="${ADMIN_CLIENT_ID:-uno-admin}" \
            --from-literal=secret_key="$ADMIN_SECRET_KEY" \
            --dry-run=client -o yaml | kubectl apply -f -
    else
        log_warn "ADMIN_API_KEY not set, skipping admin-credentials secret"
    fi

    # R2 credentials
    if [ -n "$R2_ACCESS_KEY_ID" ]; then
        kubectl create secret generic r2-credentials \
            --namespace uno-app \
            --from-literal=access_key_id="$R2_ACCESS_KEY_ID" \
            --from-literal=secret_access_key="$R2_SECRET_ACCESS_KEY" \
            --from-literal=endpoint="$R2_ENDPOINT" \
            --from-literal=bucket_name="$R2_BUCKET_NAME" \
            --dry-run=client -o yaml | kubectl apply -f -
    else
        log_warn "R2_ACCESS_KEY_ID not set, skipping r2-credentials secret"
    fi

    # Cloudflare tunnel
    if [ -n "$CLOUDFLARE_TUNNEL_TOKEN" ]; then
        kubectl create secret generic cloudflare-tunnel \
            --namespace cloudflared \
            --from-literal=token="$CLOUDFLARE_TUNNEL_TOKEN" \
            --dry-run=client -o yaml | kubectl apply -f -
    else
        log_warn "CLOUDFLARE_TUNNEL_TOKEN not set, skipping cloudflare-tunnel secret"
    fi

    # Grafana credentials
    if [ -n "$GRAFANA_API_KEY" ]; then
        kubectl create secret generic grafana-credentials \
            --namespace monitoring \
            --from-literal=prometheus_url="$GRAFANA_PROMETHEUS_URL" \
            --from-literal=loki_url="$GRAFANA_LOKI_URL" \
            --from-literal=user="$GRAFANA_USER" \
            --from-literal=api_key="$GRAFANA_API_KEY" \
            --dry-run=client -o yaml | kubectl apply -f -
    else
        log_warn "GRAFANA_API_KEY not set, skipping grafana-credentials secret"
    fi
}

# Deploy uno-app using Helm
deploy_app() {
    local region=$1
    log_info "Deploying uno-app to $region using Helm..."

    # Check if values file exists for region
    local values_file="$CHART_DIR/values-$region.yaml"
    if [ ! -f "$values_file" ]; then
        log_error "Values file not found: $values_file"
        exit 1
    fi

    # Helm upgrade --install (idempotent)
    helm upgrade --install uno-app "$CHART_DIR" \
        -f "$CHART_DIR/values.yaml" \
        -f "$values_file" \
        --namespace uno-app \
        --create-namespace \
        --wait \
        --timeout 10m

    log_info "Helm deployment complete."
}

# Deploy using a cost-optimized strategy
deploy_strategy() {
    local strategy=$1
    log_info "Deploying uno-app using strategy: $strategy"

    # Map strategy to region for kubectl configuration
    local target_region
    case $strategy in
        local)
            target_region="local"
            log_info "Strategy: Local - development/testing"
            log_info "Estimated cost: \$0/month"
            log_info "Requires: minikube, k3d, kind, or Docker Desktop Kubernetes"
            ;;
        europe)
            target_region="europe"
            log_info "Strategy: Europe - serves EU, Middle East, Africa"
            log_info "Estimated cost: ~\$150/month"
            ;;
        afroasia)
            target_region="apac"
            log_info "Strategy: AFRO-ASIA - serves Africa + Asia-Pacific"
            log_info "Estimated cost: ~\$230/month"
            ;;
        americas)
            target_region="americas"
            log_info "Strategy: Americas - serves North/South America"
            log_info "Estimated cost: ~\$294/month"
            ;;
        india)
            target_region="india"
            log_info "Strategy: India - serves South Asia + Southeast Asia"
            log_info "Estimated cost: ~\$250/month"
            ;;
        *)
            log_error "Unknown strategy: $strategy"
            log_error "Available strategies: local, europe, afroasia, india, americas"
            exit 1
            ;;
    esac

    # Check if strategy values file exists
    local values_file="$CHART_DIR/values-strategy-$strategy.yaml"
    if [ ! -f "$values_file" ]; then
        log_error "Strategy values file not found: $values_file"
        exit 1
    fi

    # Configure kubectl for target region
    configure_kubectl "$target_region"

    # Create secrets
    create_secrets "$target_region"

    # Deploy with strategy values
    log_info "Deploying with strategy values..."
    helm upgrade --install uno-app "$CHART_DIR" \
        -f "$CHART_DIR/values.yaml" \
        -f "$values_file" \
        --namespace uno-app \
        --create-namespace \
        --wait \
        --timeout 10m

    # For local development, update secrets with test values after Helm install
    if [ "$strategy" = "local" ]; then
        log_info "Updating secrets with test values for local development..."

        kubectl create secret generic db-credentials \
            --namespace uno-app \
            --from-literal=url="postgres://postgres:postgres@postgres-postgresql:5432/uno" \
            --from-literal=read_replica_url="postgres://postgres:postgres@postgres-postgresql:5432/uno" \
            --dry-run=client -o yaml | kubectl apply -f -

        kubectl create secret generic admin-credentials \
            --namespace uno-app \
            --from-literal=api_key="local-dev-api-key" \
            --from-literal=client_id="local-admin" \
            --from-literal=secret_key="local-dev-secret-key" \
            --dry-run=client -o yaml | kubectl apply -f -

        kubectl create secret generic r2-credentials \
            --namespace uno-app \
            --from-literal=access_key_id="local-access-key" \
            --from-literal=secret_access_key="local-secret-key" \
            --from-literal=endpoint="http://localhost:9000" \
            --from-literal=bucket_name="local-bucket" \
            --dry-run=client -o yaml | kubectl apply -f -

        log_info "Test secrets updated. Restarting deployment to pick up new values..."
        kubectl rollout restart deployment/uno-app -n uno-app 2>/dev/null || true
    fi

    log_info "Strategy deployment complete: $strategy"
    if [ "$strategy" != "local" ]; then
        log_warn "Remember to update Cloudflare geo-routing to match this strategy!"
        log_warn "See: infra/RUNBOOK.md for Cloudflare configuration"
    fi
}

# Verify deployment
verify_deployment() {
    log_info "Verifying deployment..."

    log_info "uno-app pods:"
    kubectl get pods -n uno-app

    log_info "cloudflared pods:"
    kubectl get pods -n cloudflared

    log_info "monitoring pods:"
    kubectl get pods -n monitoring

    log_info "HPA status:"
    kubectl get hpa -n uno-app
}

# Rollback function
rollback() {
    local revision=${1:-1}
    log_info "Rolling back to revision $revision..."
    helm rollback uno-app "$revision" --namespace uno-app
}

# Uninstall function
uninstall() {
    log_warn "Uninstalling uno-app..."
    helm uninstall uno-app --namespace uno-app
    kubectl delete namespace uno-app monitoring cloudflared --ignore-not-found
}

# Show help
show_help() {
    echo "Usage: $0 <command> [region|strategy]"
    echo ""
    echo "Commands:"
    echo "  deploy [region]           Deploy uno-app to specified region (default: americas)"
    echo "  deploy-strategy <name>    Deploy using cost-optimized strategy"
    echo "  verify                    Verify deployment status"
    echo "  rollback [rev]            Rollback to specified revision (default: 1)"
    echo "  uninstall                 Uninstall uno-app completely"
    echo "  help                      Show this help message"
    echo ""
    echo "Regions: americas, europe, apac, africa"
    echo ""
    echo "Strategies (cost-optimized):"
    echo "  local      \$0/month    - Local development (minikube/k3d/kind)"
    echo "  europe     ~\$150/month - Serves EU, Middle East, Africa (lowest cost)"
    echo "  afroasia   ~\$230/month - Serves Africa + Asia-Pacific (Singapore)"
    echo "  india      ~\$250/month - Serves South Asia + Southeast Asia (Mumbai)"
    echo "  americas   ~\$294/month - Serves North/South America"
    echo ""
    echo "See infra/RUNBOOK.md for detailed cost analysis and Cloudflare configuration"
    echo ""
    echo "Environment variables required for secrets:"
    echo "  DATABASE_URL                 PostgreSQL connection URL"
    echo "  DATABASE_READ_REPLICA_URL    Read replica URL (optional, for replica regions)"
    echo "  ADMIN_API_KEY                Admin API key"
    echo "  ADMIN_CLIENT_ID              Admin client ID"
    echo "  ADMIN_SECRET_KEY             Admin secret key"
    echo "  CLOUDFLARE_TUNNEL_TOKEN      Cloudflare tunnel token"
    echo "  GRAFANA_PROMETHEUS_URL       Grafana Cloud Prometheus URL"
    echo "  GRAFANA_LOKI_URL             Grafana Cloud Loki URL"
    echo "  GRAFANA_USER                 Grafana Cloud user ID"
    echo "  GRAFANA_API_KEY              Grafana Cloud API key"
    echo "  R2_ACCESS_KEY_ID             Cloudflare R2 access key"
    echo "  R2_SECRET_ACCESS_KEY         Cloudflare R2 secret key"
    echo "  R2_ENDPOINT                  Cloudflare R2 endpoint"
    echo "  R2_BUCKET_NAME               Cloudflare R2 bucket name"
}

# Main function
main() {
    local command=${1:-deploy}
    local region=${2:-americas}

    case $command in
        deploy)
            log_info "Bootstrapping uno-app infrastructure for $region"
            echo ""
            check_prerequisites
            configure_kubectl "$region"
            create_secrets "$region"
            deploy_app "$region"
            verify_deployment
            echo ""
            log_info "Bootstrap complete for $region!"
            ;;
        deploy-strategy)
            local strategy=$region  # Second arg is strategy name
            if [ -z "$strategy" ] || [ "$strategy" = "americas" ]; then
                # If no strategy provided, show help
                if [ -z "$2" ]; then
                    log_error "Strategy name required"
                    echo "Available strategies: europe, afroasia, india, americas"
                    exit 1
                fi
            fi
            echo ""
            check_prerequisites
            deploy_strategy "$strategy"
            verify_deployment
            echo ""
            log_info "Strategy deployment complete: $strategy"
            ;;
        verify)
            verify_deployment
            ;;
        rollback)
            rollback "$region"  # region is used as revision number here
            ;;
        uninstall)
            uninstall
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            log_error "Unknown command: $command"
            show_help
            exit 1
            ;;
    esac
}

# Run with arguments
main "$@"
