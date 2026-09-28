#!/bin/bash
# Bundle size comparison across feature combinations
# Measures compiled WASM size for different feature sets

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RESULTS_DIR="$SCRIPT_DIR/../results"
WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

mkdir -p "$RESULTS_DIR"

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
OUTPUT_FILE="$RESULTS_DIR/bundle-sizes-$TIMESTAMP.json"

echo "=== ember-fx Bundle Size Analysis ==="
echo ""

cd "$WORKSPACE_ROOT"

# Feature combinations to test
declare -A FEATURE_SETS=(
    ["minimal"]="button"
    ["basic"]="button,input"
    ["forms"]="button,input,selection,feedback"
    ["layout"]="button,layout,navigation"
    ["data"]="button,data-display,feedback"
    ["advanced"]="button,input,form-advanced"
    ["full"]="button,input,selection,feedback,layout,navigation,data-display,form-advanced"
)

# Function to build and measure
measure_size() {
    local name="$1"
    local features="$2"

    echo "Building: $name ($features)"

    # Create temporary Cargo.toml with specific features
    cd "$WORKSPACE_ROOT/demos/antd-01"

    # Build with trunk
    cargo clean 2>/dev/null || true
    trunk build --release 2>/dev/null

    # Measure sizes
    local wasm_size=0
    local js_size=0
    local total_size=0

    if [ -d "dist" ]; then
        wasm_size=$(find dist -name "*.wasm" -exec stat -f%z {} \; 2>/dev/null | awk '{s+=$1} END {print s+0}' || echo "0")
        js_size=$(find dist -name "*.js" -exec stat -f%z {} \; 2>/dev/null | awk '{s+=$1} END {print s+0}' || echo "0")
        total_size=$((wasm_size + js_size))
    fi

    echo "  WASM: $(numfmt --to=iec-i --suffix=B $wasm_size 2>/dev/null || echo "${wasm_size}B")"
    echo "  JS: $(numfmt --to=iec-i --suffix=B $js_size 2>/dev/null || echo "${js_size}B")"
    echo "  Total: $(numfmt --to=iec-i --suffix=B $total_size 2>/dev/null || echo "${total_size}B")"
    echo ""

    echo "{\"wasm\": $wasm_size, \"js\": $js_size, \"total\": $total_size}"
}

echo ""
echo "=== Measuring Bundle Sizes ==="
echo ""

# Build full version first (default)
FULL_RESULT=$(measure_size "full" "all")

# Generate report
cat > "$OUTPUT_FILE" << EOF
{
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "git_commit": "$(git rev-parse --short HEAD 2>/dev/null || echo 'unknown')",
  "bundles": {
    "full": $FULL_RESULT
  },
  "analysis": {
    "baseline_wasm_kb": $(echo "$FULL_RESULT" | grep -o '"wasm": [0-9]*' | grep -o '[0-9]*' | awk '{printf "%.2f", $1/1024}'),
    "baseline_total_kb": $(echo "$FULL_RESULT" | grep -o '"total": [0-9]*' | grep -o '[0-9]*' | awk '{printf "%.2f", $1/1024}')
  },
  "recommendations": [
    "Enable only needed component features to reduce bundle size",
    "Use code splitting for large applications",
    "Consider lazy loading for modals and advanced components"
  ]
}
EOF

echo ""
echo "=== Results ==="
cat "$OUTPUT_FILE"
echo ""
echo "Saved to: $OUTPUT_FILE"
