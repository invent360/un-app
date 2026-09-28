# Cloudflare R2 Object Storage
# For images and media storage with zero egress fees

terraform {
  required_version = ">= 1.5.0"
}

# R2 Bucket for images
resource "cloudflare_r2_bucket" "images" {
  account_id = var.cloudflare_account_id
  name       = "uno-app-images-${var.environment}"
  location   = "WNAM" # Western North America
}

# R2 Bucket for backups
resource "cloudflare_r2_bucket" "backups" {
  account_id = var.cloudflare_account_id
  name       = "uno-app-backups-${var.environment}"
  location   = "WNAM"
}

# Public access for images bucket (via custom domain)
resource "cloudflare_record" "images_cdn" {
  zone_id = var.cloudflare_zone_id
  name    = "images"
  type    = "CNAME"
  content = "${cloudflare_r2_bucket.images.name}.${var.cloudflare_account_id}.r2.cloudflarestorage.com"
  proxied = true
  ttl     = 1
}

# Cache rules for R2 images
resource "cloudflare_ruleset" "r2_cache" {
  zone_id = var.cloudflare_zone_id
  name    = "R2 Images Cache"
  kind    = "zone"
  phase   = "http_request_cache_settings"

  rules {
    action = "set_cache_settings"
    action_parameters {
      cache = true
      edge_ttl {
        mode    = "override_origin"
        default = 2592000 # 1 month
      }
      browser_ttl {
        mode    = "override_origin"
        default = 604800 # 1 week
      }
    }
    expression  = "(http.host eq \"images.${var.domain}\")"
    description = "Cache R2 images"
    enabled     = true
  }
}

# =============================================================================
# Outputs
# =============================================================================

output "images_bucket_name" {
  description = "R2 images bucket name"
  value       = cloudflare_r2_bucket.images.name
}

output "backups_bucket_name" {
  description = "R2 backups bucket name"
  value       = cloudflare_r2_bucket.backups.name
}

output "images_public_url" {
  description = "Public URL for images"
  value       = "https://images.${var.domain}"
}

output "r2_endpoint" {
  description = "R2 S3-compatible endpoint"
  value       = "https://${var.cloudflare_account_id}.r2.cloudflarestorage.com"
}
