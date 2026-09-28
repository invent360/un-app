# Asia-Pacific Region - DigitalOcean
# Secondary region for uno-app deployment

terraform {
  required_version = ">= 1.5.0"

  required_providers {
    digitalocean = {
      source  = "digitalocean/digitalocean"
      version = "~> 2.34"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.5"
    }
  }

  backend "gcs" {
    bucket = "uno-app-terraform-state"
    prefix = "apac"
  }
}

provider "digitalocean" {
  token = var.do_token
}

# Random suffix for unique resource names
resource "random_id" "suffix" {
  byte_length = 4
}

locals {
  name_prefix = "uno-app-${random_id.suffix.hex}"
  region      = "sgp1" # Singapore
  common_tags = ["uno-app", var.environment, "apac", "terraform"]
}

# =============================================================================
# SSH Key
# =============================================================================

resource "digitalocean_ssh_key" "default" {
  name       = "${local.name_prefix}-ssh-key"
  public_key = var.ssh_public_key
}

# =============================================================================
# VPC
# =============================================================================

resource "digitalocean_vpc" "main" {
  name     = "${local.name_prefix}-vpc"
  region   = local.region
  ip_range = "10.2.0.0/16"
}

# =============================================================================
# Firewall
# =============================================================================

resource "digitalocean_firewall" "k3s" {
  name = "${local.name_prefix}-k3s-firewall"

  droplet_ids = concat(
    [digitalocean_droplet.k3s_master.id],
    digitalocean_droplet.k3s_worker[*].id
  )

  # SSH from specific IPs
  dynamic "inbound_rule" {
    for_each = var.ssh_allowed_ips
    content {
      protocol         = "tcp"
      port_range       = "22"
      source_addresses = [inbound_rule.value]
    }
  }

  # HTTPS from Cloudflare
  inbound_rule {
    protocol   = "tcp"
    port_range = "443"
    source_addresses = [
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

  # K3s API within VPC
  inbound_rule {
    protocol         = "tcp"
    port_range       = "6443"
    source_addresses = ["10.2.0.0/16"]
  }

  # Internal traffic
  inbound_rule {
    protocol         = "tcp"
    port_range       = "1-65535"
    source_addresses = ["10.2.0.0/16"]
  }

  inbound_rule {
    protocol         = "udp"
    port_range       = "1-65535"
    source_addresses = ["10.2.0.0/16"]
  }

  inbound_rule {
    protocol         = "icmp"
    source_addresses = ["10.2.0.0/16"]
  }

  # Outbound - all
  outbound_rule {
    protocol              = "tcp"
    port_range            = "1-65535"
    destination_addresses = ["0.0.0.0/0", "::/0"]
  }

  outbound_rule {
    protocol              = "udp"
    port_range            = "1-65535"
    destination_addresses = ["0.0.0.0/0", "::/0"]
  }

  outbound_rule {
    protocol              = "icmp"
    destination_addresses = ["0.0.0.0/0", "::/0"]
  }
}

# =============================================================================
# K3s Master Node
# =============================================================================

resource "digitalocean_droplet" "k3s_master" {
  name     = "${local.name_prefix}-k3s-master"
  size     = var.master_size
  image    = "debian-12-x64"
  region   = local.region
  vpc_uuid = digitalocean_vpc.main.id
  ssh_keys = [digitalocean_ssh_key.default.fingerprint]

  user_data = templatefile("${path.module}/scripts/k3s-master.sh", {
    k3s_token    = var.k3s_token
    k3s_version  = var.k3s_version
    cluster_name = local.name_prefix
  })

  tags = local.common_tags

  lifecycle {
    ignore_changes = [user_data]
  }
}

# =============================================================================
# K3s Worker Nodes
# =============================================================================

resource "digitalocean_droplet" "k3s_worker" {
  count    = var.worker_count
  name     = "${local.name_prefix}-k3s-worker-${count.index}"
  size     = var.worker_size
  image    = "debian-12-x64"
  region   = local.region
  vpc_uuid = digitalocean_vpc.main.id
  ssh_keys = [digitalocean_ssh_key.default.fingerprint]

  user_data = templatefile("${path.module}/scripts/k3s-worker.sh", {
    k3s_token   = var.k3s_token
    k3s_version = var.k3s_version
    master_ip   = digitalocean_droplet.k3s_master.ipv4_address_private
  })

  tags = local.common_tags

  depends_on = [digitalocean_droplet.k3s_master]

  lifecycle {
    ignore_changes = [user_data]
  }
}

# =============================================================================
# PostgreSQL Read Replica
# =============================================================================

resource "digitalocean_droplet" "postgres_replica" {
  name     = "${local.name_prefix}-postgres-replica"
  size     = var.db_size
  image    = "debian-12-x64"
  region   = local.region
  vpc_uuid = digitalocean_vpc.main.id
  ssh_keys = [digitalocean_ssh_key.default.fingerprint]

  user_data = templatefile("${path.module}/scripts/postgres-replica.sh", {
    primary_host     = var.primary_db_host
    replication_user = var.db_replication_user
    replication_pass = var.db_replication_password
  })

  tags = local.common_tags

  lifecycle {
    ignore_changes = [user_data]
  }
}

# Volume for PostgreSQL data
resource "digitalocean_volume" "postgres_data" {
  name                    = "${local.name_prefix}-postgres-data"
  region                  = local.region
  size                    = 50
  initial_filesystem_type = "ext4"
  description             = "PostgreSQL data volume for APAC replica"
  tags                    = local.common_tags
}

resource "digitalocean_volume_attachment" "postgres_data" {
  droplet_id = digitalocean_droplet.postgres_replica.id
  volume_id  = digitalocean_volume.postgres_data.id
}

# =============================================================================
# Outputs
# =============================================================================

output "vpc_id" {
  description = "VPC ID"
  value       = digitalocean_vpc.main.id
}

output "k3s_master_ip" {
  description = "K3s master private IP"
  value       = digitalocean_droplet.k3s_master.ipv4_address_private
}

output "k3s_master_public_ip" {
  description = "K3s master public IP"
  value       = digitalocean_droplet.k3s_master.ipv4_address
}

output "k3s_worker_ips" {
  description = "K3s worker private IPs"
  value       = digitalocean_droplet.k3s_worker[*].ipv4_address_private
}

output "postgres_replica_ip" {
  description = "PostgreSQL replica private IP"
  value       = digitalocean_droplet.postgres_replica.ipv4_address_private
}

output "database_url" {
  description = "PostgreSQL connection URL (read replica)"
  value       = "postgresql://uno_app:${var.db_password}@${digitalocean_droplet.postgres_replica.ipv4_address_private}:5432/uno_app"
  sensitive   = true
}
