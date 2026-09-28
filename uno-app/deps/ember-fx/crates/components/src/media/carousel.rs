//! Carousel component for displaying media galleries.
//!
//! The Carousel component displays a sequence of slides (images or videos)
//! with navigation controls and optional thumbnail preview.
//!
//! # Features
//!
//! - Image and video slide support
//! - Forward/backward navigation arrows
//! - Thumbnail navigation
//! - Pagination dots
//! - Slide info counter (current/total)
//! - Optional title and description
//! - Keyboard navigation
//! - Autoplay with configurable interval
//! - Infinite loop mode
//! - Multiple size variants
//! - Slide and fade transition effects
//! - Multiple visible slides (fixed or responsive)
//! - Centered mode
//! - RTL (right-to-left) support
//! - Auto-height
//! - Drag/swipe support
//! - CSS scroll-snap mode
//! - Loading state animations
//!
//! # Example
//!
//! ```ignore
//! use ember_fx_components::{Carousel, CarouselSlide, SlidesQty};
//! use leptos::prelude::*;
//!
//! let slides = vec![
//!     CarouselSlide::image("1", "/images/photo1.jpg")
//!         .title("First Slide")
//!         .alt("Photo 1"),
//!     CarouselSlide::image("2", "/images/photo2.jpg")
//!         .title("Second Slide"),
//!     CarouselSlide::video("3", "/videos/demo.mp4")
//!         .poster("/images/poster.jpg")
//!         .title("Demo Video"),
//! ];
//!
//! let active = RwSignal::new(0usize);
//!
//! view! {
//!     <Carousel
//!         slides=slides
//!         active_index=active
//!         slides_qty=SlidesQty::Fixed(3)
//!         show_thumbnails=true
//!         show_pagination=true
//!         is_draggable=true
//!         title="Gallery"
//!     />
//! }
//! ```

use leptos::prelude::*;
use super::types::{
    CarouselSlide, CarouselSize, CarouselEffect, CarouselMode,
    ThumbnailPosition, PaginationPosition, SlideContent,
    SlidesQty, LoadingClasses,
};
use crate::try_use_theme;

/// Carousel component for displaying media galleries.
///
/// # Props
///
/// ## Required
/// - `slides` - Vector of carousel slides
/// - `active_index` - Currently active slide index as RwSignal
///
/// ## Display Options
/// - `size` - Carousel size variant (Small, Medium, Large)
/// - `effect` - Transition effect (Slide, Fade)
/// - `mode` - Display mode (Default, ScrollSnap)
///
/// ## Slides Configuration
/// - `slides_qty` - Number of visible slides (fixed or responsive)
/// - `is_centered` - Center the active slide in viewport
/// - `is_auto_height` - Adapt height to content
///
/// ## Navigation Options
/// - `show_arrows` - Show prev/next arrows (default: true)
/// - `show_thumbnails` - Show thumbnail navigation
/// - `show_pagination` - Show pagination dots
/// - `show_slide_info` - Show "1 / 5" counter
/// - `thumbnail_position` - Position of thumbnails
/// - `pagination_position` - Position of pagination dots
///
/// ## Behavior Options
/// - `autoplay` - Autoplay interval in ms (0 = disabled)
/// - `loop_slides` - Enable infinite loop
/// - `is_draggable` - Enable drag/swipe navigation
/// - `is_snap` - Use CSS scroll-snap
/// - `has_snap_spacers` - Add spacers in snap mode (default: true)
/// - `is_rtl` - Right-to-left layout
///
/// ## Loading State
/// - `loading_classes` - CSS classes for loading animations
///
/// ## Header
/// - `title` - Optional carousel title
/// - `description` - Optional carousel description
///
/// ## Customization
/// - `class` - Additional CSS classes
/// - `dots_class` - Custom class for pagination dots
///
/// ## Callbacks
/// - `on_change` - Called when slide changes
#[component]
pub fn Carousel(
    /// Carousel slides to display.
    #[prop(into)]
    slides: Vec<CarouselSlide>,
    /// Currently active slide index (controlled).
    #[prop(into)]
    active_index: RwSignal<usize>,
    /// Carousel size variant.
    #[prop(optional, into)]
    size: Option<CarouselSize>,
    /// Transition effect.
    #[prop(optional, into)]
    effect: Option<CarouselEffect>,
    /// Display mode (Default or ScrollSnap).
    #[prop(optional, into)]
    mode: Option<CarouselMode>,
    /// Number of visible slides.
    #[prop(optional, into)]
    slides_qty: Option<SlidesQty>,
    /// Center the active slide in viewport.
    #[prop(optional)]
    is_centered: bool,
    /// Adapt height to slide content.
    #[prop(optional)]
    is_auto_height: bool,
    /// Show navigation arrows (default: true).
    #[prop(optional)]
    #[prop(default = true)]
    show_arrows: bool,
    /// Show thumbnail navigation.
    #[prop(optional)]
    show_thumbnails: bool,
    /// Show pagination dots.
    #[prop(optional)]
    show_pagination: bool,
    /// Show slide counter (e.g., "1 / 5").
    #[prop(optional)]
    show_slide_info: bool,
    /// Thumbnail position.
    #[prop(optional, into)]
    thumbnail_position: Option<ThumbnailPosition>,
    /// Pagination dot position.
    #[prop(optional, into)]
    pagination_position: Option<PaginationPosition>,
    /// Autoplay interval in milliseconds (0 = disabled).
    #[prop(optional)]
    #[allow(unused)]
    autoplay: u32,
    /// Enable infinite loop.
    #[prop(optional)]
    loop_slides: bool,
    /// Enable drag/swipe navigation.
    #[prop(optional)]
    is_draggable: bool,
    /// Use CSS scroll-snap mode.
    #[prop(optional)]
    is_snap: bool,
    /// Add spacers in snap mode (default: true).
    #[prop(optional)]
    #[prop(default = true)]
    #[allow(unused)]
    has_snap_spacers: bool,
    /// Right-to-left layout.
    #[prop(optional)]
    is_rtl: bool,
    /// CSS classes for loading state animations.
    #[prop(optional, into)]
    #[allow(unused)]
    loading_classes: Option<LoadingClasses>,
    /// Optional carousel title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Optional carousel description.
    #[prop(optional, into)]
    description: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Custom class for pagination dots.
    #[prop(optional, into)]
    dots_class: Option<String>,
    /// Callback when slide changes.
    #[prop(optional, into)]
    on_change: Option<Callback<usize>>,
) -> impl IntoView {
    // Get theme context for design system
    let theme = try_use_theme();
    let design_system = theme
        .map(|t| t.design_system().as_str())
        .unwrap_or("ant");

    // Store slides for closure capture
    let slides = StoredValue::new(slides);
    let slide_count = slides.with_value(|s| s.len());

    // Resolve optional props
    let size = size.unwrap_or_default();
    let effect = effect.unwrap_or_default();
    let mode = mode.unwrap_or_default();
    let thumb_pos = thumbnail_position.unwrap_or_default();
    let pagination_pos = pagination_position.unwrap_or_default();
    let slides_qty = slides_qty.unwrap_or_default();

    // Calculate visible slides count
    // For now, use fixed value. Responsive requires window resize listener.
    let visible_slides = match &slides_qty {
        SlidesQty::Fixed(n) => *n as usize,
        SlidesQty::Responsive(resp) => {
            // Default to xs value or 1
            resp.xs.unwrap_or(1) as usize
        }
    };

    // Store visible slides for WASM responsive listener
    let visible_slides_signal = RwSignal::new(visible_slides);

    // Set up responsive breakpoint listener (WASM only)
    #[cfg(target_arch = "wasm32")]
    {
        let slides_qty_clone = slides_qty.clone();
        Effect::new(move |_| {
            if let SlidesQty::Responsive(ref resp) = slides_qty_clone {
                use wasm_bindgen::prelude::*;

                let window = match web_sys::window() {
                    Some(w) => w,
                    None => return,
                };

                let width = window.inner_width()
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0) as u32;

                // Determine visible slides based on breakpoint
                let count = if width >= 1536 && resp.xxl.is_some() {
                    resp.xxl.unwrap()
                } else if width >= 1280 && resp.xl.is_some() {
                    resp.xl.unwrap()
                } else if width >= 1024 && resp.lg.is_some() {
                    resp.lg.unwrap()
                } else if width >= 768 && resp.md.is_some() {
                    resp.md.unwrap()
                } else if width >= 640 && resp.sm.is_some() {
                    resp.sm.unwrap()
                } else {
                    resp.xs.unwrap_or(1)
                };

                visible_slides_signal.set(count as usize);

                // Set up resize listener
                let resp_clone = resp.clone();
                let callback = Closure::wrap(Box::new(move || {
                    let window = match web_sys::window() {
                        Some(w) => w,
                        None => return,
                    };

                    let width = window.inner_width()
                        .ok()
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0) as u32;

                    let count = if width >= 1536 && resp_clone.xxl.is_some() {
                        resp_clone.xxl.unwrap()
                    } else if width >= 1280 && resp_clone.xl.is_some() {
                        resp_clone.xl.unwrap()
                    } else if width >= 1024 && resp_clone.lg.is_some() {
                        resp_clone.lg.unwrap()
                    } else if width >= 768 && resp_clone.md.is_some() {
                        resp_clone.md.unwrap()
                    } else if width >= 640 && resp_clone.sm.is_some() {
                        resp_clone.sm.unwrap()
                    } else {
                        resp_clone.xs.unwrap_or(1)
                    };

                    visible_slides_signal.set(count as usize);
                }) as Box<dyn Fn()>);

                let _ = window.add_event_listener_with_callback(
                    "resize",
                    callback.as_ref().unchecked_ref(),
                );

                callback.forget();
            }
        });
    }

    // Drag state (used in WASM for drag/swipe)
    let is_dragging = RwSignal::new(false);
    #[allow(unused)]
    let drag_start_x = RwSignal::new(0.0f64);
    let drag_delta = RwSignal::new(0.0f64);

    // Derived state for navigation
    let can_go_prev = Memo::new(move |_| {
        loop_slides || active_index.get() > 0
    });

    let can_go_next = Memo::new(move |_| {
        let max_index = slide_count.saturating_sub(visible_slides_signal.get());
        loop_slides || active_index.get() < max_index
    });

    // Internal navigation helper
    let navigate_prev = {
        let on_change = on_change.clone();
        move || {
            let current = active_index.get();
            let new_index = if current == 0 {
                if loop_slides {
                    slide_count.saturating_sub(visible_slides_signal.get())
                } else {
                    0
                }
            } else {
                current.saturating_sub(1)
            };
            active_index.set(new_index);
            if let Some(ref cb) = on_change {
                cb.run(new_index);
            }
        }
    };

    let navigate_next = {
        let on_change = on_change.clone();
        move || {
            let current = active_index.get();
            let max_index = slide_count.saturating_sub(visible_slides_signal.get());
            let new_index = if current >= max_index {
                if loop_slides { 0 } else { max_index }
            } else {
                current + 1
            };
            active_index.set(new_index);
            if let Some(ref cb) = on_change {
                cb.run(new_index);
            }
        }
    };

    // Click handlers (need MouseEvent signature)
    let go_prev = {
        let navigate_prev = navigate_prev.clone();
        move |_: web_sys::MouseEvent| {
            navigate_prev();
        }
    };

    let go_next = {
        let navigate_next = navigate_next.clone();
        move |_: web_sys::MouseEvent| {
            navigate_next();
        }
    };

    let go_to = {
        let on_change = on_change.clone();
        move |index: usize| {
            let max_index = slide_count.saturating_sub(visible_slides_signal.get());
            if index <= max_index {
                active_index.set(index);
                if let Some(ref cb) = on_change {
                    cb.run(index);
                }
            }
        }
    };

    // Drag handlers (WASM only)
    #[cfg(target_arch = "wasm32")]
    let (on_mouse_down, on_mouse_move, on_mouse_up, on_touch_start, on_touch_move, on_touch_end) = {
        let navigate_prev_drag = navigate_prev.clone();
        let navigate_next_drag = navigate_next.clone();

        let on_mouse_down = move |e: web_sys::MouseEvent| {
            if !is_draggable { return; }
            is_dragging.set(true);
            drag_start_x.set(e.client_x() as f64);
            drag_delta.set(0.0);
        };

        let on_mouse_move = move |e: web_sys::MouseEvent| {
            if !is_draggable || !is_dragging.get() { return; }
            let delta = e.client_x() as f64 - drag_start_x.get();
            drag_delta.set(delta);
        };

        let navigate_prev_up = navigate_prev_drag.clone();
        let navigate_next_up = navigate_next_drag.clone();
        let on_mouse_up = move |_: web_sys::MouseEvent| {
            if !is_draggable || !is_dragging.get() { return; }
            is_dragging.set(false);

            let delta = drag_delta.get();
            let threshold = 50.0;

            if is_rtl {
                // RTL: directions are reversed
                if delta < -threshold && can_go_prev.get() {
                    navigate_prev_up();
                } else if delta > threshold && can_go_next.get() {
                    navigate_next_up();
                }
            } else {
                if delta > threshold && can_go_prev.get() {
                    navigate_prev_up();
                } else if delta < -threshold && can_go_next.get() {
                    navigate_next_up();
                }
            }
            drag_delta.set(0.0);
        };

        // Touch handlers
        let on_touch_start = move |e: web_sys::TouchEvent| {
            if !is_draggable { return; }
            if let Some(touch) = e.touches().get(0) {
                is_dragging.set(true);
                drag_start_x.set(touch.client_x() as f64);
                drag_delta.set(0.0);
            }
        };

        let on_touch_move = move |e: web_sys::TouchEvent| {
            if !is_draggable || !is_dragging.get() { return; }
            if let Some(touch) = e.touches().get(0) {
                let delta = touch.client_x() as f64 - drag_start_x.get();
                drag_delta.set(delta);
            }
        };

        let navigate_prev_touch = navigate_prev.clone();
        let navigate_next_touch = navigate_next.clone();
        let on_touch_end = move |_: web_sys::TouchEvent| {
            if !is_draggable || !is_dragging.get() { return; }
            is_dragging.set(false);

            let delta = drag_delta.get();
            let threshold = 50.0;

            if is_rtl {
                if delta < -threshold && can_go_prev.get() {
                    navigate_prev_touch();
                } else if delta > threshold && can_go_next.get() {
                    navigate_next_touch();
                }
            } else {
                if delta > threshold && can_go_prev.get() {
                    navigate_prev_touch();
                } else if delta < -threshold && can_go_next.get() {
                    navigate_next_touch();
                }
            }
            drag_delta.set(0.0);
        };

        (on_mouse_down, on_mouse_move, on_mouse_up, on_touch_start, on_touch_move, on_touch_end)
    };

    // Non-WASM stubs
    #[cfg(not(target_arch = "wasm32"))]
    let (on_mouse_down, on_mouse_move, on_mouse_up, on_touch_start, on_touch_move, on_touch_end) = {
        let noop_mouse = |_: web_sys::MouseEvent| {};
        let noop_touch = |_: web_sys::TouchEvent| {};
        (noop_mouse.clone(), noop_mouse.clone(), noop_mouse, noop_touch.clone(), noop_touch.clone(), noop_touch)
    };

    // Autoplay effect
    #[cfg(target_arch = "wasm32")]
    {
        if autoplay > 0 {
            let go_next_auto = {
                let on_change = on_change.clone();
                move || {
                    let current = active_index.get();
                    let max_index = slide_count.saturating_sub(visible_slides_signal.get());
                    let new_index = if current >= max_index {
                        if loop_slides { 0 } else { max_index }
                    } else {
                        current + 1
                    };
                    active_index.set(new_index);
                    if let Some(ref cb) = on_change {
                        cb.run(new_index);
                    }
                }
            };

            Effect::new(move |_| {
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::JsCast;

                let window = web_sys::window().expect("no window");
                let callback = Closure::wrap(Box::new(move || {
                    go_next_auto();
                }) as Box<dyn Fn()>);

                let interval_id = window
                    .set_interval_with_callback_and_timeout_and_arguments_0(
                        callback.as_ref().unchecked_ref(),
                        autoplay as i32,
                    )
                    .expect("failed to set interval");

                callback.forget();

                on_cleanup(move || {
                    if let Some(window) = web_sys::window() {
                        window.clear_interval_with_handle(interval_id);
                    }
                });
            });
        }
    }

    // Keyboard navigation
    let on_keydown = {
        let navigate_prev = navigate_prev.clone();
        let navigate_next = navigate_next.clone();
        let on_change = on_change.clone();
        move |ev: web_sys::KeyboardEvent| {
            match ev.key().as_str() {
                "ArrowLeft" => {
                    ev.prevent_default();
                    if is_rtl { navigate_next(); } else { navigate_prev(); }
                }
                "ArrowUp" => {
                    ev.prevent_default();
                    navigate_prev();
                }
                "ArrowRight" => {
                    ev.prevent_default();
                    if is_rtl { navigate_prev(); } else { navigate_next(); }
                }
                "ArrowDown" | " " => {
                    ev.prevent_default();
                    navigate_next();
                }
                "Home" => {
                    ev.prevent_default();
                    active_index.set(0);
                    if let Some(ref cb) = on_change {
                        cb.run(0);
                    }
                }
                "End" => {
                    ev.prevent_default();
                    let last = slide_count.saturating_sub(visible_slides_signal.get());
                    active_index.set(last);
                    if let Some(ref cb) = on_change {
                        cb.run(last);
                    }
                }
                _ => {}
            }
        }
    };

    // Build CSS classes
    let root_class = move || {
        let ds = design_system;
        let mut classes = vec![
            format!("fx-carousel-{}", ds),
            size.class(&format!("fx-carousel-{}", ds)),
            effect.class(&format!("fx-carousel-{}", ds)),
        ];

        // Mode class
        if matches!(mode, CarouselMode::ScrollSnap) {
            classes.push(format!("fx-carousel-{}-snap", ds));
        }

        if show_thumbnails {
            classes.push(thumb_pos.class(&format!("fx-carousel-{}", ds)));
        } else {
            classes.push(format!("fx-carousel-{}-thumbs-none", ds));
        }

        // Feature classes
        if is_centered {
            classes.push(format!("fx-carousel-{}-centered", ds));
        }
        if is_rtl {
            classes.push(format!("fx-carousel-{}-rtl", ds));
        }
        if is_auto_height {
            classes.push(format!("fx-carousel-{}-auto-height", ds));
        }
        if is_dragging.get() {
            classes.push(format!("fx-carousel-{}-dragging", ds));
        }
        if is_draggable {
            classes.push(format!("fx-carousel-{}-draggable", ds));
        }

        // Initialized class
        classes.push(format!("fx-carousel-{}-init", ds));

        if let Some(ref custom) = class {
            classes.push(custom.clone());
        }

        classes.join(" ")
    };

    // Calculate track transform
    let track_style = move || {
        let index = active_index.get();
        let visible = visible_slides_signal.get();

        // In snap mode, we use scroll-snap instead of transform
        if is_snap {
            return String::new();
        }

        let slide_width = 100.0 / visible as f64;

        // Apply drag delta during dragging
        let drag_offset = if is_dragging.get() {
            drag_delta.get()
        } else {
            0.0
        };

        let base_translate = index as f64 * slide_width;

        // In centered mode, offset to center the active slide
        let centered_offset = if is_centered && visible > 1 {
            (visible as f64 - 1.0) * slide_width / 2.0
        } else {
            0.0
        };

        let translate = base_translate - centered_offset;

        // RTL reverses direction
        let direction = if is_rtl { 1.0 } else { -1.0 };

        match effect {
            CarouselEffect::Slide => {
                if drag_offset != 0.0 {
                    format!(
                        "transform: translateX(calc({}% + {}px)); transition: none;",
                        translate * direction,
                        drag_offset
                    )
                } else {
                    format!("transform: translateX({}%);", translate * direction)
                }
            }
            CarouselEffect::Fade => String::new(),
        }
    };

    // Clone for use in view
    let title_for_aria = title.clone();
    let title_for_header = title.clone();
    let description_for_header = description.clone();
    let has_header = title.is_some() || description.is_some();

    // Pagination dot count (one per slide position)
    let pagination_count = move || {
        slide_count.saturating_sub(visible_slides_signal.get() - 1).max(1)
    };

    view! {
        <div
            class=root_class
            role="region"
            aria-roledescription="carousel"
            aria-label=title_for_aria.unwrap_or_else(|| "Image carousel".to_string())
            tabindex="0"
            dir=if is_rtl { "rtl" } else { "ltr" }
            on:keydown=on_keydown
            on:mousedown=on_mouse_down
            on:mousemove=on_mouse_move
            on:mouseup=on_mouse_up.clone()
            on:mouseleave=on_mouse_up
            on:touchstart=on_touch_start
            on:touchmove=on_touch_move
            on:touchend=on_touch_end
        >
            // Optional header with title/description
            {has_header.then(|| {
                let ds = design_system;
                view! {
                    <div class=format!("fx-carousel-{}-header", ds)>
                        {title_for_header.clone().map(|t| view! {
                            <h3 class=format!("fx-carousel-{}-title", ds)>{t}</h3>
                        })}
                        {description_for_header.clone().map(|d| view! {
                            <p class=format!("fx-carousel-{}-description", ds)>{d}</p>
                        })}
                    </div>
                }
            })}

            // Slide info (current / total) - top position
            {(show_slide_info && matches!(pagination_pos, PaginationPosition::Top | PaginationPosition::TopOutside)).then(|| {
                let ds = design_system;
                view! {
                    <div class=format!("fx-carousel-{}-info fx-carousel-{}-info-top", ds, ds)>
                        <span class=format!("fx-carousel-{}-info-current", ds)>
                            {move || active_index.get() + 1}
                        </span>
                        <span class=format!("fx-carousel-{}-info-separator", ds)>" / "</span>
                        <span class=format!("fx-carousel-{}-info-total", ds)>
                            {slide_count}
                        </span>
                    </div>
                }
            })}

            // Pagination dots - top position
            {(show_pagination && matches!(pagination_pos, PaginationPosition::Top | PaginationPosition::TopOutside)).then(|| {
                let ds = design_system;
                let dots_cls = dots_class.clone();
                view! {
                    <div
                        class=move || {
                            let mut classes = vec![
                                format!("fx-carousel-{}-pagination", ds),
                                format!("fx-carousel-{}-pagination-top", ds),
                            ];
                            if let Some(ref custom) = dots_cls {
                                classes.push(custom.clone());
                            }
                            classes.join(" ")
                        }
                        role="tablist"
                        aria-label="Slide navigation"
                    >
                        {move || (0..pagination_count()).map(|index| {
                            let is_active = move || active_index.get() == index;
                            let go_to_slide = move |_: web_sys::MouseEvent| go_to(index);

                            view! {
                                <button
                                    type="button"
                                    class=move || {
                                        let mut classes = vec![format!("fx-carousel-{}-dot", ds)];
                                        if is_active() {
                                            classes.push(format!("fx-carousel-{}-dot-active", ds));
                                        }
                                        classes.join(" ")
                                    }
                                    role="tab"
                                    aria-selected=move || is_active()
                                    aria-label=format!("Go to slide {}", index + 1)
                                    on:click=go_to_slide
                                />
                            }
                        }).collect_view()}
                    </div>
                }
            })}

            // Main viewport with slides
            <div class=move || format!("fx-carousel-{}-viewport", design_system)>
                // Previous button
                {show_arrows.then(|| {
                    let ds = design_system;
                    view! {
                        <button
                            type="button"
                            class=move || {
                                let mut classes = vec![
                                    format!("fx-carousel-{}-arrow", ds),
                                    format!("fx-carousel-{}-arrow-prev", ds),
                                ];
                                if !can_go_prev.get() {
                                    classes.push(format!("fx-carousel-{}-arrow-disabled", ds));
                                }
                                classes.join(" ")
                            }
                            disabled=move || !can_go_prev.get()
                            on:click=go_prev.clone()
                            aria-label="Previous slide"
                        >
                            <svg viewBox="0 0 24 24" fill="currentColor" width="24" height="24">
                                <path d="M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z"/>
                            </svg>
                        </button>
                    }
                })}

                // Slides track
                <div
                    class=move || {
                        let ds = design_system;
                        let mut classes = vec![format!("fx-carousel-{}-track", ds)];
                        if is_snap {
                            classes.push(format!("fx-carousel-{}-track-snap", ds));
                        }
                        classes.join(" ")
                    }
                    style=track_style
                >
                    {slides.with_value(|slides| {
                        slides.iter().enumerate().map(|(index, slide)| {
                            let slide = slide.clone();
                            let is_active = move || {
                                let current = active_index.get();
                                let visible = visible_slides_signal.get();
                                index >= current && index < current + visible
                            };
                            let ds = design_system;
                            // Reactive slide style - recomputes when visible_slides_signal changes
                            // Always apply width to ensure proper layout
                            let slide_style_reactive = move || {
                                let visible = visible_slides_signal.get();
                                let width = 100.0 / visible as f64;
                                format!("flex: 0 0 {}%; max-width: {}%; min-width: {}%;", width, width, width)
                            };

                            view! {
                                <div
                                    class=move || {
                                        let mut classes = vec![format!("fx-carousel-{}-slide", ds)];
                                        if is_active() {
                                            classes.push(format!("fx-carousel-{}-slide-active", ds));
                                        }
                                        if is_snap {
                                            classes.push(format!("fx-carousel-{}-slide-snap", ds));
                                        }
                                        classes.join(" ")
                                    }
                                    style=slide_style_reactive
                                    role="group"
                                    aria-roledescription="slide"
                                    aria-label=format!("Slide {} of {}", index + 1, slide_count)
                                    aria-hidden=move || !is_active()
                                >
                                    {match &slide.content {
                                        SlideContent::Image { src, alt } => {
                                            let src = src.clone();
                                            let alt = alt.clone().unwrap_or_default();
                                            view! {
                                                <img
                                                    class=format!("fx-carousel-{}-slide-image", ds)
                                                    src=src
                                                    alt=alt
                                                    loading="lazy"
                                                    draggable="false"
                                                />
                                            }.into_any()
                                        }
                                        SlideContent::Video { src, poster, autoplay: video_autoplay, muted, controls } => {
                                            let src = src.clone();
                                            let poster = poster.clone();
                                            let video_autoplay = *video_autoplay;
                                            let muted = *muted;
                                            let controls = *controls;
                                            view! {
                                                <video
                                                    class=format!("fx-carousel-{}-slide-video", ds)
                                                    src=src
                                                    poster=poster
                                                    autoplay=video_autoplay
                                                    muted=muted
                                                    controls=controls
                                                    playsinline=true
                                                />
                                            }.into_any()
                                        }
                                        SlideContent::Custom { html } => {
                                            // Render testimonial/custom content
                                            let author = slide.author.clone();
                                            let author_location = slide.author_location.clone();
                                            let author_avatar = slide.author_avatar.clone();
                                            let rating = slide.rating;
                                            let quote = slide.description.clone();
                                            let html_content = html.clone();

                                            view! {
                                                <div class=format!("fx-carousel-{}-testimonial", ds)>
                                                    // Quote/description
                                                    {quote.map(|q| view! {
                                                        <div class=format!("fx-carousel-{}-testimonial-content", ds)>
                                                            <p>{q}</p>
                                                        </div>
                                                    })}

                                                    // Optional raw HTML content
                                                    {html_content.map(|h| view! {
                                                        <div class=format!("fx-carousel-{}-testimonial-html", ds) inner_html=h />
                                                    })}

                                                    // Author info
                                                    {author.as_ref().map(|_| {
                                                        let avatar = author_avatar.clone().unwrap_or_else(|| {
                                                            author.as_ref().map(|a| a.chars().next().unwrap_or('?').to_string()).unwrap_or_default()
                                                        });
                                                        view! {
                                                            <div class=format!("fx-carousel-{}-testimonial-author", ds)>
                                                                <div class=format!("fx-carousel-{}-testimonial-avatar", ds)>
                                                                    {avatar}
                                                                </div>
                                                                <div class=format!("fx-carousel-{}-testimonial-info", ds)>
                                                                    <span class=format!("fx-carousel-{}-testimonial-name", ds)>
                                                                        {author.clone().unwrap_or_default()}
                                                                    </span>
                                                                    {author_location.clone().map(|loc| view! {
                                                                        <span class=format!("fx-carousel-{}-testimonial-location", ds)>
                                                                            {loc}
                                                                        </span>
                                                                    })}
                                                                </div>
                                                                // Rating stars
                                                                {rating.map(|r| {
                                                                    let stars = (0..5).map(|i| {
                                                                        if i < r {
                                                                            view! { <span class=format!("fx-carousel-{}-star fx-carousel-{}-star-filled", ds, ds)>"★"</span> }
                                                                        } else {
                                                                            view! { <span class=format!("fx-carousel-{}-star", ds)>"☆"</span> }
                                                                        }
                                                                    }).collect_view();
                                                                    view! {
                                                                        <div class=format!("fx-carousel-{}-testimonial-rating", ds)>
                                                                            {stars}
                                                                        </div>
                                                                    }
                                                                })}
                                                            </div>
                                                        }
                                                    })}
                                                </div>
                                            }.into_any()
                                        }
                                    }}

                                    // Slide overlay content (title/description) - only for image/video slides
                                    {(!slide.is_testimonial() && (slide.title.is_some() || slide.description.is_some())).then(|| {
                                        view! {
                                            <div class=format!("fx-carousel-{}-slide-content", ds)>
                                                {slide.title.clone().map(|t| view! {
                                                    <h4 class=format!("fx-carousel-{}-slide-title", ds)>{t}</h4>
                                                })}
                                                {slide.description.clone().map(|d| view! {
                                                    <p class=format!("fx-carousel-{}-slide-description", ds)>{d}</p>
                                                })}
                                            </div>
                                        }
                                    })}
                                </div>
                            }
                        }).collect_view()
                    })}
                </div>

                // Next button
                {show_arrows.then(|| {
                    let ds = design_system;
                    view! {
                        <button
                            type="button"
                            class=move || {
                                let mut classes = vec![
                                    format!("fx-carousel-{}-arrow", ds),
                                    format!("fx-carousel-{}-arrow-next", ds),
                                ];
                                if !can_go_next.get() {
                                    classes.push(format!("fx-carousel-{}-arrow-disabled", ds));
                                }
                                classes.join(" ")
                            }
                            disabled=move || !can_go_next.get()
                            on:click=go_next.clone()
                            aria-label="Next slide"
                        >
                            <svg viewBox="0 0 24 24" fill="currentColor" width="24" height="24">
                                <path d="M8.59 16.59L10 18l6-6-6-6-1.41 1.41L13.17 12z"/>
                            </svg>
                        </button>
                    }
                })}
            </div>

            // Slide info (current / total) - bottom position
            {(show_slide_info && matches!(pagination_pos, PaginationPosition::Bottom | PaginationPosition::BottomOutside)).then(|| {
                let ds = design_system;
                view! {
                    <div class=format!("fx-carousel-{}-info fx-carousel-{}-info-bottom", ds, ds)>
                        <span class=format!("fx-carousel-{}-info-current", ds)>
                            {move || active_index.get() + 1}
                        </span>
                        <span class=format!("fx-carousel-{}-info-separator", ds)>" / "</span>
                        <span class=format!("fx-carousel-{}-info-total", ds)>
                            {slide_count}
                        </span>
                    </div>
                }
            })}

            // Pagination dots - bottom position
            {(show_pagination && matches!(pagination_pos, PaginationPosition::Bottom | PaginationPosition::BottomOutside)).then(|| {
                let ds = design_system;
                let dots_cls = dots_class.clone();
                view! {
                    <div
                        class=move || {
                            let mut classes = vec![
                                format!("fx-carousel-{}-pagination", ds),
                                format!("fx-carousel-{}-pagination-bottom", ds),
                            ];
                            if let Some(ref custom) = dots_cls {
                                classes.push(custom.clone());
                            }
                            classes.join(" ")
                        }
                        role="tablist"
                        aria-label="Slide navigation"
                    >
                        {move || (0..pagination_count()).map(|index| {
                            let is_active = move || active_index.get() == index;
                            let go_to_slide = move |_: web_sys::MouseEvent| go_to(index);

                            view! {
                                <button
                                    type="button"
                                    class=move || {
                                        let mut classes = vec![format!("fx-carousel-{}-dot", ds)];
                                        if is_active() {
                                            classes.push(format!("fx-carousel-{}-dot-active", ds));
                                        }
                                        classes.join(" ")
                                    }
                                    role="tab"
                                    aria-selected=move || is_active()
                                    aria-label=format!("Go to slide {}", index + 1)
                                    on:click=go_to_slide
                                />
                            }
                        }).collect_view()}
                    </div>
                }
            })}

            // Thumbnail navigation
            {show_thumbnails.then(|| {
                view! {
                    <div
                        class=move || format!("fx-carousel-{}-thumbnails", design_system)
                        role="tablist"
                        aria-label="Slide thumbnails"
                    >
                        {slides.with_value(|slides| {
                            slides.iter().enumerate().map(|(index, slide)| {
                                let thumb_url = slide.get_thumbnail();
                                let slide_title = slide.title.clone();
                                let is_active = move || {
                                    let current = active_index.get();
                                    let visible = visible_slides_signal.get();
                                    index >= current && index < current + visible
                                };
                                let go_to_slide = move |_: web_sys::MouseEvent| go_to(index);
                                let ds = design_system;

                                view! {
                                    <button
                                        type="button"
                                        class=move || {
                                            let mut classes = vec![format!("fx-carousel-{}-thumbnail", ds)];
                                            if is_active() {
                                                classes.push(format!("fx-carousel-{}-thumbnail-active", ds));
                                            }
                                            classes.join(" ")
                                        }
                                        role="tab"
                                        aria-selected=move || is_active()
                                        aria-label=format!("Go to slide {}: {}", index + 1, slide_title.clone().unwrap_or_default())
                                        on:click=go_to_slide
                                    >
                                        {thumb_url.map(|url| view! {
                                            <img
                                                src=url
                                                alt=""
                                                loading="lazy"
                                                draggable="false"
                                            />
                                        })}
                                        <span class=move || format!("fx-carousel-{}-thumbnail-label", ds)>
                                            {slide_title.clone().unwrap_or_else(|| format!("Slide {}", index + 1))}
                                        </span>
                                    </button>
                                }
                            }).collect_view()
                        })}
                    </div>
                }
            })}
        </div>
    }
}
