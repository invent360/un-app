//! Device earnings comparison component

use leptos::prelude::*;

/// Device type for earnings comparison
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceEarnings {
    pub device_type: String,
    pub icon: &'static str,
    pub min_monthly: f64,
    pub max_monthly: f64,
    pub description: String,
}

impl DeviceEarnings {
    pub fn min_display(&self) -> String {
        format!("${:.2}", self.min_monthly)
    }

    pub fn max_display(&self) -> String {
        format!("${:.2}", self.max_monthly)
    }
}

/// Device comparison component showing side-by-side earnings
#[component]
pub fn DeviceComparison(
    /// Devices to compare - must be provided from CMS
    devices: Vec<DeviceEarnings>,
) -> impl IntoView {
    view! {
        <div class="device-comparison">
            <h2>"Device Earnings Comparison"</h2>

            <div class="comparison-cards">
                {devices.iter().map(|device| {
                    let device = device.clone();
                    view! {
                        <div class="device-card">
                            <div class="device-icon">{device.icon}</div>
                            <h3 class="device-name">{device.device_type.clone()}</h3>
                            <p class="device-description">{device.description.clone()}</p>
                            <div class="device-earnings">
                                <div class="earnings-range">
                                    <span class="min">{device.min_display()}</span>
                                    <span class="separator">" - "</span>
                                    <span class="max">{device.max_display()}</span>
                                </div>
                                <span class="period">"/month"</span>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>

            <table class="comparison-table">
                <thead>
                    <tr>
                        <th>"Device Type"</th>
                        <th>"Min/Month"</th>
                        <th>"Max/Month"</th>
                    </tr>
                </thead>
                <tbody>
                    {devices.iter().map(|device| {
                        let device = device.clone();
                        view! {
                            <tr>
                                <td>
                                    <span class="device-icon-small">{device.icon}</span>
                                    {device.device_type.clone()}
                                </td>
                                <td>{device.min_display()}</td>
                                <td>{device.max_display()}</td>
                            </tr>
                        }
                    }).collect_view()}
                </tbody>
            </table>
        </div>
    }
}

