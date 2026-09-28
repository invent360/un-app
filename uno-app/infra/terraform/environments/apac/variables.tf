# Variables for APAC (DigitalOcean) region

variable "do_token" {
  description = "DigitalOcean API token"
  type        = string
  sensitive   = true
}

variable "environment" {
  description = "Environment name (development, staging, production)"
  type        = string
  default     = "production"
}

# K3s Configuration
variable "master_size" {
  description = "Droplet size for K3s master"
  type        = string
  default     = "s-1vcpu-2gb"
}

variable "worker_size" {
  description = "Droplet size for K3s workers"
  type        = string
  default     = "s-1vcpu-2gb"
}

variable "worker_count" {
  description = "Number of K3s worker nodes"
  type        = number
  default     = 1
}

variable "k3s_version" {
  description = "K3s version to install"
  type        = string
  default     = "v1.29.0+k3s1"
}

variable "k3s_token" {
  description = "K3s cluster join token"
  type        = string
  sensitive   = true
}

# SSH Configuration
variable "ssh_public_key" {
  description = "SSH public key for node access"
  type        = string
}

variable "ssh_allowed_ips" {
  description = "IP addresses allowed to SSH to nodes"
  type        = list(string)
  default     = []
}

# Database Configuration
variable "db_size" {
  description = "Droplet size for PostgreSQL replica"
  type        = string
  default     = "s-1vcpu-2gb"
}

variable "db_password" {
  description = "Database password for uno_app user"
  type        = string
  sensitive   = true
}

variable "db_replication_user" {
  description = "Replication user name"
  type        = string
  default     = "replicator"
}

variable "db_replication_password" {
  description = "Database password for replication user"
  type        = string
  sensitive   = true
}

variable "primary_db_host" {
  description = "Primary database host (GCP Cloud SQL private IP)"
  type        = string
}
