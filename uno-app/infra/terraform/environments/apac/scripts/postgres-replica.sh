#!/bin/bash
# PostgreSQL Read Replica Setup Script for DigitalOcean
set -e

# Variables from Terraform template
PRIMARY_HOST="${primary_host}"
REPLICATION_USER="${replication_user}"
REPLICATION_PASS="${replication_pass}"

PGVERSION="16"
PGDATA="/var/lib/postgresql/$PGVERSION/main"

echo "Starting PostgreSQL replica setup..."

# Install PostgreSQL
apt-get update
apt-get install -y postgresql-$PGVERSION postgresql-client-$PGVERSION

# Stop PostgreSQL to configure
systemctl stop postgresql

# Mount data volume if available (DO volumes appear as /dev/disk/by-id/scsi-0DO_*)
VOLUME_PATH=$(ls /dev/disk/by-id/scsi-0DO_* 2>/dev/null | head -n 1 || echo "")
if [ -n "$VOLUME_PATH" ]; then
  mkdir -p /mnt/postgres-data
  mount "$VOLUME_PATH" /mnt/postgres-data || true

  # Move data directory to volume
  if [ ! -d /mnt/postgres-data/main ]; then
    mv $PGDATA /mnt/postgres-data/main
  fi
  ln -sf /mnt/postgres-data/main $PGDATA
  chown -R postgres:postgres /mnt/postgres-data

  # Add to fstab for persistence
  echo "$VOLUME_PATH /mnt/postgres-data ext4 defaults,nofail 0 2" >> /etc/fstab
fi

# Remove existing data
rm -rf $PGDATA/*

# Configure pg_hba.conf for replication
cat >> /etc/postgresql/$PGVERSION/main/pg_hba.conf <<EOF
# Replication connections
host    replication     $REPLICATION_USER   0.0.0.0/0    md5
host    all             all                 10.0.0.0/8   md5
EOF

# Configure postgresql.conf for replica
cat >> /etc/postgresql/$PGVERSION/main/postgresql.conf <<EOF

# Replica configuration
hot_standby = on
max_standby_streaming_delay = 30s
wal_receiver_status_interval = 10s
hot_standby_feedback = on

# Performance tuning (adjusted for 2GB RAM)
shared_buffers = 512MB
effective_cache_size = 1536MB
work_mem = 8MB
maintenance_work_mem = 128MB
max_connections = 100

# Logging
log_destination = 'stderr'
logging_collector = on
log_directory = 'log'
log_filename = 'postgresql-%Y-%m-%d.log'
log_statement = 'ddl'
log_min_duration_statement = 1000
EOF

# Create .pgpass for passwordless auth
cat > /var/lib/postgresql/.pgpass <<EOF
$PRIMARY_HOST:5432:*:$REPLICATION_USER:$REPLICATION_PASS
EOF
chmod 600 /var/lib/postgresql/.pgpass
chown postgres:postgres /var/lib/postgresql/.pgpass

# Initialize replica from primary using pg_basebackup
su - postgres -c "PGPASSWORD='$REPLICATION_PASS' pg_basebackup -h $PRIMARY_HOST -D $PGDATA -U $REPLICATION_USER -P -v -R -X stream -C -S replica_apac"

# Fix permissions
chown -R postgres:postgres $PGDATA
chmod 700 $PGDATA

# Start PostgreSQL
systemctl start postgresql
systemctl enable postgresql

echo "PostgreSQL replica setup complete!"
echo "Replica is now streaming from $PRIMARY_HOST"
