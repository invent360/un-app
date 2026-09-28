# Variables for Americas (GCP) region

variable "project_id" {
  description = "GCP project ID"
  type        = string
}

variable "region" {
  description = "GCP region"
  type        = string
  default     = "us-central1"
}

variable "environment" {
  description = "Environment name (development, staging, production)"
  type        = string
  default     = "production"
}

# K3s Configuration
variable "master_instance_type" {
  description = "Instance type for K3s master"
  type        = string
  default     = "e2-small"
}

variable "worker_instance_type" {
  description = "Instance type for K3s workers"
  type        = string
  default     = "e2-small"
}

variable "worker_count" {
  description = "Number of K3s worker nodes"
  type        = number
  default     = 2
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
variable "ssh_user" {
  description = "SSH username"
  type        = string
  default     = "admin"
}

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
variable "db_tier" {
  description = "Cloud SQL tier"
  type        = string
  default     = "db-f1-micro"
}

variable "db_password" {
  description = "Database password for uno_app user"
  type        = string
  sensitive   = true
}

variable "db_replication_password" {
  description = "Database password for replication user"
  type        = string
  sensitive   = true
}
