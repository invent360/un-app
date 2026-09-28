#!/bin/bash
# Build-time metrics collection for ember-fx
# Measures compilation time for different feature combinations

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RESULTS_DIR="$SCRIPT_DIR/../results"
WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

mkdir -p "$RESULTS_DIR"

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
OUTPUT_FILE="$RESULTS_DIR/build-metrics-$TIMESTAMP.json"

echo "=== ember-fx Build Metrics ==="
echo "Workspace: $WORKSPACE_ROOT"
echo "Output: $OUTPUT_FILE"
echo ""

cd "$WORKSPACE_ROOT"

# Clean previous builds
cargo clean 2>/dev/null || true

# Function to measure build time
measure_build() {
    local name="$1"
    local features="$2"

    echo "Building: $name"

    # Clean incremental build artifacts
    cargo clean -p ember-fx-components 2>/dev/null || true

    local start_time=$(date +%s.%N)

    if [ -z "$features" ]; then
        cargo build -p ember-fx-components --release 2>/dev/null
    else
        cargo build -p ember-fx-components --release --features "$features" 2>/dev/null
    fi

    local end_time=$(date +%s.%N)
    local duration=$(echo "$end_time - $start_time" | bc)

    echo "  Time: ${duration}s"
    echo "$duration"
}

# Function to get binary size
get_size() {
    local path="$1"
    if [ -f "$path" ]; then
        stat -f%z "$path" 2>/dev/null || stat -c%s "$path" 2>/dev/null || echo "0"
    else
        echo "0"
    fi
}

# Collect metrics
echo ""
echo "=== Measuring Build Times ==="

# Default build (all features)
TIME_ALL=$(measure_build "all-features" "")

# Minimal build
TIME_MINIMAL=$(measure_build "minimal" "button")

# Core components only
TIME_CORE=$(measure_build "core-components" "button,input,feedback")

# Full UI
TIME_FULL=$(measure_build "full-ui" "button,input,selection,feedback,layout,navigation,data-display,form-advanced")

echo ""
echo "=== Measuring WASM Build ==="

cd "$WORKSPACE_ROOT/demos/antd-01"

# Clean and build WASM
cargo clean 2>/dev/null || true

WASM_START=$(date +%s.%N)
trunk build --release 2>/dev/null || echo "Trunk build skipped"
WASM_END=$(date +%s.%N)
WASM_TIME=$(echo "$WASM_END - $WASM_START" | bc 2>/dev/null || echo "0")

# Get WASM file size
WASM_SIZE=$(get_size "dist/*.wasm" 2>/dev/null || echo "0")
JS_SIZE=$(get_size "dist/*.js" 2>/dev/null || echo "0")

cd "$WORKSPACE_ROOT"

echo ""
echo "=== Generating Report ==="

# Generate JSON report
cat > "$OUTPUT_FILE" << EOF
{
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "git_commit": "$(git rev-parse --short HEAD 2>/dev/null || echo 'unknown')",
  "git_branch": "$(git branch --show-current 2>/dev/null || echo 'unknown')",
  "rust_version": "$(rustc --version | awk '{print $2}')",
  "build_times": {
    "all_features": $TIME_ALL,
    "minimal": $TIME_MINIMAL,
    "core_components": $TIME_CORE,
    "full_ui": $TIME_FULL,
    "wasm_release": $WASM_TIME
  },
  "sizes": {
    "wasm_bytes": $WASM_SIZE,
    "js_bytes": $JS_SIZE
  },
  "features_tested": [
    "all-features",
    "minimal (button only)",
    "core-components (button, input, feedback)",
    "full-ui (all component categories)"
  ]
}
EOF

echo "Results saved to: $OUTPUT_FILE"
echo ""
cat "$OUTPUT_FILE"
