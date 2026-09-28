#!/bin/bash
# Database Failover Script
# Promotes a read replica to primary in case of primary failure

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Configuration
PRIMARY_HOST="${PRIMARY_HOST:-}"
REPLICA_HOST="${REPLICA_HOST:-}"
DB_USER="${DB_USER:-uno_app}"
DB_PASSWORD="${DB_PASSWORD:-}"
DB_NAME="${DB_NAME:-uno_app}"
NOTIFY_WEBHOOK="${NOTIFY_WEBHOOK:-}"

# Check if primary is reachable
check_primary() {
    log_info "Checking primary database at $PRIMARY_HOST..."

    if pg_isready -h "$PRIMARY_HOST" -U "$DB_USER" -d "$DB_NAME" -t 5; then
        log_info "Primary is healthy"
        return 0
    else
        log_error "Primary is not responding"
        return 1
    fi
}

# Check replica health and lag
check_replica() {
    log_info "Checking replica at $REPLICA_HOST..."

    if ! pg_isready -h "$REPLICA_HOST" -U "$DB_USER" -d "$DB_NAME" -t 5; then
        log_error "Replica is not responding"
        return 1
    fi

    # Check replication lag
    LAG=$(PGPASSWORD="$DB_PASSWORD" psql -h "$REPLICA_HOST" -U "$DB_USER" -d "$DB_NAME" -tAc \
        "SELECT EXTRACT(EPOCH FROM (now() - pg_last_xact_replay_timestamp()))::integer;")

    log_info "Replica lag: ${LAG}s"

    if [ "$LAG" -gt 300 ]; then
        log_warn "Replica lag is high (>5 minutes). Data loss may occur."
    fi

    return 0
}

# Promote replica to primary
promote_replica() {
    log_info "Promoting replica to primary..."

    # Create trigger file to initiate promotion
    ssh "$REPLICA_HOST" "touch /tmp/postgresql.trigger" || {
        # If SSH fails, try pg_ctl
        PGPASSWORD="$DB_PASSWORD" psql -h "$REPLICA_HOST" -U postgres -c "SELECT pg_promote();"
    }

    # Wait for promotion to complete
    sleep 10

    # Verify promotion
    IS_STANDBY=$(PGPASSWORD="$DB_PASSWORD" psql -h "$REPLICA_HOST" -U "$DB_USER" -d "$DB_NAME" -tAc \
        "SELECT pg_is_in_recovery();")

    if [ "$IS_STANDBY" = "f" ]; then
        log_info "Replica successfully promoted to primary"
        return 0
    else
        log_error "Promotion failed - replica is still in recovery mode"
        return 1
    fi
}

# Update application configuration
update_app_config() {
    local new_primary=$1
    log_info "Updating application configuration..."

    # Update Kubernetes secret
    kubectl create secret generic db-credentials \
        --namespace uno-app \
        --from-literal=url="postgresql://$DB_USER:$DB_PASSWORD@$new_primary:5432/$DB_NAME" \
        --from-literal=read_replica_url="postgresql://$DB_USER:$DB_PASSWORD@$new_primary:5432/$DB_NAME" \
        --dry-run=client -o yaml | kubectl apply -f -

    # Restart application pods to pick up new config
    kubectl rollout restart deployment/uno-app -n uno-app

    # Wait for rollout
    kubectl rollout status deployment/uno-app -n uno-app --timeout=300s

    log_info "Application configuration updated"
}

# Send notification
notify() {
    local status=$1
    local message=$2

    if [ -n "$NOTIFY_WEBHOOK" ]; then
        curl -X POST "$NOTIFY_WEBHOOK" \
            -H "Content-Type: application/json" \
            -d "{\"status\": \"$status\", \"message\": \"$message\", \"timestamp\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"}" \
            || log_warn "Failed to send notification"
    fi

    log_info "Notification: [$status] $message"
}

# Main failover procedure
main() {
    local mode=${1:-check}

    case $mode in
        check)
            log_info "Running health checks..."
            check_primary || true
            check_replica || true
            ;;

        failover)
            log_warn "Starting failover procedure..."
            notify "STARTING" "Database failover initiated"

            # Verify primary is actually down
            if check_primary; then
                log_error "Primary is still reachable. Aborting failover."
                notify "ABORTED" "Primary is still reachable"
                exit 1
            fi

            # Check replica is healthy
            if ! check_replica; then
                log_error "Replica is not healthy. Cannot proceed with failover."
                notify "FAILED" "Replica is not healthy"
                exit 1
            fi

            # Promote replica
            if ! promote_replica; then
                log_error "Failed to promote replica"
                notify "FAILED" "Failed to promote replica"
                exit 1
            fi

            # Update application
            update_app_config "$REPLICA_HOST"

            log_info "Failover complete!"
            notify "SUCCESS" "Failover completed. New primary: $REPLICA_HOST"
            ;;

        rollback)
            log_warn "Rolling back to original primary..."
            # This would be implemented based on your specific requirements
            log_error "Rollback not implemented"
            exit 1
            ;;

        *)
            echo "Usage: $0 {check|failover|rollback}"
            exit 1
            ;;
    esac
}

main "$@"
