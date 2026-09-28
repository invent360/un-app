//! BandwidthMeter Leptos component.
//!
//! A dual progress bar showing upload/download bandwidth utilization.

use leptos::prelude::*;
use crate::try_use_theme;

/// BandwidthMeter component.
///
/// Shows upload and download bandwidth as dual progress bars.
///
/// # Props
///
/// - `upload` - Upload bandwidth in Mbps
/// - `download` - Download bandwidth in Mbps
/// - `max_upload` - Maximum upload bandwidth
/// - `max_download` - Maximum download bandwidth
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::BandwidthMeter;
///
/// view! {
///     <BandwidthMeter
///         upload=Signal::derive(move || 25.5)
///         download=Signal::derive(move || 150.0)
///         max_upload=100.0
///         max_download=500.0
///     />
/// }
/// ```
#[component]
pub fn BandwidthMeter(
    /// Upload bandwidth in Mbps.
    #[prop(into)]
    upload: Signal<f64>,
    /// Download bandwidth in Mbps.
    #[prop(into)]
    download: Signal<f64>,
    /// Maximum upload bandwidth.
    #[prop(optional)]
    max_upload: Option<f64>,
    /// Maximum download bandwidth.
    #[prop(optional)]
    max_download: Option<f64>,
    /// Show values.
    #[prop(optional)]
    show_values: Option<bool>,
    /// Compact mode.
    #[prop(optional)]
    compact: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let max_upload = max_upload.unwrap_or(100.0);
    let max_download = max_download.unwrap_or(100.0);
    let show_values = show_values.unwrap_or(true);
    let compact = compact.unwrap_or(false);

    let prefix = format!("fx-bandwidth-meter-{}", design_system);

    let combined_class = {
        let prefix = prefix.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![prefix.clone()];
            if compact {
                parts.push(format!("{}-compact", prefix));
            }
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    let upload_percent = move || ((upload.get() / max_upload) * 100.0).clamp(0.0, 100.0);
    let download_percent = move || ((download.get() / max_download) * 100.0).clamp(0.0, 100.0);

    let format_mbps = |val: f64| -> String {
        if val >= 1000.0 {
            format!("{:.1} Gbps", val / 1000.0)
        } else {
            format!("{:.1} Mbps", val)
        }
    };

    // Clone prefix for closures
    let upload_prefix = prefix.clone();
    let download_prefix = prefix.clone();

    view! {
        <div class=combined_class>
            // Upload bar
            <div class=format!("{}-row", prefix)>
                <div class=format!("{}-label", prefix)>
                    <span class=format!("{}-icon {}-icon-upload", prefix, prefix)>"↑"</span>
                    {if !compact { Some("Upload") } else { None }}
                </div>
                <div class=format!("{}-bar-container", prefix)>
                    <div
                        class=format!("{}-bar {}-bar-upload", prefix, prefix)
                        style=move || format!("width: {}%;", upload_percent())
                    />
                </div>
                {move || {
                    if show_values {
                        Some(view! {
                            <div class=format!("{}-value", upload_prefix)>
                                {format_mbps(upload.get())}
                            </div>
                        })
                    } else {
                        None
                    }
                }}
            </div>

            // Download bar
            <div class=format!("{}-row", prefix)>
                <div class=format!("{}-label", prefix)>
                    <span class=format!("{}-icon {}-icon-download", prefix, prefix)>"↓"</span>
                    {if !compact { Some("Download") } else { None }}
                </div>
                <div class=format!("{}-bar-container", prefix)>
                    <div
                        class=format!("{}-bar {}-bar-download", prefix, prefix)
                        style=move || format!("width: {}%;", download_percent())
                    />
                </div>
                {move || {
                    if show_values {
                        Some(view! {
                            <div class=format!("{}-value", download_prefix)>
                                {format_mbps(download.get())}
                            </div>
                        })
                    } else {
                        None
                    }
                }}
            </div>
        </div>
    }
}
