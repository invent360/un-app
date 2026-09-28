# Monitoring Module
# Configures Grafana Agent for metrics, logs, and traces

terraform {
  required_version = ">= 1.5.0"
}

variable "cluster_name" {
  description = "Name of the cluster being monitored"
  type        = string
}

variable "region" {
  description = "Cloud region"
  type        = string
}

variable "grafana_cloud_url" {
  description = "Grafana Cloud Prometheus remote write URL"
  type        = string
}

variable "grafana_cloud_user" {
  description = "Grafana Cloud user ID"
  type        = string
}

variable "grafana_cloud_api_key" {
  description = "Grafana Cloud API key"
  type        = string
  sensitive   = true
}

variable "loki_url" {
  description = "Grafana Cloud Loki URL"
  type        = string
}

variable "tempo_url" {
  description = "Grafana Cloud Tempo URL"
  type        = string
  default     = ""
}

variable "scrape_interval" {
  description = "Metrics scrape interval"
  type        = string
  default     = "60s"
}

variable "namespace" {
  description = "Kubernetes namespace for monitoring"
  type        = string
  default     = "monitoring"
}

# Grafana Agent configuration
locals {
  grafana_agent_config = yamlencode({
    metrics = {
      global = {
        scrape_interval = var.scrape_interval
        external_labels = {
          cluster = var.cluster_name
          region  = var.region
        }
      }
      configs = [
        {
          name = "uno-app"
          remote_write = [
            {
              url = var.grafana_cloud_url
              basic_auth = {
                username = var.grafana_cloud_user
                password = var.grafana_cloud_api_key
              }
            }
          ]
          scrape_configs = [
            {
              job_name = "kubernetes-pods"
              kubernetes_sd_configs = [
                {
                  role = "pod"
                }
              ]
              relabel_configs = [
                {
                  source_labels = ["__meta_kubernetes_pod_annotation_prometheus_io_scrape"]
                  regex         = "true"
                  action        = "keep"
                },
                {
                  source_labels = ["__meta_kubernetes_pod_annotation_prometheus_io_path"]
                  target_label  = "__metrics_path__"
                  regex         = "(.+)"
                },
                {
                  source_labels = ["__address__", "__meta_kubernetes_pod_annotation_prometheus_io_port"]
                  target_label  = "__address__"
                  regex         = "([^:]+)(?::\\d+)?;(\\d+)"
                  replacement   = "$1:$2"
                }
              ]
            },
            {
              job_name = "kubernetes-nodes"
              kubernetes_sd_configs = [
                {
                  role = "node"
                }
              ]
              relabel_configs = [
                {
                  target_label = "__address__"
                  replacement  = "kubernetes.default.svc:443"
                },
                {
                  source_labels = ["__meta_kubernetes_node_name"]
                  target_label  = "__metrics_path__"
                  replacement   = "/api/v1/nodes/$1/proxy/metrics"
                }
              ]
            }
          ]
        }
      ]
    }
    logs = {
      configs = [
        {
          name = "uno-logs"
          clients = [
            {
              url = var.loki_url
              basic_auth = {
                username = var.grafana_cloud_user
                password = var.grafana_cloud_api_key
              }
              external_labels = {
                cluster = var.cluster_name
                region  = var.region
              }
            }
          ]
          positions = {
            filename = "/tmp/positions.yaml"
          }
          scrape_configs = [
            {
              job_name = "kubernetes-pods"
              kubernetes_sd_configs = [
                {
                  role = "pod"
                }
              ]
              relabel_configs = [
                {
                  source_labels = ["__meta_kubernetes_pod_label_app"]
                  target_label  = "app"
                },
                {
                  source_labels = ["__meta_kubernetes_namespace"]
                  target_label  = "namespace"
                },
                {
                  source_labels = ["__meta_kubernetes_pod_name"]
                  target_label  = "pod"
                }
              ]
            }
          ]
        }
      ]
    }
  })
}

output "grafana_agent_config" {
  description = "Grafana Agent configuration"
  value       = local.grafana_agent_config
  sensitive   = true
}

output "namespace" {
  description = "Monitoring namespace"
  value       = var.namespace
}

# Alert rules for uno-app
locals {
  alert_rules = yamlencode({
    groups = [
      {
        name = "uno-app-alerts"
        rules = [
          {
            alert = "HighErrorRate"
            expr  = "sum(rate(http_requests_total{status=~\"5..\"}[5m])) / sum(rate(http_requests_total[5m])) > 0.01"
            for   = "5m"
            labels = {
              severity = "critical"
            }
            annotations = {
              summary     = "Error rate above 1%"
              description = "The error rate for {{ $labels.instance }} is {{ $value | humanizePercentage }}"
            }
          },
          {
            alert = "HighLatency"
            expr  = "histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m])) > 2"
            for   = "5m"
            labels = {
              severity = "warning"
            }
            annotations = {
              summary     = "P95 latency above 2s"
              description = "P95 latency is {{ $value | humanizeDuration }}"
            }
          },
          {
            alert = "DatabaseReplicationLag"
            expr  = "pg_replication_lag_seconds > 60"
            for   = "5m"
            labels = {
              severity = "warning"
            }
            annotations = {
              summary     = "Database replication lag above 60s"
              description = "Replication lag is {{ $value | humanizeDuration }}"
            }
          },
          {
            alert = "PodRestarts"
            expr  = "increase(kube_pod_container_status_restarts_total[1h]) > 3"
            labels = {
              severity = "warning"
            }
            annotations = {
              summary     = "Pod restarting frequently"
              description = "Pod {{ $labels.pod }} has restarted {{ $value }} times in the last hour"
            }
          },
          {
            alert = "HighCPUUsage"
            expr  = "avg(rate(container_cpu_usage_seconds_total{container=\"uno-app\"}[5m])) > 0.8"
            for   = "10m"
            labels = {
              severity = "warning"
            }
            annotations = {
              summary     = "High CPU usage"
              description = "CPU usage is above 80% for 10 minutes"
            }
          },
          {
            alert = "HighMemoryUsage"
            expr  = "avg(container_memory_usage_bytes{container=\"uno-app\"}) / avg(container_spec_memory_limit_bytes{container=\"uno-app\"}) > 0.85"
            for   = "10m"
            labels = {
              severity = "warning"
            }
            annotations = {
              summary     = "High memory usage"
              description = "Memory usage is above 85% for 10 minutes"
            }
          }
        ]
      }
    ]
  })
}

output "alert_rules" {
  description = "Prometheus alert rules"
  value       = local.alert_rules
}
