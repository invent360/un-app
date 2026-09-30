#!/usr/bin/env bash
# Both applications and canonical dependencies are built from the root workspace.
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPOSITORY_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
exec docker build --file "$REPOSITORY_ROOT/uno-app/Dockerfile.local" --tag uno-app:local "$@" "$REPOSITORY_ROOT"
