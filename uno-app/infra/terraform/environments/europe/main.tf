# Europe Region - Hetzner Cloud
# Secondary region for uno-app deployment

terraform {
  required_version = ">= 1.5.0"

  required_providers {
    hcloud = {
      source  = "hetznercloud/hcloud"
      version = "~> 1.45"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.5"
    }
  }

  backend "gcs" {
    bucket = "uno-app-terraform-state"
    prefix = "europe"
  }
}

provider "hcloud" {
  token = var.hcloud_token
}

# Random suffix for unique resource names
resource "random_id" "suffix" {
  byte_length = 4
}

locals {
  name_prefix = "uno-app-${random_id.suffix.hex}"
  location    = "nbg1" # Nuremberg, Germany
  common_labels = {
    project     = "uno-app"
    environment = var.environment
    region      = "europe"
    managed_by  = "terraform"
  }
}

# =============================================================================
# SSH Key
# =============================================================================

resource "hcloud_ssh_key" "default" {
  name       = "${local.name_prefix}-ssh-key"
  public_key = var.ssh_public_key
}

# =============================================================================
# Network
# =============================================================================

resource "hcloud_network" "vpc" {
  name     = "${local.name_prefix}-vpc"
  ip_range = "10.1.0.0/16"
}

resource "hcloud_network_subnet" "k3s" {
  network_id   = hcloud_network.vpc.id
  type         = "cloud"
  network_zone = "eu-central"
  ip_range     = "10.1.0.0/24"
}

resource "hcloud_network_subnet" "database" {
  network_id   = hcloud_network.vpc.id
  type         = "cloud"
  network_zone = "eu-central"
  ip_range     = "10.1.1.0/24"
}

# =============================================================================
# Firewall
# =============================================================================

resource "hcloud_firewall" "k3s" {
  name = "${local.name_prefix}-k3s-firewall"

  # Allow SSH from specific IPs
  dynamic "rule" {
    for_each = var.ssh_allowed_ips
    content {
      direction  = "in"
      protocol   = "tcp"
      port       = "22"
      source_ips = [rule.value]
    }
  }

  # Allow HTTPS from Cloudflare IPs
  rule {
    direction = "in"
    protocol  = "tcp"
    port      = "443"
    source_ips = [
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
  }

  # Allow K3s API within VPC
  rule {
    direction  = "in"
    protocol   = "tcp"
    port       = "6443"
    source_ips = ["10.1.0.0/16"]
  }

  # Allow internal traffic
  rule {
    direction  = "in"
    protocol   = "tcp"
    port       = "any"
    source_ips = ["10.1.0.0/16"]
  }

  rule {
    direction  = "in"
    protocol   = "udp"
    port       = "any"
    source_ips = ["10.1.0.0/16"]
  }

  # Allow all outbound
  rule {
    direction       = "out"
    protocol        = "tcp"
    port            = "any"
    destination_ips = ["0.0.0.0/0"]
  }

  rule {
    direction       = "out"
    protocol        = "udp"
    port            = "any"
    destination_ips = ["0.0.0.0/0"]
  }

  rule {
    direction       = "out"
    protocol        = "icmp"
    destination_ips = ["0.0.0.0/0"]
  }
}

# =============================================================================
# K3s Master Node
# =============================================================================

resource "hcloud_server" "k3s_master" {
  name        = "${local.name_prefix}-k3s-master"
  server_type = var.master_server_type
  image       = "debian-12"
  location    = local.location
  ssh_keys    = [hcloud_ssh_key.default.id]

  network {
    network_id = hcloud_network.vpc.id
    ip         = "10.1.0.10"
  }

  firewall_ids = [hcloud_firewall.k3s.id]

  user_data = templatefile("${path.module}/scripts/k3s-master.sh", {
    k3s_token    = var.k3s_token
    k3s_version  = var.k3s_version
    cluster_name = local.name_prefix
  })

  labels = local.common_labels

  depends_on = [hcloud_network_subnet.k3s]

  lifecycle {
    ignore_changes = [user_data]
  }
}

# =============================================================================
# K3s Worker Nodes
# =============================================================================

resource "hcloud_server" "k3s_worker" {
  count       = var.worker_count
  name        = "${local.name_prefix}-k3s-worker-${count.index}"
  server_type = var.worker_server_type
  image       = "debian-12"
  location    = local.location
  ssh_keys    = [hcloud_ssh_key.default.id]

  network {
    network_id = hcloud_network.vpc.id
    ip         = "10.1.0.${20 + count.index}"
  }

  firewall_ids = [hcloud_firewall.k3s.id]

  user_data = templatefile("${path.module}/scripts/k3s-worker.sh", {
    k3s_token   = var.k3s_token
    k3s_version = var.k3s_version
    master_ip   = "10.1.0.10"
  })

  labels = local.common_labels

  depends_on = [hcloud_server.k3s_master]

  lifecycle {
    ignore_changes = [user_data]
  }
}

# =============================================================================
# PostgreSQL Read Replica (Self-managed)
# =============================================================================

resource "hcloud_server" "postgres_replica" {
  name        = "${local.name_prefix}-postgres-replica"
  server_type = var.db_server_type
  image       = "debian-12"
  location    = local.location
  ssh_keys    = [hcloud_ssh_key.default.id]

  network {
    network_id = hcloud_network.vpc.id
    ip         = "10.1.1.10"
  }

  firewall_ids = [hcloud_firewall.k3s.id]

  user_data = templatefile("${path.module}/scripts/postgres-replica.sh", {
    primary_host     = var.primary_db_host
    replication_user = var.db_replication_user
    replication_pass = var.db_replication_password
  })

  labels = local.common_labels

  depends_on = [hcloud_network_subnet.database]

  lifecycle {
    ignore_changes = [user_data]
  }
}

# =============================================================================
# Volumes for PostgreSQL Data
# =============================================================================

resource "hcloud_volume" "postgres_data" {
  name      = "${local.name_prefix}-postgres-data"
  size      = 50
  location  = local.location
  format    = "ext4"
  automount = true

  labels = local.common_labels
}

resource "hcloud_volume_attachment" "postgres_data" {
  volume_id = hcloud_volume.postgres_data.id
  server_id = hcloud_server.postgres_replica.id
  automount = true
}

# =============================================================================
# Outputs
# =============================================================================

output "vpc_id" {
  description = "VPC ID"
  value       = hcloud_network.vpc.id
}

output "k3s_master_ip" {
  description = "K3s master internal IP"
  value       = hcloud_server.k3s_master.network[0].ip
}

output "k3s_master_public_ip" {
  description = "K3s master public IP"
  value       = hcloud_server.k3s_master.ipv4_address
}

output "k3s_worker_ips" {
  description = "K3s worker internal IPs"
  value       = hcloud_server.k3s_worker[*].network[0].ip
}

output "postgres_replica_ip" {
  description = "PostgreSQL replica internal IP"
  value       = hcloud_server.postgres_replica.network[0].ip
}

output "database_url" {
  description = "PostgreSQL connection URL (read replica)"
  value       = "postgresql://uno_app:${var.db_password}@10.1.1.10:5432/uno_app"
  sensitive   = true
}
