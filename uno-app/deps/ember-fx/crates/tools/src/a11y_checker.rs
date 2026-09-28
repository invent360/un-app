//! Accessibility checker for auditing component accessibility.

use leptos::prelude::*;

/// Accessibility issue severity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A11ySeverity {
    /// Critical accessibility violation.
    Error,
    /// Potential accessibility issue.
    Warning,
    /// Suggestion for improvement.
    Info,
}

/// An accessibility issue found during checking.
#[derive(Clone, Debug)]
pub struct A11yIssue {
    /// Severity of the issue.
    pub severity: A11ySeverity,
    /// Description of the issue.
    pub message: String,
    /// Element selector or description.
    pub element: String,
    /// WCAG criterion if applicable.
    pub wcag: Option<String>,
}

/// Accessibility checker component.
///
/// Scans the page for common accessibility issues and displays a report.
///
/// # Example
///
/// ```ignore
/// use ember_fx_tools::A11yChecker;
///
/// view! {
///     <A11yChecker />
/// }
/// ```
#[component]
pub fn A11yChecker(
    /// Whether the checker panel is initially open.
    #[prop(optional)]
    initial_open: Option<bool>,
) -> impl IntoView {
    let is_open = RwSignal::new(initial_open.unwrap_or(false));
    let issues = RwSignal::new(Vec::<A11yIssue>::new());
    let is_scanning = RwSignal::new(false);

    let run_check = move |_| {
        is_scanning.set(true);
        let mut found_issues = Vec::new();

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;

            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                // Check for images without alt text
                if let Ok(images) = document.query_selector_all("img:not([alt])") {
                    let count = images.length();
                    if count > 0 {
                        found_issues.push(A11yIssue {
                            severity: A11ySeverity::Error,
                            message: format!("{} image(s) missing alt attribute", count),
                            element: "img".to_string(),
                            wcag: Some("1.1.1".to_string()),
                        });
                    }
                }

                // Check for buttons without accessible names
                if let Ok(buttons) = document.query_selector_all("button:empty:not([aria-label])") {
                    let count = buttons.length();
                    if count > 0 {
                        found_issues.push(A11yIssue {
                            severity: A11ySeverity::Error,
                            message: format!("{} button(s) without accessible name", count),
                            element: "button".to_string(),
                            wcag: Some("4.1.2".to_string()),
                        });
                    }
                }

                // Check for links without href
                if let Ok(links) = document.query_selector_all("a:not([href])") {
                    let count = links.length();
                    if count > 0 {
                        found_issues.push(A11yIssue {
                            severity: A11ySeverity::Warning,
                            message: format!("{} link(s) without href attribute", count),
                            element: "a".to_string(),
                            wcag: Some("2.4.4".to_string()),
                        });
                    }
                }

                // Check for missing form labels
                if let Ok(inputs) = document.query_selector_all("input:not([type='hidden']):not([aria-label]):not([aria-labelledby])") {
                    let mut unlabeled = 0u32;
                    for i in 0..inputs.length() {
                        if let Some(input) = inputs.item(i) {
                            if let Some(element) = input.dyn_ref::<web_sys::Element>() {
                                let id = element.get_attribute("id").unwrap_or_default();
                                if id.is_empty() {
                                    unlabeled += 1;
                                } else if document.query_selector(&format!("label[for='{}']", id)).ok().flatten().is_none() {
                                    unlabeled += 1;
                                }
                            }
                        }
                    }
                    if unlabeled > 0 {
                        found_issues.push(A11yIssue {
                            severity: A11ySeverity::Error,
                            message: format!("{} input(s) without associated label", unlabeled),
                            element: "input".to_string(),
                            wcag: Some("1.3.1".to_string()),
                        });
                    }
                }

                // Check for missing document language
                if let Some(html) = document.document_element() {
                    if html.get_attribute("lang").map(|l| l.is_empty()).unwrap_or(true) {
                        found_issues.push(A11yIssue {
                            severity: A11ySeverity::Error,
                            message: "Document missing lang attribute".to_string(),
                            element: "html".to_string(),
                            wcag: Some("3.1.1".to_string()),
                        });
                    }
                }

                // Check for missing main landmark
                if document.query_selector("main, [role='main']").ok().flatten().is_none() {
                    found_issues.push(A11yIssue {
                        severity: A11ySeverity::Warning,
                        message: "Page missing main landmark".to_string(),
                        element: "document".to_string(),
                        wcag: Some("1.3.1".to_string()),
                    });
                }

                // Check for positive tabindex
                if let Ok(elements) = document.query_selector_all("[tabindex]") {
                    let mut positive_count = 0u32;
                    for i in 0..elements.length() {
                        if let Some(el) = elements.item(i) {
                            if let Some(element) = el.dyn_ref::<web_sys::Element>() {
                                if let Some(tabindex) = element.get_attribute("tabindex") {
                                    if let Ok(val) = tabindex.parse::<i32>() {
                                        if val > 0 {
                                            positive_count += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if positive_count > 0 {
                        found_issues.push(A11yIssue {
                            severity: A11ySeverity::Warning,
                            message: format!("{} element(s) with positive tabindex", positive_count),
                            element: "[tabindex]".to_string(),
                            wcag: Some("2.4.3".to_string()),
                        });
                    }
                }
            }
        }

        if found_issues.is_empty() {
            found_issues.push(A11yIssue {
                severity: A11ySeverity::Info,
                message: "No accessibility issues found!".to_string(),
                element: "document".to_string(),
                wcag: None,
            });
        }

        issues.set(found_issues);
        is_scanning.set(false);
    };

    let error_count = move || issues.get().iter().filter(|i| i.severity == A11ySeverity::Error).count();
    let warning_count = move || issues.get().iter().filter(|i| i.severity == A11ySeverity::Warning).count();

    view! {
        <div
            class="fx-a11y-checker"
            style="position: fixed; bottom: 16px; left: 16px; z-index: 9999; font-family: monospace; font-size: 12px;"
        >
            <button
                type="button"
                style="padding: 8px 12px; background: #1a1a2e; color: #eee; border: 1px solid #333; border-radius: 4px; cursor: pointer;"
                on:click=move |_| is_open.update(|v| *v = !*v)
            >
                {move || if is_open.get() { "Close A11y" } else { "A11y Check" }}
            </button>

            <Show when=move || is_open.get()>
                <div
                    style="margin-top: 8px; padding: 16px; background: #1a1a2e; border: 1px solid #333; border-radius: 8px; width: 350px; max-height: 400px; overflow: hidden; display: flex; flex-direction: column; color: #eee;"
                >
                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                        <h3 style="margin: 0; font-size: 14px; font-weight: 600;">
                            "Accessibility Check"
                        </h3>
                        <button
                            type="button"
                            style="padding: 4px 12px; background: #2e7d32; color: #fff; border: none; border-radius: 4px; cursor: pointer;"
                            on:click=run_check
                            disabled=move || is_scanning.get()
                        >
                            {move || if is_scanning.get() { "Scanning..." } else { "Run Check" }}
                        </button>
                    </div>

                    <Show when=move || !issues.get().is_empty()>
                        <div style="display: flex; gap: 12px; margin-bottom: 12px; font-size: 11px;">
                            <span style="color: #f44336;">
                                {move || format!("{} errors", error_count())}
                            </span>
                            <span style="color: #ff9800;">
                                {move || format!("{} warnings", warning_count())}
                            </span>
                        </div>
                    </Show>

                    <div style="overflow-y: auto; flex: 1;">
                        {move || {
                            issues.get().into_iter().map(|issue| {
                                let (color, icon) = match issue.severity {
                                    A11ySeverity::Error => ("#f44336", "X"),
                                    A11ySeverity::Warning => ("#ff9800", "!"),
                                    A11ySeverity::Info => ("#4caf50", "i"),
                                };

                                view! {
                                    <div style="padding: 8px; margin-bottom: 8px; background: #2a2a3e; border-radius: 4px; border-left: 3px solid; border-left-color: {color};">
                                        <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 4px;">
                                            <span style=format!(
                                                "display: inline-flex; align-items: center; justify-content: center; width: 16px; height: 16px; border-radius: 50%; background: {}; color: #fff; font-size: 10px; font-weight: bold;",
                                                color
                                            )>
                                                {icon}
                                            </span>
                                            <span style="color: #eee; font-weight: 500;">
                                                {issue.message.clone()}
                                            </span>
                                        </div>
                                        <div style="display: flex; gap: 12px; font-size: 10px; color: #888;">
                                            <span>"Element: "{issue.element}</span>
                                            {issue.wcag.map(|wcag| view! {
                                                <span>"WCAG: "{wcag}</span>
                                            })}
                                        </div>
                                    </div>
                                }
                            }).collect_view()
                        }}
                    </div>
                </div>
            </Show>
        </div>
    }
}
