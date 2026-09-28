#!/bin/bash
# K3s Worker Installation Script
set -e

# Variables from Terraform template
K3S_TOKEN="${k3s_token}"
K3S_VERSION="${k3s_version}"
MASTER_IP="${master_ip}"

echo "Starting K3s worker installation..."

# Wait for cloud-init to complete
cloud-init status --wait || true

# Install required packages
apt-get update
apt-get install -y curl ca-certificates netcat-openbsd

# Wait for master to be available
echo "Waiting for K3s master at $MASTER_IP:6443..."
until nc -z "$MASTER_IP" 6443; do
  echo "Master not ready yet, waiting..."
  sleep 10
done

# Give master a bit more time to fully initialize
sleep 30

# Install K3s agent
curl -sfL https://get.k3s.io | INSTALL_K3S_VERSION="$K3S_VERSION" K3S_URL="https://$MASTER_IP:6443" K3S_TOKEN="$K3S_TOKEN" sh -s - agent \
  --kubelet-arg="max-pods=110"

echo "K3s worker installation complete!"
