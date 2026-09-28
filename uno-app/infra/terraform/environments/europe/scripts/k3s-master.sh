#!/bin/bash
# K3s Master Installation Script for Hetzner
set -e

# Variables from Terraform template
K3S_TOKEN="${k3s_token}"
K3S_VERSION="${k3s_version}"
CLUSTER_NAME="${cluster_name}"

echo "Starting K3s master installation..."

# Install required packages
apt-get update
apt-get install -y curl ca-certificates gnupg jq

# Install K3s master
curl -sfL https://get.k3s.io | INSTALL_K3S_VERSION="$K3S_VERSION" sh -s - server \
  --cluster-init \
  --token="$K3S_TOKEN" \
  --tls-san="$CLUSTER_NAME" \
  --disable=traefik \
  --disable=servicelb \
  --write-kubeconfig-mode=644 \
  --kubelet-arg="max-pods=110" \
  --node-label="region=europe" \
  --node-label="provider=hetzner"

# Wait for K3s to be ready
echo "Waiting for K3s to be ready..."
until kubectl get nodes; do
  echo "K3s not ready yet, waiting..."
  sleep 5
done

# Install Helm
curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash

# Create namespaces
kubectl create namespace uno-app --dry-run=client -o yaml | kubectl apply -f -
kubectl create namespace monitoring --dry-run=client -o yaml | kubectl apply -f -
kubectl create namespace cloudflared --dry-run=client -o yaml | kubectl apply -f -

# Install metrics-server
kubectl apply -f https://github.com/kubernetes-sigs/metrics-server/releases/latest/download/components.yaml || true

echo "K3s master installation complete!"
