//! Visitor statistics by country component

use leptos::prelude::*;

/// Visitor data by country
#[derive(Debug, Clone)]
pub struct CountryVisitors {
    pub country_code: String,
    pub country_name: String,
    pub visitor_count: u32,
    pub percentage: f64,
}

/// Visitor map/list component
#[component]
pub fn VisitorMap(
    countries: Vec<CountryVisitors>,
    #[prop(optional)] title: Option<String>,
) -> impl IntoView {
    let total: u32 = countries.iter().map(|c| c.visitor_count).sum();
    let max_visitors = countries.iter().map(|c| c.visitor_count).max().unwrap_or(1);

    view! {
        <div class="visitor-map">
            {title.map(|t| view! { <h3 class="chart-title">{t}</h3> })}

            <div class="visitor-list">
                {countries.into_iter().map(|country| {
                    let bar_width = (country.visitor_count as f64 / max_visitors as f64) * 100.0;

                    view! {
                        <div class="visitor-row">
                            <div class="country-info">
                                <span class="country-flag">{get_flag_emoji(&country.country_code)}</span>
                                <span class="country-name">{country.country_name.clone()}</span>
                            </div>
                            <div class="visitor-bar-container">
                                <div
                                    class="visitor-bar"
                                    style=format!("width: {}%", bar_width)
                                />
                            </div>
                            <div class="visitor-stats">
                                <span class="visitor-count">{country.visitor_count}</span>
                                <span class="visitor-percentage">
                                    {format!("{}%", country.percentage as i32)}
                                </span>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>

            <div class="visitor-total">
                <span class="total-label">"Total Visitors:"</span>
                <span class="total-value">{total}</span>
            </div>
        </div>
    }
}

/// Get flag emoji for country code
fn get_flag_emoji(country_code: &str) -> String {
    // Convert country code to regional indicator symbols
    country_code
        .to_uppercase()
        .chars()
        .map(|c| {
            char::from_u32(0x1F1E6 - 'A' as u32 + c as u32)
                .unwrap_or(c)
        })
        .collect()
}
