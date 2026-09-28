#!/bin/bash
# Database Backup Script
# Creates and uploads backups to Cloudflare R2

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
DB_HOST="${DB_HOST:-localhost}"
DB_USER="${DB_USER:-uno_app}"
DB_PASSWORD="${DB_PASSWORD:-}"
DB_NAME="${DB_NAME:-uno_app}"
BACKUP_DIR="${BACKUP_DIR:-/tmp/backups}"
RETENTION_DAYS="${RETENTION_DAYS:-30}"

# R2 Configuration
R2_ENDPOINT="${R2_ENDPOINT:-}"
R2_BUCKET="${R2_BUCKET:-uno-app-backups}"
R2_ACCESS_KEY="${R2_ACCESS_KEY:-}"
R2_SECRET_KEY="${R2_SECRET_KEY:-}"

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Generate backup filename
generate_filename() {
    local type=$1
    local timestamp=$(date +%Y%m%d_%H%M%S)
    echo "uno_app_${type}_${timestamp}"
}

# Create full backup
create_full_backup() {
    log_info "Creating full database backup..."

    local filename=$(generate_filename "full")
    local backup_file="$BACKUP_DIR/${filename}.sql.gz"

    PGPASSWORD="$DB_PASSWORD" pg_dump \
        -h "$DB_HOST" \
        -U "$DB_USER" \
        -d "$DB_NAME" \
        --format=custom \
        --compress=9 \
        --file="$BACKUP_DIR/${filename}.dump"

    # Also create SQL dump for portability
    PGPASSWORD="$DB_PASSWORD" pg_dump \
        -h "$DB_HOST" \
        -U "$DB_USER" \
        -d "$DB_NAME" \
        --format=plain | gzip > "$backup_file"

    log_info "Backup created: $backup_file"
    echo "$backup_file"
}

# Upload to R2
upload_to_r2() {
    local file=$1
    local filename=$(basename "$file")

    log_info "Uploading $filename to R2..."

    if [ -z "$R2_ENDPOINT" ] || [ -z "$R2_ACCESS_KEY" ]; then
        log_warn "R2 credentials not configured, skipping upload"
        return 0
    fi

    # Use AWS CLI with R2 endpoint
    AWS_ACCESS_KEY_ID="$R2_ACCESS_KEY" \
    AWS_SECRET_ACCESS_KEY="$R2_SECRET_KEY" \
    aws s3 cp "$file" "s3://$R2_BUCKET/database/$filename" \
        --endpoint-url "$R2_ENDPOINT"

    log_info "Upload complete"
}

# Clean old backups
cleanup_old_backups() {
    log_info "Cleaning up old backups..."

    # Local cleanup
    find "$BACKUP_DIR" -name "uno_app_*.sql.gz" -mtime +$RETENTION_DAYS -delete
    find "$BACKUP_DIR" -name "uno_app_*.dump" -mtime +$RETENTION_DAYS -delete

    # R2 cleanup (if configured)
    if [ -n "$R2_ENDPOINT" ] && [ -n "$R2_ACCESS_KEY" ]; then
        local cutoff_date=$(date -d "-$RETENTION_DAYS days" +%Y-%m-%d)

        AWS_ACCESS_KEY_ID="$R2_ACCESS_KEY" \
        AWS_SECRET_ACCESS_KEY="$R2_SECRET_KEY" \
        aws s3 ls "s3://$R2_BUCKET/database/" \
            --endpoint-url "$R2_ENDPOINT" | while read -r line; do
            local file_date=$(echo "$line" | awk '{print $1}')
            local file_name=$(echo "$line" | awk '{print $4}')

            if [[ "$file_date" < "$cutoff_date" ]]; then
                log_info "Deleting old backup: $file_name"
                AWS_ACCESS_KEY_ID="$R2_ACCESS_KEY" \
                AWS_SECRET_ACCESS_KEY="$R2_SECRET_KEY" \
                aws s3 rm "s3://$R2_BUCKET/database/$file_name" \
                    --endpoint-url "$R2_ENDPOINT"
            fi
        done
    fi

    log_info "Cleanup complete"
}

# Verify backup
verify_backup() {
    local file=$1
    log_info "Verifying backup integrity..."

    if [[ "$file" == *.gz ]]; then
        gzip -t "$file" || {
            log_error "Backup verification failed - file is corrupted"
            return 1
        }
    elif [[ "$file" == *.dump ]]; then
        pg_restore --list "$file" > /dev/null || {
            log_error "Backup verification failed - dump is invalid"
            return 1
        }
    fi

    log_info "Backup verified successfully"
    return 0
}

# Restore from backup
restore_backup() {
    local file=$1
    log_warn "Restoring database from $file..."

    read -p "This will overwrite the current database. Continue? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_info "Restore cancelled"
        return 0
    fi

    if [[ "$file" == *.dump ]]; then
        PGPASSWORD="$DB_PASSWORD" pg_restore \
            -h "$DB_HOST" \
            -U "$DB_USER" \
            -d "$DB_NAME" \
            --clean \
            --if-exists \
            "$file"
    elif [[ "$file" == *.gz ]]; then
        gunzip -c "$file" | PGPASSWORD="$DB_PASSWORD" psql \
            -h "$DB_HOST" \
            -U "$DB_USER" \
            -d "$DB_NAME"
    fi

    log_info "Restore complete"
}

# List available backups
list_backups() {
    log_info "Local backups:"
    ls -lh "$BACKUP_DIR"/uno_app_*.{sql.gz,dump} 2>/dev/null || echo "No local backups found"

    if [ -n "$R2_ENDPOINT" ] && [ -n "$R2_ACCESS_KEY" ]; then
        log_info "R2 backups:"
        AWS_ACCESS_KEY_ID="$R2_ACCESS_KEY" \
        AWS_SECRET_ACCESS_KEY="$R2_SECRET_KEY" \
        aws s3 ls "s3://$R2_BUCKET/database/" \
            --endpoint-url "$R2_ENDPOINT" \
            --human-readable
    fi
}

# Main function
main() {
    local action=${1:-backup}

    case $action in
        backup)
            local backup_file=$(create_full_backup)
            verify_backup "$backup_file"
            upload_to_r2 "$backup_file"
            cleanup_old_backups
            log_info "Backup process complete"
            ;;

        restore)
            local file=$2
            if [ -z "$file" ]; then
                log_error "Please specify a backup file to restore"
                exit 1
            fi
            restore_backup "$file"
            ;;

        list)
            list_backups
            ;;

        cleanup)
            cleanup_old_backups
            ;;

        verify)
            local file=$2
            if [ -z "$file" ]; then
                log_error "Please specify a backup file to verify"
                exit 1
            fi
            verify_backup "$file"
            ;;

        *)
            echo "Usage: $0 {backup|restore <file>|list|cleanup|verify <file>}"
            exit 1
            ;;
    esac
}

main "$@"
