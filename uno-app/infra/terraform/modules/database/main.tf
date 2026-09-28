# PostgreSQL Database Module
# Configures PostgreSQL with replication support

terraform {
  required_version = ">= 1.5.0"
}

variable "name" {
  description = "Database instance name"
  type        = string
}

variable "region" {
  description = "Cloud region"
  type        = string
}

variable "tier" {
  description = "Database tier/size"
  type        = string
  default     = "db-f1-micro"
}

variable "postgres_version" {
  description = "PostgreSQL version"
  type        = string
  default     = "POSTGRES_16"
}

variable "network_id" {
  description = "VPC network ID"
  type        = string
}

variable "database_name" {
  description = "Name of the database to create"
  type        = string
  default     = "uno_app"
}

variable "database_user" {
  description = "Database user name"
  type        = string
  default     = "uno_app"
}

variable "database_password" {
  description = "Database user password"
  type        = string
  sensitive   = true
}

variable "backup_enabled" {
  description = "Enable automated backups"
  type        = bool
  default     = true
}

variable "backup_start_time" {
  description = "Backup start time (HH:MM format)"
  type        = string
  default     = "02:00"
}

variable "point_in_time_recovery" {
  description = "Enable point-in-time recovery"
  type        = bool
  default     = true
}

variable "max_connections" {
  description = "Maximum database connections"
  type        = number
  default     = 100
}

variable "deletion_protection" {
  description = "Enable deletion protection"
  type        = bool
  default     = true
}

variable "is_replica" {
  description = "Whether this is a read replica"
  type        = bool
  default     = false
}

variable "primary_instance_name" {
  description = "Primary instance name (for replicas)"
  type        = string
  default     = ""
}

variable "tags" {
  description = "Tags to apply to resources"
  type        = map(string)
  default     = {}
}

# PostgreSQL configuration for replication
locals {
  postgres_flags = var.is_replica ? [
    {
      name  = "hot_standby"
      value = "on"
    },
    {
      name  = "max_standby_streaming_delay"
      value = "30000"
    }
  ] : [
    {
      name  = "max_connections"
      value = tostring(var.max_connections)
    },
    {
      name  = "wal_level"
      value = "replica"
    },
    {
      name  = "max_wal_senders"
      value = "10"
    },
    {
      name  = "max_replication_slots"
      value = "10"
    }
  ]
}

output "database_flags" {
  description = "PostgreSQL configuration flags"
  value       = local.postgres_flags
}

output "connection_name" {
  description = "Database connection name"
  value       = var.name
}

output "database_name" {
  description = "Database name"
  value       = var.database_name
}
