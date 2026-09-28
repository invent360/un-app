# Cloudflare Global Configuration
# CDN, WAF, Load Balancing, and DNS

terraform {
  required_version = ">= 1.5.0"

  required_providers {
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.20"
    }
  }

  backend "gcs" {
    bucket = "uno-app-terraform-state"
    prefix = "global/cloudflare"
  }
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}

# =============================================================================
# Variables
# =============================================================================

variable "cloudflare_api_token" {
  description = "Cloudflare API token"
  type        = string
  sensitive   = true
}

variable "cloudflare_account_id" {
  description = "Cloudflare account ID"
  type        = string
}

variable "cloudflare_zone_id" {
  description = "Cloudflare zone ID for the domain"
  type        = string
}

variable "domain" {
  description = "Primary domain name"
  type        = string
}

variable "environment" {
  description = "Environment name"
  type        = string
  default     = "production"
}

variable "americas_origin" {
  description = "Americas origin hostname or IP"
  type        = string
}

variable "europe_origin" {
  description = "Europe origin hostname or IP"
  type        = string
}

variable "apac_origin" {
  description = "APAC origin hostname or IP"
  type        = string
}

# =============================================================================
# DNS Records
# =============================================================================

# Main A record (proxied through Cloudflare)
resource "cloudflare_record" "main" {
  zone_id = var.cloudflare_zone_id
  name    = "@"
  type    = "A"
  content = var.americas_origin # Fallback, Load Balancer handles routing
  proxied = true
  ttl     = 1 # Auto TTL when proxied
}

# www CNAME
resource "cloudflare_record" "www" {
  zone_id = var.cloudflare_zone_id
  name    = "www"
  type    = "CNAME"
  content = var.domain
  proxied = true
  ttl     = 1
}

# API subdomain
resource "cloudflare_record" "api" {
  zone_id = var.cloudflare_zone_id
  name    = "api"
  type    = "CNAME"
  content = var.domain
  proxied = true
  ttl     = 1
}

# =============================================================================
# Load Balancer
# =============================================================================

# Health Monitor
resource "cloudflare_load_balancer_monitor" "health" {
  account_id     = var.cloudflare_account_id
  type           = "https"
  method         = "GET"
  path           = "/api/v1/health"
  port           = 443
  timeout        = 5
  retries        = 2
  interval       = 60
  expected_codes = "200"
  description    = "Health check for uno-app"

  header {
    header = "Host"
    values = [var.domain]
  }
}

# Americas Pool
resource "cloudflare_load_balancer_pool" "americas" {
  account_id         = var.cloudflare_account_id
  name               = "uno-app-americas"
  description        = "Americas origin pool (GCP)"
  enabled            = true
  minimum_origins    = 1
  notification_email = var.alert_email

  origins {
    name    = "gcp-us-central"
    address = var.americas_origin
    enabled = true
    weight  = 1

    header {
      header = "Host"
      values = [var.domain]
    }
  }

  monitor = cloudflare_load_balancer_monitor.health.id

  origin_steering {
    policy = "random"
  }
}

# Europe Pool
resource "cloudflare_load_balancer_pool" "europe" {
  account_id         = var.cloudflare_account_id
  name               = "uno-app-europe"
  description        = "Europe origin pool (Hetzner)"
  enabled            = true
  minimum_origins    = 1
  notification_email = var.alert_email

  origins {
    name    = "hetzner-nbg"
    address = var.europe_origin
    enabled = true
    weight  = 1

    header {
      header = "Host"
      values = [var.domain]
    }
  }

  monitor = cloudflare_load_balancer_monitor.health.id

  origin_steering {
    policy = "random"
  }
}

# APAC Pool
resource "cloudflare_load_balancer_pool" "apac" {
  account_id         = var.cloudflare_account_id
  name               = "uno-app-apac"
  description        = "Asia-Pacific origin pool (DigitalOcean)"
  enabled            = true
  minimum_origins    = 1
  notification_email = var.alert_email

  origins {
    name    = "do-sgp"
    address = var.apac_origin
    enabled = true
    weight  = 1

    header {
      header = "Host"
      values = [var.domain]
    }
  }

  monitor = cloudflare_load_balancer_monitor.health.id

  origin_steering {
    policy = "random"
  }
}

# Load Balancer with Geo Routing
resource "cloudflare_load_balancer" "main" {
  zone_id          = var.cloudflare_zone_id
  name             = var.domain
  fallback_pool_id = cloudflare_load_balancer_pool.americas.id
  default_pool_ids = [
    cloudflare_load_balancer_pool.americas.id,
    cloudflare_load_balancer_pool.europe.id,
    cloudflare_load_balancer_pool.apac.id
  ]
  proxied     = true
  steering_policy = "geo"
  description = "Geo-routed load balancer for uno-app"

  # Americas
  region_pools {
    region   = "WNAM" # Western North America
    pool_ids = [cloudflare_load_balancer_pool.americas.id]
  }

  region_pools {
    region   = "ENAM" # Eastern North America
    pool_ids = [cloudflare_load_balancer_pool.americas.id]
  }

  region_pools {
    region   = "SAM" # South America
    pool_ids = [cloudflare_load_balancer_pool.americas.id]
  }

  # Europe & Africa
  region_pools {
    region   = "WEU" # Western Europe
    pool_ids = [cloudflare_load_balancer_pool.europe.id]
  }

  region_pools {
    region   = "EEU" # Eastern Europe
    pool_ids = [cloudflare_load_balancer_pool.europe.id]
  }

  region_pools {
    region   = "NSAF" # North & South Africa
    pool_ids = [cloudflare_load_balancer_pool.europe.id]
  }

  region_pools {
    region   = "ME" # Middle East
    pool_ids = [cloudflare_load_balancer_pool.europe.id]
  }

  # Asia-Pacific
  region_pools {
    region   = "SAS" # South Asia
    pool_ids = [cloudflare_load_balancer_pool.apac.id]
  }

  region_pools {
    region   = "SEAS" # Southeast Asia
    pool_ids = [cloudflare_load_balancer_pool.apac.id]
  }

  region_pools {
    region   = "NEAS" # Northeast Asia
    pool_ids = [cloudflare_load_balancer_pool.apac.id]
  }

  region_pools {
    region   = "OC" # Oceania
    pool_ids = [cloudflare_load_balancer_pool.apac.id]
  }

  session_affinity       = "cookie"
  session_affinity_ttl   = 1800
}

variable "alert_email" {
  description = "Email for load balancer alerts"
  type        = string
}

# =============================================================================
# Cache Rules
# =============================================================================

# Cache static assets aggressively
resource "cloudflare_ruleset" "cache_rules" {
  zone_id = var.cloudflare_zone_id
  name    = "Cache Rules"
  kind    = "zone"
  phase   = "http_request_cache_settings"

  # Cache WASM files for 1 year
  rules {
    action = "set_cache_settings"
    action_parameters {
      cache = true
      edge_ttl {
        mode    = "override_origin"
        default = 31536000 # 1 year
      }
      browser_ttl {
        mode    = "override_origin"
        default = 31536000
      }
    }
    expression  = "(http.request.uri.path.extension eq \"wasm\")"
    description = "Cache WASM files for 1 year"
    enabled     = true
  }

  # Cache CSS/JS files for 1 year
  rules {
    action = "set_cache_settings"
    action_parameters {
      cache = true
      edge_ttl {
        mode    = "override_origin"
        default = 31536000
      }
      browser_ttl {
        mode    = "override_origin"
        default = 31536000
      }
    }
    expression  = "(http.request.uri.path matches \"^/pkg/.*\\.(css|js)$\")"
    description = "Cache compiled assets for 1 year"
    enabled     = true
  }

  # Cache static assets for 1 month
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
        default = 2592000
      }
    }
    expression  = "(http.request.uri.path matches \"^/assets/.*\")"
    description = "Cache assets for 1 month"
    enabled     = true
  }

  # Bypass cache for API
  rules {
    action = "set_cache_settings"
    action_parameters {
      cache = false
    }
    expression  = "(http.request.uri.path matches \"^/api/.*\")"
    description = "Bypass cache for API"
    enabled     = true
  }

  # Cache HTML pages with stale-while-revalidate
  rules {
    action = "set_cache_settings"
    action_parameters {
      cache = true
      edge_ttl {
        mode    = "override_origin"
        default = 300 # 5 minutes
      }
      browser_ttl {
        mode    = "override_origin"
        default = 60 # 1 minute
      }
      serve_stale {
        disable_stale_while_updating = false
      }
    }
    expression  = "(http.request.uri.path eq \"/\" or http.request.method eq \"GET\") and not (http.request.uri.path matches \"^/(api|pkg|assets)/.*\")"
    description = "Cache HTML pages with stale-while-revalidate"
    enabled     = true
  }
}

# =============================================================================
# WAF Rules
# =============================================================================

resource "cloudflare_ruleset" "waf_custom" {
  zone_id = var.cloudflare_zone_id
  name    = "Custom WAF Rules"
  kind    = "zone"
  phase   = "http_request_firewall_custom"

  # Rate limit API endpoints
  rules {
    action = "block"
    ratelimit {
      characteristics     = ["ip.src"]
      period             = 60
      requests_per_period = 100
      mitigation_timeout  = 600
    }
    expression  = "(http.request.uri.path matches \"^/api/.*\")"
    description = "Rate limit API to 100 req/min per IP"
    enabled     = true
  }

  # Block SQL injection attempts
  rules {
    action     = "block"
    expression = "(http.request.uri.query contains \"UNION\" and http.request.uri.query contains \"SELECT\") or (http.request.uri.query contains \"DROP\" and http.request.uri.query contains \"TABLE\")"
    description = "Block SQL injection patterns"
    enabled     = true
  }

  # Require client ID header for admin endpoints
  rules {
    action     = "block"
    expression = "(http.request.uri.path matches \"^/api/v1/admin/.*\") and not (len(http.request.headers[\"x-client-id\"]) gt 0)"
    description = "Require x-client-id for admin endpoints"
    enabled     = true
  }

  # Block known bad user agents
  rules {
    action     = "block"
    expression = "(http.user_agent contains \"sqlmap\") or (http.user_agent contains \"nikto\") or (http.user_agent contains \"nmap\")"
    description = "Block known scanner user agents"
    enabled     = true
  }
}

# Enable OWASP Managed Rules
resource "cloudflare_ruleset" "waf_managed" {
  zone_id = var.cloudflare_zone_id
  name    = "Managed WAF Rules"
  kind    = "zone"
  phase   = "http_request_firewall_managed"

  rules {
    action = "execute"
    action_parameters {
      id = "efb7b8c949ac4650a09736fc376e9aee" # Cloudflare Managed Ruleset
    }
    expression  = "true"
    description = "Execute Cloudflare Managed Ruleset"
    enabled     = true
  }

  rules {
    action = "execute"
    action_parameters {
      id = "4814384a9e5d4991b9815dcfc25d2f1f" # OWASP Core Ruleset
    }
    expression  = "true"
    description = "Execute OWASP Core Ruleset"
    enabled     = true
  }
}

# =============================================================================
# Security Headers
# =============================================================================

resource "cloudflare_ruleset" "transform_headers" {
  zone_id = var.cloudflare_zone_id
  name    = "Security Headers"
  kind    = "zone"
  phase   = "http_response_headers_transform"

  rules {
    action = "set"
    action_parameters {
      headers {
        name      = "X-Content-Type-Options"
        value     = "nosniff"
        operation = "set"
      }
      headers {
        name      = "X-Frame-Options"
        value     = "DENY"
        operation = "set"
      }
      headers {
        name      = "X-XSS-Protection"
        value     = "1; mode=block"
        operation = "set"
      }
      headers {
        name      = "Referrer-Policy"
        value     = "strict-origin-when-cross-origin"
        operation = "set"
      }
      headers {
        name      = "Permissions-Policy"
        value     = "geolocation=(), microphone=(), camera=()"
        operation = "set"
      }
    }
    expression  = "true"
    description = "Add security headers to all responses"
    enabled     = true
  }
}

# =============================================================================
# SSL/TLS Settings
# =============================================================================

resource "cloudflare_zone_settings_override" "ssl_settings" {
  zone_id = var.cloudflare_zone_id

  settings {
    ssl                      = "strict"
    always_use_https         = "on"
    min_tls_version          = "1.2"
    tls_1_3                  = "on"
    automatic_https_rewrites = "on"
    opportunistic_encryption = "on"
    http2                    = "on"
    http3                    = "on"
    zero_rtt                 = "on"
    brotli                   = "on"
    minify {
      css  = "on"
      html = "on"
      js   = "on"
    }
    rocket_loader    = "off" # Disable for WASM compatibility
    early_hints      = "on"
    websockets       = "on"
    ip_geolocation   = "on"
    browser_check    = "on"
    challenge_ttl    = 1800
    security_level   = "medium"
    privacy_pass     = "on"
  }
}

# =============================================================================
# Cloudflare Tunnel (Zero Trust)
# =============================================================================

resource "cloudflare_tunnel" "uno_app" {
  account_id = var.cloudflare_account_id
  name       = "uno-app-${var.environment}"
  secret     = base64encode(var.tunnel_secret)
}

variable "tunnel_secret" {
  description = "Secret for Cloudflare Tunnel"
  type        = string
  sensitive   = true
}

resource "cloudflare_tunnel_config" "uno_app" {
  account_id = var.cloudflare_account_id
  tunnel_id  = cloudflare_tunnel.uno_app.id

  config {
    ingress_rule {
      hostname = var.domain
      service  = "http://uno-app.uno-app.svc.cluster.local:3000"
    }

    ingress_rule {
      hostname = "www.${var.domain}"
      service  = "http://uno-app.uno-app.svc.cluster.local:3000"
    }

    ingress_rule {
      hostname = "api.${var.domain}"
      service  = "http://uno-app.uno-app.svc.cluster.local:3000"
    }

    # Catch-all
    ingress_rule {
      service = "http_status:404"
    }
  }
}

# =============================================================================
# Outputs
# =============================================================================

output "load_balancer_hostname" {
  description = "Load balancer hostname"
  value       = cloudflare_load_balancer.main.name
}

output "tunnel_id" {
  description = "Cloudflare Tunnel ID"
  value       = cloudflare_tunnel.uno_app.id
}

output "tunnel_token" {
  description = "Cloudflare Tunnel token for deployment"
  value       = cloudflare_tunnel.uno_app.tunnel_token
  sensitive   = true
}

output "nameservers" {
  description = "Cloudflare nameservers for the zone"
  value       = "Check Cloudflare dashboard for nameservers"
}
