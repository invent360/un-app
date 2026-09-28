//! Guides page - displays guides from CMS with TaskCard-style layout and GuideStepper modal

use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use crate::components::common::{Button, ButtonShape, PreviewBanner};

#[cfg(feature = "hydrate")]
use crate::hooks::{t, use_locale};
#[cfg(feature = "hydrate")]
use crate::api::{get_guides_with_preview, GuidesPageResponse, GuideSection, Stage};

#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
use ember_fx_icons::tabler::{TablerIconData, SystemIcon};

#[cfg(feature = "hydrate")]
use ember_fx_components::{GuideStepper, GuideStep, GuideStepperOrientation as StepperOrientation, ImagePosition, StepLayout, ControlsPosition, StepperHeaderLayout};

/// Pre-requisite guide info for navigation
#[cfg(feature = "hydrate")]
#[derive(Clone, Debug)]
struct PrerequisiteInfo {
    title: String,
    index: usize,
}

/// Convert API stages to ember-fx GuideStep format
#[cfg(feature = "hydrate")]
fn convert_stages(stages: &[Stage]) -> Vec<GuideStep> {
    stages.iter().map(|s| {
        let mut step = GuideStep::new(
            s.order.to_string(),
            s.title.clone(),
            s.description.clone(),
        ).order(s.order as i32);
        // Use first image if available
        if let Some(img) = s.images.first() {
            step = step.image(img.clone());
        }
        step
    }).collect()
}

/// Guide stepper modal component
#[cfg(feature = "hydrate")]
#[component]
fn GuideModal(
    /// Whether the modal is visible
    visible: Memo<bool>,
    /// Guide title
    title: String,
    /// Guide slug (used to determine layout)
    slug: String,
    /// Guide stages
    stages: Vec<Stage>,
    /// Pre-requisite guides
    prerequisites: Vec<PrerequisiteInfo>,
    /// Signal to control which modal is open
    open_modal_signal: RwSignal<Option<usize>>,
) -> impl IntoView {
    let active_step = RwSignal::new(0usize);

    // Track whether we're showing prerequisites or the actual guide
    let has_prereqs = !prerequisites.is_empty();
    let showing_prereqs = RwSignal::new(has_prereqs);

    // Reset showing_prereqs when modal becomes visible (for re-opening)
    Effect::new(move || {
        if visible.get() {
            showing_prereqs.set(has_prereqs);
            active_step.set(0);
        }
    });

    let stepper_steps = convert_stages(&stages);
    let title_clone = title.clone();

    // Store non-Copy values for reactive closure
    let prereqs_stored = StoredValue::new(prerequisites);
    let slug_stored = StoredValue::new(slug);
    let stepper_steps_stored = StoredValue::new(stepper_steps);

    view! {
        <Show when=move || visible.get()>
            <div
                class="guide-modal-backdrop"
                on:click=move |_| open_modal_signal.set(None)
            >
                <div
                    class="guide-modal"
                    on:click=move |e| e.stop_propagation()
                >
                    // Modal header
                    <div class="guide-modal-header">
                        <h2 class="guide-modal-title">{title_clone.clone()}</h2>
                        <button
                            type="button"
                            class="guide-modal-close"
                            on:click=move |_| open_modal_signal.set(None)
                        >
                            "×"
                        </button>
                    </div>

                    // Modal body - either prerequisites or stepper
                    <div class="guide-modal-body">
                        {move || {
                            if showing_prereqs.get() {
                                // Prerequisites screen
                                let prereqs_clone = prereqs_stored.get_value();
                                view! {
                                    <div class="guide-prereqs">
                                        <div class="guide-prereqs-icon">
                                            <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                <path d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
                                            </svg>
                                        </div>
                                        <h3 class="guide-prereqs-title">{t("guides.before_you_begin")}</h3>
                                        <p class="guide-prereqs-subtitle">{t("guides.complete_these_first")}</p>
                                        <div class="guide-prereqs-list">
                                            {prereqs_clone.iter().map(|prereq| {
                                                let idx = prereq.index;
                                                let prereq_title = prereq.title.clone();
                                                view! {
                                                    <button
                                                        type="button"
                                                        class="guide-prereq-button"
                                                        on:click=move |_| open_modal_signal.set(Some(idx))
                                                    >
                                                        <span class="guide-prereq-icon">
                                                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                                <path d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"/>
                                                            </svg>
                                                        </span>
                                                        <span class="guide-prereq-title">{prereq_title}</span>
                                                        <span class="guide-prereq-arrow">
                                                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                                <path d="M9 18l6-6-6-6"/>
                                                            </svg>
                                                        </span>
                                                    </button>
                                                }
                                            }).collect_view()}
                                        </div>
                                        <button
                                            type="button"
                                            class="guide-continue-button"
                                            on:click=move |_| showing_prereqs.set(false)
                                        >
                                            {t("guides.continue_to_guide")}
                                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                <path d="M9 18l6-6-6-6"/>
                                            </svg>
                                        </button>
                                    </div>
                                }.into_any()
                            } else {
                                // Normal stepper view
                                let current_slug = slug_stored.get_value();
                                let current_steps = stepper_steps_stored.get_value();
                                view! {
                                    <div class="guide-stepper-container">
                                        {
                                            // Determine layout based on guide slug
                                            let layout = if current_slug.contains("incentive-withdrawal") {
                                                StepLayout::Stacked
                                            } else {
                                                StepLayout::SideBySide
                                            };

                                            if !current_steps.is_empty() {
                                                let on_complete = move |_| {
                                                    open_modal_signal.set(None);
                                                };

                                                // Use top-right controls for side-by-side layout to allow larger images
                                                let controls_pos = if layout == StepLayout::SideBySide {
                                                    ControlsPosition::TopRight
                                                } else {
                                                    ControlsPosition::Bottom
                                                };

                                                // Use Split header layout to separate steps (left) from controls (right)
                                                let header_layout = if layout == StepLayout::SideBySide {
                                                    StepperHeaderLayout::Split
                                                } else {
                                                    StepperHeaderLayout::Standard
                                                };

                                                view! {
                                                    <GuideStepper
                                                        steps=current_steps.clone()
                                                        active_step=active_step
                                                        orientation=StepperOrientation::Horizontal
                                                        clickable=true
                                                        show_controls=true
                                                        show_titles=false
                                                        image_position=ImagePosition::Left
                                                        layout=layout
                                                        controls_position=controls_pos
                                                        header_layout=header_layout
                                                        max_visible_steps=10
                                                        on_complete=on_complete
                                                    />
                                                }.into_any()
                                            } else {
                                                view! { <p class="guide-modal-empty">"No stages available"</p> }.into_any()
                                            }
                                        }
                                    </div>
                                }.into_any()
                            }
                        }}
                    </div>
                </div>
            </div>
        </Show>
    }
}

/// Single guide card component with modal
#[cfg(feature = "hydrate")]
#[component]
fn GuideCard(
    /// The guide section data
    section: GuideSection,
    /// Index of this section in the sections array
    section_index: usize,
    /// All sections (for resolving prerequisites)
    all_sections: Vec<GuideSection>,
    /// Shared signal to track which modal is open
    open_modal_index: RwSignal<Option<usize>>,
) -> impl IntoView {
    // Modal visibility derived from shared signal
    let show_modal = Memo::new(move |_| open_modal_index.get() == Some(section_index));

    // Use section-level title and description
    let title = section.title.clone();
    let description = section.description.clone();

    // Use first cover image as thumbnail
    let thumbnail = section.cover_images.first().cloned();

    // Get stages from content
    let stages = section.content.stages.clone();
    let stage_count = stages.len();

    // Truncated description for card view
    let truncated_desc = if description.len() > 120 {
        format!("{}...", &description[..120])
    } else {
        description.clone()
    };

    let title_clone = title.clone();
    let title_for_modal = title.clone();
    let stages_for_modal = stages.clone();

    // Resolve prerequisites from indices to actual section info
    leptos::logging::log!("GuideCard '{}': pre_steps = {:?}, all_sections count = {}",
        section.title, section.pre_steps, all_sections.len());
    let prereqs: Vec<PrerequisiteInfo> = section.pre_steps.iter()
        .filter_map(|&idx| all_sections.get(idx).map(|s| PrerequisiteInfo {
            title: s.title.clone(),
            index: idx,
        }))
        .collect();
    leptos::logging::log!("GuideCard '{}': resolved {} prerequisites", section.title, prereqs.len());

    // Plus icon for More button
    #[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
    let icon_plus = SystemIcon::Plus.outline_svg();
    #[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
    let icon_plus = "+".to_string();

    // Clone values for rendering
    let has_thumbnail = thumbnail.is_some();
    let thumbnail_url = thumbnail.unwrap_or_default();
    let has_stages = stage_count > 0;
    let stages_text = format!("{} stages", stage_count);

    // New fields
    let complexity = section.complexity.clone();
    let estimated_time = section.estimated_time.clone();
    let has_complexity = !complexity.is_empty();
    let has_time = !estimated_time.is_empty();
    let complexity_display = if complexity == "medium" { "MEDIUM" } else { "EASY" };
    let time_display = format!("{} Mins", estimated_time);

    view! {
        <>
            <article class="guide-card">
                // Header section with title
                <div class="guide-card-header">
                    <h3 class="guide-card-title">{title.clone()}</h3>
                </div>

                // Body section with background image
                <div class="guide-card-body">
                    // Background thumbnail
                    <div class=if has_thumbnail { "guide-card-media" } else { "guide-card-media guide-card-media-hidden" }>
                        <img
                            src=thumbnail_url.clone()
                            alt=title_clone.clone()
                            loading="lazy"
                        />
                    </div>

                    // Content overlay
                    <div class="guide-card-content">
                        // Description
                        <p class="guide-card-description">{truncated_desc}</p>

                        // Meta info
                        <div class="guide-card-meta">
                            <span class=if has_stages { "guide-card-steps" } else { "guide-card-steps guide-hidden" }>
                                {stages_text.clone()}
                            </span>

                            // Complexity and Estimated Time badges
                            <div class="guide-card-badges">
                                {has_complexity.then(|| view! {
                                    <span class=format!("guide-badge guide-badge-complexity guide-badge-{}", complexity.clone())>
                                        {complexity_display}
                                    </span>
                                })}
                                {has_time.then(|| view! {
                                    <span class="guide-badge guide-badge-time">
                                        {time_display.clone()}
                                    </span>
                                })}
                            </div>

                            // More button
                            <Button
                                shape=ButtonShape::Round
                                icon=icon_plus
                                on_click=Callback::new(move |_| open_modal_index.set(Some(section_index)))
                            >
                                {move || t("common.more")}
                            </Button>
                        </div>
                    </div>
                </div>
            </article>

            // Modal with stepper
            <GuideModal
                visible=show_modal
                title=title_for_modal
                slug=section.slug.clone()
                stages=stages_for_modal
                prerequisites=prereqs
                open_modal_signal=open_modal_index
            />
        </>
    }
}

/// Default empty response for error cases
#[cfg(feature = "hydrate")]
fn default_guides_response() -> GuidesPageResponse {
    GuidesPageResponse {
        name: "Guides".to_string(),
        description: String::new(),
        content_type: "guide".to_string(),
        sections: vec![],
        is_preview: false,
    }
}

/// Client-only guides content component - only rendered after hydration
#[cfg(feature = "hydrate")]
#[component]
fn GuidesContent(
    /// Preview token from query string
    preview_token: Option<String>,
    /// Callback to set preview mode state
    set_is_preview: WriteSignal<bool>,
) -> impl IntoView {
    let ctx = use_locale();
    let token = preview_token.clone();

    let guides_resource = Resource::new(
        move || (ctx.locale.get().code().to_string(), token.clone()),
        |(locale, token)| async move {
            match get_guides_with_preview(locale, token).await {
                Ok(response) => {
                    leptos::logging::log!("Guides API success: name='{}', sections={}", response.name, response.sections.len());
                    response
                }
                Err(e) => {
                    leptos::logging::error!("Guides API error: {:?}", e);
                    default_guides_response()
                }
            }
        }
    );

    // Update preview state when resource loads
    Effect::new(move || {
        if let Some(response) = guides_resource.get() {
            set_is_preview.set(response.is_preview);
        }
    });

    view! {
        <Suspense fallback=move || view! {
            // Loading state - no header, just spinner
            <section class="guides-content">
                <div class="guides-content-wrapper">
                    <div class="loading-state" style="text-align: center; padding: 3rem;">
                        <div class="spinner" style="width: 40px; height: 40px; border: 3px solid #e2e8f0; border-top-color: #3b82f6; border-radius: 50%; animation: spin 1s linear infinite; margin: 0 auto;"></div>
                    </div>
                    <style>{"@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }"}</style>
                </div>
            </section>
        }>
            {move || {
                // Check if resource is still loading
                let Some(response) = guides_resource.get() else {
                    // Still loading - show spinner only
                    return view! {
                        <section class="guides-content">
                            <div class="guides-content-wrapper">
                                <div class="loading-state" style="text-align: center; padding: 3rem;">
                                    <div class="spinner" style="width: 40px; height: 40px; border: 3px solid #e2e8f0; border-top-color: #3b82f6; border-radius: 50%; animation: spin 1s linear infinite; margin: 0 auto;"></div>
                                </div>
                                <style>{"@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }"}</style>
                            </div>
                        </section>
                    }.into_any();
                };

                // Show empty state if no guides from CMS - only "Coming Soon", no header
                if response.sections.is_empty() {
                    return view! {
                        <section class="guides-content">
                            <div class="guides-content-wrapper">
                                <div class="empty-state" style="text-align: center; padding: 3rem;">
                                    <p class="placeholder-text">
                                        {move || t("guides.coming_soon")}
                                    </p>
                                </div>
                            </div>
                        </section>
                    }.into_any();
                }

                let page_name = response.name.clone();
                let page_description = response.description.clone();

                // Shared signal to track which modal is open (by section index)
                let open_modal_index: RwSignal<Option<usize>> = RwSignal::new(None);

                // Clone sections for passing to each card
                let sections_for_cards = response.sections.clone();

                view! {
                    // Page header from API response - only shown when there's content
                    <header class="page-header">
                        <h1>{page_name}</h1>
                        <p>{page_description}</p>
                    </header>

                    <section class="guides-content">
                        <div class="guides-content-wrapper">
                            <div class="guides-grid">
                                {response.sections.into_iter().enumerate().map(|(idx, section)| {
                                    let all_secs = sections_for_cards.clone();
                                    view! {
                                        <GuideCard
                                            section=section
                                            section_index=idx
                                            all_sections=all_secs
                                            open_modal_index=open_modal_index
                                        />
                                    }
                                }).collect_view()}
                            </div>

                            // Tip section
                            <div class="guides-tip">
                                <span class="tip-label">{move || t("guides.tip_label")}</span>
                                <span class="tip_text">{move || t("guides.tip_text")}</span>
                            </div>
                        </div>
                    </section>
                }.into_any()
            }}
        </Suspense>
    }
}

/// Guides content view - uses client-only pattern to avoid hydration mismatch
fn guides_content_view(
    preview_token: Option<String>,
    set_is_preview: WriteSignal<bool>,
) -> impl IntoView {
    // Signal to track if we're mounted on client
    let is_mounted = RwSignal::new(false);

    // Effect runs only on client after hydration
    #[cfg(feature = "hydrate")]
    {
        Effect::new(move || {
            is_mounted.set(true);
        });
    }

    view! {
        // Container div - consistent between SSR and client
        <div class="guides-dynamic-container">
            {move || {
                if is_mounted.get() {
                    // Client-only: render the actual guides grid
                    #[cfg(feature = "hydrate")]
                    {
                        view! { <GuidesContent preview_token=preview_token.clone() set_is_preview=set_is_preview /> }.into_any()
                    }
                    #[cfg(not(feature = "hydrate"))]
                    {
                        view! {}.into_any()
                    }
                } else {
                    // SSR and initial client render: show loading state with spinner
                    view! {
                        <div class="loading-state" style="text-align: center; padding: 3rem;">
                            <div class="spinner" style="width: 40px; height: 40px; border: 3px solid #e2e8f0; border-top-color: #3b82f6; border-radius: 50%; animation: spin 1s linear infinite; margin: 0 auto;"></div>
                        </div>
                        <style>{"@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }"}</style>
                    }.into_any()
                }
            }}
        </div>
    }
}

/// Guides page component
#[component]
pub fn GuidesPage() -> impl IntoView {
    let query = use_query_map();

    // Extract preview_token from query string - use get_untracked for initial value
    let preview_token = query.get_untracked().get("preview_token").map(|s| s.to_string());

    // Track if we're in preview mode
    let (is_preview, set_is_preview) = signal(preview_token.is_some());

    view! {
        <div class="page guides-page">
            // Show preview banner if in preview mode
            {move || is_preview.get().then(|| view! { <PreviewBanner /> })}

            // Page header and content come from API via GuidesContent
            {guides_content_view(preview_token.clone(), set_is_preview)}
        </div>
    }
}
