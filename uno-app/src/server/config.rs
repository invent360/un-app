//! Server configuration utilities
//!
//! R5-11: Provides unified production-mode detection across the application.

use std::sync::OnceLock;

/// Cached production mode result (computed once at startup)
static IS_PRODUCTION: OnceLock<bool> = OnceLock::new();

/// R5-11: Unified production-mode resolver
///
/// Checks multiple environment variables in priority order:
/// 1. RUST_ENV == "production" (case-insensitive)
/// 2. LEPTOS_ENV == "PROD" or "production" (case-insensitive)
/// 3. PRODUCTION == "true" or "1" or "production"
/// 4. APP_ENV == "production"
///
/// The result is cached after first call for consistency within a process.
pub fn is_production_mode() -> bool {
    *IS_PRODUCTION.get_or_init(|| {
        // Priority 1: RUST_ENV (standard Rust convention)
        if let Ok(value) = std::env::var("RUST_ENV") {
            if value.eq_ignore_ascii_case("production") {
                return true;
            }
        }

        // Priority 2: LEPTOS_ENV (Leptos framework convention)
        if let Ok(value) = std::env::var("LEPTOS_ENV") {
            if value.eq_ignore_ascii_case("prod") || value.eq_ignore_ascii_case("production") {
                return true;
            }
        }

        // Priority 3: PRODUCTION (explicit flag)
        if let Ok(value) = std::env::var("PRODUCTION") {
            let v = value.to_lowercase();
            if v == "true" || v == "1" || v == "production" {
                return true;
            }
        }

        // Priority 4: APP_ENV (general app convention)
        if let Ok(value) = std::env::var("APP_ENV") {
            if value.eq_ignore_ascii_case("production") {
                return true;
            }
        }

        false
    })
}

/// R5-11: Safe write-test using temporary file
///
/// Creates a temporary file with random name under the given directory,
/// writes test data, and cleans up automatically.
/// Returns Ok(()) if write succeeds, Err with message otherwise.
pub fn verify_writable(path: &std::path::Path) -> Result<(), String> {
    use std::io::Write;
    use uuid::Uuid;

    // Generate random filename to avoid collisions
    let temp_name = format!(".tmp-write-test-{}", Uuid::new_v4());
    let temp_path = path.join(&temp_name);

    // Attempt write
    let mut file = std::fs::File::create(&temp_path)
        .map_err(|e| format!("Cannot create test file: {}", e))?;

    file.write_all(b"write-test")
        .map_err(|e| format!("Cannot write to test file: {}", e))?;

    file.sync_all()
        .map_err(|e| format!("Cannot sync test file: {}", e))?;

    drop(file);

    // Cleanup (best effort)
    if let Err(e) = std::fs::remove_file(&temp_path) {
        tracing::warn!(path = %temp_path.display(), error = %e, "Failed to cleanup write-test file");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_writable_success() {
        let temp_dir = std::env::temp_dir();
        assert!(verify_writable(&temp_dir).is_ok());
    }

    #[test]
    fn test_verify_writable_nonexistent() {
        let bad_path = std::path::Path::new("/nonexistent/path/that/does/not/exist");
        assert!(verify_writable(bad_path).is_err());
    }
}
