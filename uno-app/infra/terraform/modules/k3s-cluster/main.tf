# K3s Cluster Module
# Provisions a lightweight Kubernetes cluster using K3s

terraform {
  required_version = ">= 1.5.0"
}

variable "cluster_name" {
  description = "Name of the K3s cluster"
  type        = string
}

variable "region" {
  description = "Cloud region for the cluster"
  type        = string
}

variable "master_count" {
  description = "Number of master nodes"
  type        = number
  default     = 1
}

variable "worker_count" {
  description = "Number of worker nodes"
  type        = number
  default     = 2
}

variable "master_instance_type" {
  description = "Instance type for master nodes"
  type        = string
}

variable "worker_instance_type" {
  description = "Instance type for worker nodes"
  type        = string
}

variable "ssh_public_key" {
  description = "SSH public key for node access"
  type        = string
}

variable "k3s_token" {
  description = "Token for K3s cluster join"
  type        = string
  sensitive   = true
}

variable "k3s_version" {
  description = "K3s version to install"
  type        = string
  default     = "v1.29.0+k3s1"
}

variable "network_id" {
  description = "VPC/Network ID for the cluster"
  type        = string
}

variable "subnet_id" {
  description = "Subnet ID for the cluster"
  type        = string
}

variable "tags" {
  description = "Tags to apply to resources"
  type        = map(string)
  default     = {}
}

# K3s installation script for master node
locals {
  k3s_master_install_script = <<-EOF
    #!/bin/bash
    set -e

    # Wait for cloud-init to complete
    cloud-init status --wait

    # Install K3s master
    curl -sfL https://get.k3s.io | INSTALL_K3S_VERSION="${var.k3s_version}" sh -s - server \
      --cluster-init \
      --token="${var.k3s_token}" \
      --tls-san="${var.cluster_name}.${var.region}" \
      --disable=traefik \
      --write-kubeconfig-mode=644

    # Wait for K3s to be ready
    until kubectl get nodes; do
      sleep 5
    done

    # Install Traefik with custom config
    kubectl apply -f https://raw.githubusercontent.com/traefik/traefik-helm-chart/master/traefik/crds/ingressroute.yaml

    echo "K3s master installation complete"
  EOF

  k3s_worker_install_script = <<-EOF
    #!/bin/bash
    set -e

    # Wait for cloud-init to complete
    cloud-init status --wait

    # Wait for master to be available
    until nc -z $${MASTER_IP} 6443; do
      echo "Waiting for K3s master..."
      sleep 10
    done

    # Install K3s agent
    curl -sfL https://get.k3s.io | INSTALL_K3S_VERSION="${var.k3s_version}" K3S_URL="https://$${MASTER_IP}:6443" K3S_TOKEN="${var.k3s_token}" sh -

    echo "K3s worker installation complete"
  EOF
}

output "master_install_script" {
  description = "K3s master installation script"
  value       = local.k3s_master_install_script
  sensitive   = true
}

output "worker_install_script" {
  description = "K3s worker installation script"
  value       = local.k3s_worker_install_script
  sensitive   = true
}

output "cluster_name" {
  description = "Name of the cluster"
  value       = var.cluster_name
}
