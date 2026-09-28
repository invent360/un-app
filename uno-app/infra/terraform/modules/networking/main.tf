# Networking Module
# Configures VPC, subnets, and firewall rules

terraform {
  required_version = ">= 1.5.0"
}

variable "name" {
  description = "Network name prefix"
  type        = string
}

variable "region" {
  description = "Cloud region"
  type        = string
}

variable "cidr_block" {
  description = "CIDR block for the VPC"
  type        = string
  default     = "10.0.0.0/16"
}

variable "subnet_cidr" {
  description = "CIDR block for the main subnet"
  type        = string
  default     = "10.0.1.0/24"
}

variable "enable_nat" {
  description = "Enable NAT gateway for private subnets"
  type        = bool
  default     = false
}

variable "tags" {
  description = "Tags to apply to resources"
  type        = map(string)
  default     = {}
}

# Cloudflare IP ranges for firewall rules
# These should be kept up to date: https://www.cloudflare.com/ips/
locals {
  cloudflare_ipv4_ranges = [
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

  cloudflare_ipv6_ranges = [
    "2400:cb00::/32",
    "2606:4700::/32",
    "2803:f800::/32",
    "2405:b500::/32",
    "2405:8100::/32",
    "2a06:98c0::/29",
    "2c0f:f248::/32"
  ]
}

output "cloudflare_ipv4_ranges" {
  description = "Cloudflare IPv4 ranges for firewall rules"
  value       = local.cloudflare_ipv4_ranges
}

output "cloudflare_ipv6_ranges" {
  description = "Cloudflare IPv6 ranges for firewall rules"
  value       = local.cloudflare_ipv6_ranges
}

output "name" {
  description = "Network name"
  value       = var.name
}

output "cidr_block" {
  description = "VPC CIDR block"
  value       = var.cidr_block
}

output "subnet_cidr" {
  description = "Subnet CIDR block"
  value       = var.subnet_cidr
}
