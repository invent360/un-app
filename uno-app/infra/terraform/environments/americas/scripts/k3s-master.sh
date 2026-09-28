#!/bin/bash
# K3s Master Installation Script
set -e

# Variables from Terraform template
K3S_TOKEN="${k3s_token}"
K3S_VERSION="${k3s_version}"
CLUSTER_NAME="${cluster_name}"

echo "Starting K3s master installation..."

# Wait for cloud-init to complete
cloud-init status --wait || true

# Install required packages
apt-get update
apt-get install -y curl ca-certificates gnupg lsb-release jq

# Install K3s master
curl -sfL https://get.k3s.io | INSTALL_K3S_VERSION="$K3S_VERSION" sh -s - server \
  --cluster-init \
  --token="$K3S_TOKEN" \
  --tls-san="$CLUSTER_NAME" \
  --disable=traefik \
  --disable=servicelb \
  --write-kubeconfig-mode=644 \
  --kubelet-arg="max-pods=110" \
  --kube-apiserver-arg="default-not-ready-toleration-seconds=30" \
  --kube-apiserver-arg="default-unreachable-toleration-seconds=30"

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

# Install Traefik CRDs
kubectl apply -f https://raw.githubusercontent.com/traefik/traefik-helm-chart/master/traefik/crds/ingressroute.yaml || true
kubectl apply -f https://raw.githubusercontent.com/traefik/traefik-helm-chart/master/traefik/crds/middlewares.yaml || true

# Install metrics-server
kubectl apply -f https://github.com/kubernetes-sigs/metrics-server/releases/latest/download/components.yaml || true

echo "K3s master installation complete!"
echo "Kubeconfig available at: /etc/rancher/k3s/k3s.yaml"
