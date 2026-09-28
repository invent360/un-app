//! Build script for ember-fx-styles.
//!
//! Compiles JSON theme presets to CSS and minifies component CSS at build time.
//! The resulting CSS is embedded into the binary via `include_str!()`.

use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    // Rerun if themes directory changes
    println!("cargo:rerun-if-changed=themes/");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let themes_dir = Path::new(&manifest_dir).join("themes");
    let output_dir = Path::new(&manifest_dir).join("src/compiled");

    // Ensure output directory exists
    if let Err(e) = fs::create_dir_all(&output_dir) {
        eprintln!("Warning: Failed to create compiled directory: {}", e);
        return;
    }

    // 1. Compile JSON theme presets to CSS
    compile_json_themes(&themes_dir.join("presets"), &output_dir);

    // 2. Compile and minify component CSS per design system
    compile_component_css(&themes_dir.join("base"), &output_dir);

    // 3. Generate the themes manifest (list of available themes)
    generate_themes_manifest(&themes_dir.join("presets"), &output_dir);
}

/// Compile JSON theme presets to CSS variable declarations.
fn compile_json_themes(presets_dir: &Path, output_dir: &Path) {
    if !presets_dir.exists() {
        eprintln!("Warning: presets directory does not exist: {:?}", presets_dir);
        return;
    }

    for design_system in WalkDir::new(presets_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        let system_name = match design_system.file_name().to_str() {
            Some(name) => name,
            None => continue,
        };
        let system_output_dir = output_dir.join(system_name);
        let _ = fs::create_dir_all(&system_output_dir);

        for entry in WalkDir::new(design_system.path())
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .map(|ext| ext == "json")
                    .unwrap_or(false)
            })
        {
            let json_path = entry.path();
            let theme_name = match json_path.file_stem().and_then(|s| s.to_str()) {
                Some(name) => name,
                None => continue,
            };

            match fs::read_to_string(json_path) {
                Ok(json_content) => {
                    match serde_json::from_str::<serde_json::Value>(&json_content) {
                        Ok(theme_json) => {
                            let css = json_to_css(&theme_json, theme_name);
                            let minified = minify_css(&css);

                            let output_path = system_output_dir.join(format!("{}.css", theme_name));
                            if let Err(e) = fs::write(&output_path, minified) {
                                eprintln!("Warning: Failed to write compiled CSS: {}", e);
                            } else {
                                println!(
                                    "cargo:warning=Compiled theme: {}/{}",
                                    system_name, theme_name
                                );
                            }
                        }
                        Err(e) => {
                            eprintln!("Warning: Failed to parse JSON {}: {}", json_path.display(), e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to read {}: {}", json_path.display(), e);
                }
            }
        }
    }
}

/// Convert a JSON theme definition to CSS variable declarations.
fn json_to_css(theme: &serde_json::Value, fallback_name: &str) -> String {
    let name = theme
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or(fallback_name);

    let color_scheme = theme
        .get("colorScheme")
        .and_then(|v| v.as_str())
        .unwrap_or("dark");

    let mut css_vars = Vec::new();

    // Extract colors
    if let Some(colors) = theme.get("colors").and_then(|c| c.as_object()) {
        for (key, value) in colors {
            if let Some(color_value) = value.as_str() {
                let css_key = camel_to_kebab(key);
                css_vars.push(format!("  --fx-color-{}: {};", css_key, color_value));
            }
        }
    }

    // Extract radius
    if let Some(radius) = theme.get("radius").and_then(|r| r.as_object()) {
        for (key, value) in radius {
            if let Some(radius_value) = value.as_str() {
                let css_key = camel_to_kebab(key);
                css_vars.push(format!("  --fx-radius-{}: {};", css_key, radius_value));
            }
        }
    }

    // Extract effects
    if let Some(effects) = theme.get("effects").and_then(|e| e.as_object()) {
        for (key, value) in effects {
            let css_value = match value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                _ => continue,
            };
            let css_key = camel_to_kebab(key);
            css_vars.push(format!("  --fx-effect-{}: {};", css_key, css_value));
        }
    }

    // Extract spacing
    if let Some(spacing) = theme.get("spacing").and_then(|s| s.as_object()) {
        for (key, value) in spacing {
            if let Some(spacing_value) = value.as_str() {
                let css_key = camel_to_kebab(key);
                css_vars.push(format!("  --fx-spacing-{}: {};", css_key, spacing_value));
            }
        }
    }

    format!(
        r#"[data-theme="{}"] {{
  color-scheme: {};
{}
}}"#,
        name,
        color_scheme,
        css_vars.join("\n")
    )
}

/// Convert camelCase to kebab-case.
fn camel_to_kebab(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('-');
            }
            if let Some(lower) = c.to_lowercase().next() {
                result.push(lower);
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Compile and minify component CSS for each design system.
fn compile_component_css(base_dir: &Path, output_dir: &Path) {
    if !base_dir.exists() {
        eprintln!("Warning: base CSS directory does not exist: {:?}", base_dir);
        return;
    }

    for design_system in WalkDir::new(base_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        let system_name = match design_system.file_name().to_str() {
            Some(name) => name,
            None => continue,
        };
        let system_output_dir = output_dir.join(system_name);
        let _ = fs::create_dir_all(&system_output_dir);

        // Collect all CSS files in this design system
        // Skip files starting with underscore (like _index.css) as they are entry points
        let mut css_files: Vec<_> = WalkDir::new(design_system.path())
            .min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                let is_css = e.path()
                    .extension()
                    .map(|ext| ext == "css")
                    .unwrap_or(false);
                let is_partial = e.path()
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.starts_with('_'))
                    .unwrap_or(false);
                is_css && !is_partial
            })
            .map(|e| e.path().to_path_buf())
            .collect();

        // Sort by filename to ensure consistent ordering (00-base.css comes first)
        css_files.sort_by(|a, b| {
            a.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .cmp(b.file_name().and_then(|s| s.to_str()).unwrap_or(""))
        });

        let mut combined_css = String::new();
        for path in css_files {
            if let Ok(css_content) = fs::read_to_string(&path) {
                combined_css.push_str(&css_content);
                combined_css.push('\n');
            }
        }

        if !combined_css.is_empty() {
            let minified = minify_css(&combined_css);
            let output_path = system_output_dir.join("components.css");
            if let Err(e) = fs::write(&output_path, minified) {
                eprintln!("Warning: Failed to write component CSS: {}", e);
            } else {
                println!(
                    "cargo:warning=Compiled component CSS for: {}",
                    system_name
                );
            }
        }
    }
}

/// Generate a manifest file listing all available themes.
fn generate_themes_manifest(presets_dir: &Path, output_dir: &Path) {
    let mut manifest: HashMap<String, Vec<String>> = HashMap::new();

    if presets_dir.exists() {
        for design_system in WalkDir::new(presets_dir)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
        {
            let system_name = match design_system.file_name().to_str() {
                Some(name) => name.to_string(),
                None => continue,
            };
            let mut themes = Vec::new();

            for entry in WalkDir::new(design_system.path())
                .min_depth(1)
                .max_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .map(|ext| ext == "json")
                        .unwrap_or(false)
                })
            {
                if let Some(theme_name) = entry.path().file_stem().and_then(|s| s.to_str()) {
                    themes.push(theme_name.to_string());
                }
            }

            if !themes.is_empty() {
                themes.sort();
                manifest.insert(system_name, themes);
            }
        }
    }

    // Write manifest as JSON
    if let Ok(manifest_json) = serde_json::to_string_pretty(&manifest) {
        let _ = fs::write(output_dir.join("manifest.json"), manifest_json);
    }
}

/// Minify CSS using lightningcss.
fn minify_css(css: &str) -> String {
    match StyleSheet::parse(css, ParserOptions::default()) {
        Ok(stylesheet) => {
            match stylesheet.to_css(PrinterOptions {
                minify: true,
                ..Default::default()
            }) {
                Ok(output) => output.code,
                Err(_) => css.to_string(),
            }
        }
        Err(_) => css.to_string(),
    }
}
