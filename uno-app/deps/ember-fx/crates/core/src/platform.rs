//! Platform detection for Tauri 2.0 WebView environments.
//!
//! All platforms run Leptos/WASM in WebView (Tauri 2.0), so platform
//! detection is performed via user-agent string analysis.

use ember_fx_common::DesignSystem;
#[cfg(target_arch = "wasm32")]
use web_sys::window;

/// Detected platform types.
///
/// All platforms run within Tauri 2.0 WebView, using different browser engines:
/// - iOS: WKWebView (Safari)
/// - Android: Chrome WebView
/// - macOS: WebKit WebView
/// - Windows: WebView2 (Edge/Chromium)
/// - Linux: WebKitGTK
/// - Web: Standard browser (fallback)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Platform {
    /// iOS Safari WebView (WKWebView)
    IOS,
    /// Android Chrome WebView
    Android,
    /// macOS WebKit WebView
    MacOS,
    /// Windows WebView2 (Edge/Chromium)
    Windows,
    /// Linux WebKitGTK
    Linux,
    /// Standard web browser (fallback)
    #[default]
    Web,
}

impl Platform {
    /// Detect the current platform from the WebView user-agent string.
    #[must_use]
    pub fn detect() -> Self {
        if let Some(ua) = get_user_agent() {
            let ua_lower = ua.to_lowercase();

            // iOS WebView (WKWebView) - check before macOS since iPads may contain "Macintosh"
            if ua_lower.contains("iphone") || ua_lower.contains("ipad") || ua_lower.contains("ipod")
            {
                return Self::IOS;
            }

            // Android Chrome WebView
            if ua_lower.contains("android") {
                return Self::Android;
            }

            // macOS (Safari or WebKit WebView)
            if ua_lower.contains("macintosh") || ua_lower.contains("mac os x") {
                return Self::MacOS;
            }

            // Windows (Edge/WebView2)
            if ua_lower.contains("windows") {
                return Self::Windows;
            }

            // Linux (WebKitGTK) - check after Android since Android also contains "linux"
            if ua_lower.contains("linux") {
                return Self::Linux;
            }
        }

        Self::Web
    }

    /// Get the recommended design system for this platform.
    ///
    /// - iOS → Cupertino (Apple HIG)
    /// - Android → Material (Material Design 3)
    /// - Desktop/Web → Ant Design (default)
    #[must_use]
    pub const fn default_design_system(&self) -> DesignSystem {
        match self {
            Self::IOS => DesignSystem::Cupertino,
            Self::Android => DesignSystem::Material,
            Self::MacOS | Self::Windows | Self::Linux => DesignSystem::Ant,
            Self::Web => DesignSystem::Ant,
        }
    }

    /// Check if this is a mobile platform.
    #[must_use]
    pub const fn is_mobile(&self) -> bool {
        matches!(self, Self::IOS | Self::Android)
    }

    /// Check if this is a desktop platform.
    #[must_use]
    pub const fn is_desktop(&self) -> bool {
        matches!(self, Self::MacOS | Self::Windows | Self::Linux)
    }

    /// Get a display name for this platform.
    #[must_use]
    pub const fn display_name(&self) -> &'static str {
        match self {
            Self::IOS => "iOS",
            Self::Android => "Android",
            Self::MacOS => "macOS",
            Self::Windows => "Windows",
            Self::Linux => "Linux",
            Self::Web => "Web",
        }
    }
}

/// Get the user-agent string from the browser/WebView.
/// Returns None during SSR (non-wasm target).
#[must_use]
pub fn get_user_agent() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        window().and_then(|w| w.navigator().user_agent().ok())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

/// Check if the current environment supports touch events.
///
/// This is a more reliable indicator of mobile than user-agent in some cases.
/// Returns false during SSR (non-wasm target).
#[must_use]
pub fn supports_touch() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = window() {
            if let Ok(has_touch) = js_sys::Reflect::has(&window, &"ontouchstart".into()) {
                return has_touch;
            }
        }
        false
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
}

/// Get the device pixel ratio for responsive scaling.
/// Returns 1.0 during SSR (non-wasm target).
#[must_use]
pub fn device_pixel_ratio() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        window().map(|w| w.device_pixel_ratio()).unwrap_or(1.0)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_default() {
        let platform = Platform::default();
        assert_eq!(platform, Platform::Web);
    }

    #[test]
    fn test_mobile_detection() {
        assert!(Platform::IOS.is_mobile());
        assert!(Platform::Android.is_mobile());
        assert!(!Platform::MacOS.is_mobile());
        assert!(!Platform::Windows.is_mobile());
        assert!(!Platform::Web.is_mobile());
    }

    #[test]
    fn test_desktop_detection() {
        assert!(Platform::MacOS.is_desktop());
        assert!(Platform::Windows.is_desktop());
        assert!(Platform::Linux.is_desktop());
        assert!(!Platform::IOS.is_desktop());
        assert!(!Platform::Android.is_desktop());
        assert!(!Platform::Web.is_desktop());
    }

    #[test]
    fn test_default_design_system() {
        assert_eq!(Platform::IOS.default_design_system(), DesignSystem::Cupertino);
        assert_eq!(Platform::Android.default_design_system(), DesignSystem::Material);
        assert_eq!(Platform::MacOS.default_design_system(), DesignSystem::Ant);
        assert_eq!(Platform::Windows.default_design_system(), DesignSystem::Ant);
        assert_eq!(Platform::Web.default_design_system(), DesignSystem::Ant);
    }
}
