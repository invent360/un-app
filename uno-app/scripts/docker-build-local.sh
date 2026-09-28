#!/usr/bin/env bash
# Build uno-app Docker image with local dependencies
# This script creates a temporary build context that includes all path dependencies

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_DIR="$(dirname "$SCRIPT_DIR")"
UNETWORK_DIR="$(dirname "$APP_DIR")"

# Dependency paths
EMBER_FX_DIR="/Users/admin/Dev-x/polkanight/ember/ember-fx"

# Create temporary build context
BUILD_CONTEXT=$(mktemp -d)
trap "rm -rf $BUILD_CONTEXT" EXIT

echo "Creating build context in $BUILD_CONTEXT..."

# Copy uno-app source
echo "Copying uno-app..."
cp -r "$APP_DIR/src" "$BUILD_CONTEXT/src"
cp -r "$APP_DIR/assets" "$BUILD_CONTEXT/assets"
cp -r "$APP_DIR/style" "$BUILD_CONTEXT/style"
mkdir -p "$BUILD_CONTEXT/end2end"
cp -r "$APP_DIR/end2end"/* "$BUILD_CONTEXT/end2end/" 2>/dev/null || true
cp "$APP_DIR/Cargo.lock" "$BUILD_CONTEXT/Cargo.lock"

# Copy sibling dependencies
echo "Copying uno-api..."
mkdir -p "$BUILD_CONTEXT/deps/uno-api"
cp -r "$UNETWORK_DIR/uno-api/src" "$BUILD_CONTEXT/deps/uno-api/src"
cp "$UNETWORK_DIR/uno-api/Cargo.toml" "$BUILD_CONTEXT/deps/uno-api/Cargo.toml"
cp "$UNETWORK_DIR/uno-api/Cargo.lock" "$BUILD_CONTEXT/deps/uno-api/Cargo.lock" 2>/dev/null || true

echo "Copying file-storage..."
mkdir -p "$BUILD_CONTEXT/deps/file-storage"
cp -r "$UNETWORK_DIR/file-storage/src" "$BUILD_CONTEXT/deps/file-storage/src"
cp "$UNETWORK_DIR/file-storage/Cargo.toml" "$BUILD_CONTEXT/deps/file-storage/Cargo.toml"
cp "$UNETWORK_DIR/file-storage/Cargo.lock" "$BUILD_CONTEXT/deps/file-storage/Cargo.lock" 2>/dev/null || true

# Copy entire ember-fx workspace (needed for workspace dependencies)
echo "Copying ember-fx workspace..."
mkdir -p "$BUILD_CONTEXT/deps/ember-fx/crates"
cp "$EMBER_FX_DIR/Cargo.toml" "$BUILD_CONTEXT/deps/ember-fx/Cargo.toml"

# Copy all crates
for crate in common utils macros core styles icons systems components tools mobile layouts; do
    if [ -d "$EMBER_FX_DIR/crates/$crate" ]; then
        echo "  - $crate"
        cp -r "$EMBER_FX_DIR/crates/$crate" "$BUILD_CONTEXT/deps/ember-fx/crates/$crate"
    fi
done

# Create modified Cargo.toml with adjusted paths
echo "Updating Cargo.toml paths..."
sed -e 's|path = "../../../../Dev-x/polkanight/ember/ember-fx/crates/|path = "deps/ember-fx/crates/|g' \
    -e 's|path = "../uno-api"|path = "deps/uno-api"|g' \
    -e 's|path = "../file-storage"|path = "deps/file-storage"|g' \
    "$APP_DIR/Cargo.toml" > "$BUILD_CONTEXT/Cargo.toml"

# Copy Dockerfile.local (handles deps directory)
cp "$APP_DIR/Dockerfile.local" "$BUILD_CONTEXT/Dockerfile"

echo ""
echo "Build context structure:"
find "$BUILD_CONTEXT" -maxdepth 3 -type d | head -30

# Build the image
echo ""
echo "Building Docker image..."
docker build -t uno-app:local "$BUILD_CONTEXT"

echo ""
echo "Build complete! Image tagged as uno-app:local"
echo "To test: docker run -p 3000:3000 uno-app:local"
