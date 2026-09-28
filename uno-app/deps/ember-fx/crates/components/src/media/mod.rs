//! Media components for displaying images, videos, and galleries.
//!
//! This module provides components for displaying media content including
//! carousels, image viewers, and video players.
//!
//! # Components
//!
//! - [`Carousel`] - Feature-rich image/video carousel
//!
//! # Example
//!
//! ```ignore
//! use ember_fx_components::{Carousel, CarouselSlide, CarouselSize, SlidesQty};
//! use leptos::prelude::*;
//!
//! let slides = vec![
//!     CarouselSlide::image("1", "/images/photo1.jpg")
//!         .title("First Photo")
//!         .alt("A beautiful landscape"),
//!     CarouselSlide::image("2", "/images/photo2.jpg")
//!         .title("Second Photo"),
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
//!         size=CarouselSize::Large
//!         slides_qty=SlidesQty::Fixed(3)
//!         show_thumbnails=true
//!         show_pagination=true
//!         is_draggable=true
//!         loop_slides=true
//!         title="Media Gallery"
//!         description="Browse through our collection"
//!     />
//! }
//! ```
//!
//! # Features
//!
//! - **Image slides** - Display images with lazy loading
//! - **Video slides** - Display videos with autoplay and controls
//! - **Navigation arrows** - Previous/next navigation
//! - **Thumbnail navigation** - Click thumbnails to navigate
//! - **Pagination dots** - Dot-based navigation
//! - **Slide info** - Display current/total counter
//! - **Keyboard navigation** - Arrow keys, Home, End
//! - **Drag/Swipe** - Touch and mouse drag support
//! - **Multiple slides** - Show multiple slides at once
//! - **Responsive** - Breakpoint-based slide count
//! - **Centered mode** - Center active slide in viewport
//! - **Snap scroll** - CSS scroll-snap mode
//! - **RTL support** - Right-to-left layouts
//! - **Auto-height** - Adapt height to content
//! - **Autoplay** - Automatic slide advancement
//! - **Loop mode** - Infinite navigation
//! - **Loading states** - CSS class-based loading animations
//! - **Accessible** - ARIA attributes and keyboard support

mod types;
mod carousel;

pub use types::{
    CarouselSize,
    CarouselEffect,
    CarouselMode,
    ThumbnailPosition,
    PaginationPosition,
    SlideContent,
    CarouselSlide,
    SlidesQty,
    ResponsiveSlidesQty,
    LoadingClasses,
};

pub use carousel::Carousel;
