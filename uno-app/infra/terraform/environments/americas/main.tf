# Americas Region - GCP
# Primary region for uno-app deployment

terraform {
  required_version = ">= 1.5.0"

  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.0"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.5"
    }
  }

  backend "gcs" {
    bucket = "uno-app-terraform-state"
    prefix = "americas"
  }
}

provider "google" {
  project = var.project_id
  region  = var.region
}

# Random suffix for unique resource names
resource "random_id" "suffix" {
  byte_length = 4
}

locals {
  name_prefix = "uno-app-${random_id.suffix.hex}"
  common_tags = {
    Project     = "uno-app"
    Environment = var.environment
    Region      = "americas"
    ManagedBy   = "terraform"
  }
}

# =============================================================================
# Networking
# =============================================================================

resource "google_compute_network" "vpc" {
  name                    = "${local.name_prefix}-vpc"
  auto_create_subnetworks = false
  description             = "VPC for uno-app Americas region"
}

resource "google_compute_subnetwork" "k3s" {
  name          = "${local.name_prefix}-k3s-subnet"
  ip_cidr_range = "10.0.0.0/24"
  region        = var.region
  network       = google_compute_network.vpc.id

  private_ip_google_access = true

  log_config {
    aggregation_interval = "INTERVAL_10_MIN"
    flow_sampling        = 0.5
    metadata             = "INCLUDE_ALL_METADATA"
  }
}

resource "google_compute_subnetwork" "database" {
  name          = "${local.name_prefix}-db-subnet"
  ip_cidr_range = "10.0.1.0/24"
  region        = var.region
  network       = google_compute_network.vpc.id

  private_ip_google_access = true
}

# Private Service Connection for Cloud SQL
resource "google_compute_global_address" "private_ip_range" {
  name          = "${local.name_prefix}-private-ip"
  purpose       = "VPC_PEERING"
  address_type  = "INTERNAL"
  prefix_length = 16
  network       = google_compute_network.vpc.id
}

resource "google_service_networking_connection" "private_vpc_connection" {
  network                 = google_compute_network.vpc.id
  service                 = "servicenetworking.googleapis.com"
  reserved_peering_ranges = [google_compute_global_address.private_ip_range.name]
}

# =============================================================================
# Firewall Rules
# =============================================================================

# Allow Cloudflare IPs only for HTTPS
resource "google_compute_firewall" "allow_cloudflare" {
  name    = "${local.name_prefix}-allow-cloudflare"
  network = google_compute_network.vpc.name

  allow {
    protocol = "tcp"
    ports    = ["443"]
  }

  source_ranges = [
    "173.245.48.0/20",
    "103.21.244.0/22",
    "103.22.200.0/22",
    "103.31.4.0/22",
    "141.101.64.0/18",
    "108.162.192.0/18",
    "190.93.240.0/20",
    "188.114.96.0/20",
    "197.234.240.0/22",
    "198.41.128.0/17",
    "162.158.0.0/15",
    "104.16.0.0/13",
    "104.24.0.0/14",
    "172.64.0.0/13",
    "131.0.72.0/22"
  ]

  target_tags = ["k3s-node"]
}

# Allow internal communication
resource "google_compute_firewall" "allow_internal" {
  name    = "${local.name_prefix}-allow-internal"
  network = google_compute_network.vpc.name

  allow {
    protocol = "tcp"
  }

  allow {
    protocol = "udp"
  }

  allow {
    protocol = "icmp"
  }

  source_ranges = ["10.0.0.0/8"]
  target_tags   = ["k3s-node"]
}

# Allow SSH from specific IPs (for management)
resource "google_compute_firewall" "allow_ssh" {
  name    = "${local.name_prefix}-allow-ssh"
  network = google_compute_network.vpc.name

  allow {
    protocol = "tcp"
    ports    = ["22"]
  }

  source_ranges = var.ssh_allowed_ips
  target_tags   = ["k3s-node"]
}

# Allow K3s API server
resource "google_compute_firewall" "allow_k3s_api" {
  name    = "${local.name_prefix}-allow-k3s-api"
  network = google_compute_network.vpc.name

  allow {
    protocol = "tcp"
    ports    = ["6443"]
  }

  source_ranges = ["10.0.0.0/8"]
  target_tags   = ["k3s-master"]
}

# =============================================================================
# K3s Cluster Nodes
# =============================================================================

# Service account for K3s nodes
resource "google_service_account" "k3s" {
  account_id   = "${local.name_prefix}-k3s"
  display_name = "K3s Node Service Account"
}

resource "google_project_iam_member" "k3s_logging" {
  project = var.project_id
  role    = "roles/logging.logWriter"
  member  = "serviceAccount:${google_service_account.k3s.email}"
}

resource "google_project_iam_member" "k3s_monitoring" {
  project = var.project_id
  role    = "roles/monitoring.metricWriter"
  member  = "serviceAccount:${google_service_account.k3s.email}"
}

# K3s Master Node
resource "google_compute_instance" "k3s_master" {
  name         = "${local.name_prefix}-k3s-master"
  machine_type = var.master_instance_type
  zone         = "${var.region}-a"

  boot_disk {
    initialize_params {
      image = "debian-cloud/debian-12"
      size  = 30
      type  = "pd-ssd"
    }
  }

  network_interface {
    subnetwork = google_compute_subnetwork.k3s.id

    access_config {
      # Ephemeral public IP for initial setup
    }
  }

  service_account {
    email  = google_service_account.k3s.email
    scopes = ["cloud-platform"]
  }

  metadata = {
    ssh-keys = "${var.ssh_user}:${var.ssh_public_key}"
  }

  metadata_startup_script = templatefile("${path.module}/scripts/k3s-master.sh", {
    k3s_token   = var.k3s_token
    k3s_version = var.k3s_version
    cluster_name = local.name_prefix
  })

  tags = ["k3s-node", "k3s-master"]

  labels = local.common_tags

  allow_stopping_for_update = true
}

# K3s Worker Nodes
resource "google_compute_instance" "k3s_worker" {
  count        = var.worker_count
  name         = "${local.name_prefix}-k3s-worker-${count.index}"
  machine_type = var.worker_instance_type
  zone         = "${var.region}-a"

  boot_disk {
    initialize_params {
      image = "debian-cloud/debian-12"
      size  = 30
      type  = "pd-ssd"
    }
  }

  network_interface {
    subnetwork = google_compute_subnetwork.k3s.id

    access_config {
      # Ephemeral public IP
    }
  }

  service_account {
    email  = google_service_account.k3s.email
    scopes = ["cloud-platform"]
  }

  metadata = {
    ssh-keys = "${var.ssh_user}:${var.ssh_public_key}"
  }

  metadata_startup_script = templatefile("${path.module}/scripts/k3s-worker.sh", {
    k3s_token   = var.k3s_token
    k3s_version = var.k3s_version
    master_ip   = google_compute_instance.k3s_master.network_interface[0].network_ip
  })

  tags = ["k3s-node", "k3s-worker"]

  labels = local.common_tags

  allow_stopping_for_update = true

  depends_on = [google_compute_instance.k3s_master]
}

# =============================================================================
# Cloud SQL (PostgreSQL)
# =============================================================================

resource "google_sql_database_instance" "primary" {
  name             = "${local.name_prefix}-postgres"
  database_version = "POSTGRES_16"
  region           = var.region

  settings {
    tier              = var.db_tier
    availability_type = "REGIONAL"
    disk_size         = 20
    disk_type         = "PD_SSD"
    disk_autoresize   = true

    ip_configuration {
      ipv4_enabled    = false
      private_network = google_compute_network.vpc.id
    }

    backup_configuration {
      enabled                        = true
      point_in_time_recovery_enabled = true
      start_time                     = "02:00"
      backup_retention_settings {
        retained_backups = 30
      }
    }

    database_flags {
      name  = "max_connections"
      value = "100"
    }

    database_flags {
      name  = "log_checkpoints"
      value = "on"
    }

    database_flags {
      name  = "log_connections"
      value = "on"
    }

    database_flags {
      name  = "log_disconnections"
      value = "on"
    }

    maintenance_window {
      day  = 7 # Sunday
      hour = 3 # 3 AM
    }

    insights_config {
      query_insights_enabled  = true
      record_application_tags = true
      record_client_address   = true
    }

    user_labels = local.common_tags
  }

  deletion_protection = var.environment == "production"

  depends_on = [google_service_networking_connection.private_vpc_connection]
}

resource "google_sql_database" "uno_app" {
  name     = "uno_app"
  instance = google_sql_database_instance.primary.name
}

resource "google_sql_user" "uno_app" {
  name     = "uno_app"
  instance = google_sql_database_instance.primary.name
  password = var.db_password
}

# Replication user for cross-region replicas
resource "google_sql_user" "replicator" {
  name     = "replicator"
  instance = google_sql_database_instance.primary.name
  password = var.db_replication_password
}

# =============================================================================
# Cloud Storage for Backups (Optional - R2 is primary)
# =============================================================================

resource "google_storage_bucket" "backups" {
  name          = "${local.name_prefix}-backups"
  location      = var.region
  storage_class = "STANDARD"

  versioning {
    enabled = true
  }

  lifecycle_rule {
    condition {
      age = 30
    }
    action {
      type = "Delete"
    }
  }

  uniform_bucket_level_access = true

  labels = local.common_tags
}

# =============================================================================
# Outputs
# =============================================================================

output "vpc_id" {
  description = "VPC ID"
  value       = google_compute_network.vpc.id
}

output "k3s_subnet_id" {
  description = "K3s subnet ID"
  value       = google_compute_subnetwork.k3s.id
}

output "k3s_master_ip" {
  description = "K3s master internal IP"
  value       = google_compute_instance.k3s_master.network_interface[0].network_ip
}

output "k3s_master_public_ip" {
  description = "K3s master public IP"
  value       = google_compute_instance.k3s_master.network_interface[0].access_config[0].nat_ip
}

output "k3s_worker_ips" {
  description = "K3s worker internal IPs"
  value       = google_compute_instance.k3s_worker[*].network_interface[0].network_ip
}

output "database_connection_name" {
  description = "Cloud SQL connection name"
  value       = google_sql_database_instance.primary.connection_name
}

output "database_private_ip" {
  description = "Cloud SQL private IP"
  value       = google_sql_database_instance.primary.private_ip_address
}

output "database_url" {
  description = "PostgreSQL connection URL"
  value       = "postgresql://uno_app:${var.db_password}@${google_sql_database_instance.primary.private_ip_address}:5432/uno_app"
  sensitive   = true
}

output "backup_bucket" {
  description = "Backup bucket name"
  value       = google_storage_bucket.backups.name
}
