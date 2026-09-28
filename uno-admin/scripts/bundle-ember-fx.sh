#!/bin/bash
# Bundle ember-fx-styles CSS for uno-admin (Prime design system)
# Run this script after building ember-fx-styles

EMBER_FX_PATH="/Users/admin/Dev-x/polkanight/ember/ember-fx/crates/styles/src/compiled"
OUTPUT_DIR="$(dirname "$0")/../assets/styles"

# Create output directory
mkdir -p "$OUTPUT_DIR"

echo "Bundling ember-fx styles (Prime design system for uno-admin)..."

# Create a combined theme variables file
cat > "$OUTPUT_DIR/ember-fx-themes.css" << 'EOF'
/* ember-fx-styles: Theme Variables Bundle (Prime) */
/* Auto-generated - do not edit directly */
/* uno-admin uses Prime design system */

EOF

# Append Prime theme CSS files
for theme in lara-light lara-dark aura-light aura-dark material-light material-dark; do
    if [ -f "$EMBER_FX_PATH/prime/$theme.css" ]; then
        echo "/* Theme: $theme */" >> "$OUTPUT_DIR/ember-fx-themes.css"
        cat "$EMBER_FX_PATH/prime/$theme.css" >> "$OUTPUT_DIR/ember-fx-themes.css"
        echo "" >> "$OUTPUT_DIR/ember-fx-themes.css"
        echo "Added theme: $theme"
    fi
done

# Copy Prime component styles
cp "$EMBER_FX_PATH/prime/components.css" "$OUTPUT_DIR/ember-fx-components.css"
echo "Copied: prime/components.css"

echo "Done! CSS files written to: $OUTPUT_DIR"
echo ""
echo "Files created:"
ls -la "$OUTPUT_DIR"
