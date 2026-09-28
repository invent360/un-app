# Guide/Tutorial Component Design Proposal

## Overview

A reusable component for displaying step-by-step guides/tutorials to users in an engaging, self-explanatory manner. The component will follow ember-fx patterns, support theming, and be mobile-first responsive.

---

## Design Alternatives

### Option 1: Carousel Overlay Guide

**Concept:** Full-screen or modal overlay with swipeable carousel slides. Each step is a card with image, title, and description.

```
┌─────────────────────────────────────────┐
│  ╔═══════════════════════════════════╗  │
│  ║         [Image/Screenshot]         ║  │
│  ║                                    ║  │
│  ║  ┌──────────────────────────────┐  ║  │
│  ║  │ Step 1 of 8                  │  ║  │
│  ║  │ ━━━━━━━━━░░░░░░░░░░░░░░░░░░░ │  ║  │
│  ║  │                              │  ║  │
│  ║  │ Welcome to Unetwork          │  ║  │
│  ║  │                              │  ║  │
│  ║  │ Open the app to see the      │  ║  │
│  ║  │ welcome screen. Click GET    │  ║  │
│  ║  │ STARTED to proceed.          │  ║  │
│  ║  │                              │  ║  │
│  ║  │  ○ ○ ○ ○ ○ ○ ● ○ (dots)      │  ║  │
│  ║  │                              │  ║  │
│  ║  │  [← Back]        [Next →]    │  ║  │
│  ║  └──────────────────────────────┘  ║  │
│  ╚═══════════════════════════════════╝  │
│                                         │
│              (dark overlay)             │
└─────────────────────────────────────────┘
```

**Features:**
- Swipe/drag navigation (mobile-friendly)
- Progress bar and dot indicators
- Keyboard navigation (←/→/Esc)
- Auto-advance option with pause on interaction
- Full-screen mode on mobile
- Modal/centered on desktop

**Pros:**
- Immersive, focused experience
- Excellent for onboarding flows
- Familiar pattern (app store screenshots)
- Works great on mobile

**Cons:**
- Blocks other content
- May feel intrusive for optional guides
- User must complete or dismiss

---

### Option 2: Side Panel Stepper (Drawer Style)

**Concept:** Slide-in side panel with vertical step navigation on the left and content area on the right.

```
┌─────────────────────────────────────────────────────────┐
│  Main App Content                    │ Guide Panel      │
│                                      │┌────────────────┐│
│                                      ││ App Setup      ││
│                                      ││                ││
│  (dimmed/blurred)                    ││ ● Step 1       ││
│                                      ││ ○ Step 2       ││
│                                      ││ ○ Step 3       ││
│                                      ││ ○ Step 4       ││
│                                      ││───────────────-││
│                                      ││                ││
│                                      ││ [Image]        ││
│                                      ││                ││
│                                      ││ Welcome to     ││
│                                      ││ Unetwork       ││
│                                      ││                ││
│                                      ││ Description... ││
│                                      ││                ││
│                                      ││ [Next Step →]  ││
│                                      │└────────────────┘│
└─────────────────────────────────────────────────────────┘
```

**Mobile Layout:**
```
┌─────────────────────────┐
│ App Setup Guide    [×]  │
│─────────────────────────│
│ ● ○ ○ ○ ○ ○ ○ ○ (horiz) │
│─────────────────────────│
│                         │
│    [Image/Screenshot]   │
│                         │
│─────────────────────────│
│ Step 1: Welcome         │
│                         │
│ Description text here   │
│ with markdown support   │
│                         │
│─────────────────────────│
│ [← Back]    [Next →]    │
└─────────────────────────┘
```

**Features:**
- Non-blocking (content still visible)
- Clear step progress
- Collapsible/expandable
- Slide from right (RTL: left)
- Sticky step navigation

**Pros:**
- Context-aware (see app behind)
- Easy to navigate non-linearly
- Good for reference guides
- Doesn't take full screen

**Cons:**
- Smaller image area
- May compete for attention
- Complex layout on mobile

---

### Option 3: Inline Accordion Guide

**Concept:** Expandable accordion embedded in the page. Each step is a collapsible section.

```
┌─────────────────────────────────────────────────────────┐
│                                                         │
│  ┌───────────────────────────────────────────────────┐  │
│  │ 📖 App Setup Guide                    [Expand All] │  │
│  │ 5 min • 8 steps • Easy                             │  │
│  ├───────────────────────────────────────────────────┤  │
│  │ ▼ Step 1: Welcome to Unetwork            ✓ Done   │  │
│  │   ┌─────────────────────────────────────────────┐ │  │
│  │   │ [Image]  Open the app to see the welcome   │ │  │
│  │   │          screen. Click GET STARTED.        │ │  │
│  │   │                                             │ │  │
│  │   │          [Mark Complete] [Next Step]        │ │  │
│  │   └─────────────────────────────────────────────┘ │  │
│  ├───────────────────────────────────────────────────┤  │
│  │ ▶ Step 2: Choose Your Role                  ○     │  │
│  ├───────────────────────────────────────────────────┤  │
│  │ ▶ Step 3: Access Your Leased License        ○     │  │
│  ├───────────────────────────────────────────────────┤  │
│  │ ▶ Step 4: Choose Sign-In Method             ○     │  │
│  └───────────────────────────────────────────────────┘  │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

**Features:**
- Inline/embedded in page
- Progress tracking per step
- Expandable sections
- Bookmark/resume capability
- Checkmarks for completion

**Pros:**
- Non-intrusive
- Self-paced
- Good for documentation
- SEO friendly (content visible)
- Easy to scan/skip

**Cons:**
- Less immersive
- Long guides can be overwhelming
- Images smaller/secondary

---

### Option 4: Interactive Spotlight Tour

**Concept:** Highlights actual UI elements with tooltips/popovers while walking through steps. Best for in-app tours.

```
┌─────────────────────────────────────────────────────────┐
│  ┌─────────────────────────────────────────────────┐    │
│  │ DJED NODES        [Start Earning]  Tasks Guides │    │
│  └─────────────────────────────────────────────────┘    │
│                                                         │
│  ┌──────────────────────────────────────────────────┐   │
│  │                                                  │   │
│  │     Welcome to the Guides page!                  │   │
│  │                                                  │   │
│  │     Here you'll find step-by-step tutorials      │   │
│  │     to help you get started.                     │   │
│  │                                                  │   │
│  │     ○○●○○○  Step 3 of 6                          │   │
│  │                                                  │   │
│  │     [Skip Tour]  [← Back]  [Next →]              │   │
│  │                                                  │   │
│  └────────────────────────┬─────────────────────────┘   │
│                           │                             │
│                           ▼                             │
│  ┌─────────────────────────────────────────────────┐    │
│  │ ╭─────────────────────────────────────────────╮ │    │
│  │ │  ★ App Setup Guide        ← HIGHLIGHTED     │ │    │
│  │ │  Complete guide to setup...                 │ │    │
│  │ ╰─────────────────────────────────────────────╯ │    │
│  │   Other Guide Card                              │    │
│  └─────────────────────────────────────────────────┘    │
│                                                         │
│              (rest of page dimmed)                      │
└─────────────────────────────────────────────────────────┘
```

**Features:**
- Highlights real UI elements
- Contextual positioning
- Smart repositioning on resize
- Skip/dismiss option
- Hotspot indicators

**Pros:**
- Most engaging
- Contextual learning
- Shows actual UI
- Great for onboarding

**Cons:**
- Requires UI element references
- Complex to implement
- Not suitable for external content
- Only works within the app

---

## Recommendation

For your use case (displaying guides from database with images and steps), I recommend:

**Primary: Option 1 (Carousel Overlay Guide)**
- Best for step-by-step visual guides
- Works great with the existing guide data structure (steps with images)
- Immersive, focused experience
- Mobile-friendly swipe gestures

**Secondary: Option 3 (Accordion Guide)**
- Good for documentation/reference
- Can be embedded in the guides page
- Users can scan and jump to specific steps

---

## Proposed Component Structure

### Component Name: `GuideViewer`

```
/crates/components/src/guide/
├── mod.rs              # Module exports
├── types.rs            # Enums and types
├── guide_viewer.rs     # Main orchestrator component
├── guide_carousel.rs   # Carousel variant
├── guide_accordion.rs  # Accordion variant
├── guide_drawer.rs     # Drawer/panel variant
├── guide_step.rs       # Individual step rendering
├── guide_progress.rs   # Progress indicators
└── guide_controls.rs   # Navigation buttons
```

### Type Definitions

```rust
/// Guide display variants
pub enum GuideVariant {
    Carousel,      // Full-screen/modal carousel
    Accordion,     // Inline expandable sections
    Drawer,        // Side panel
    Spotlight,     // UI tour (future)
}

/// Guide viewer size
pub enum GuideSize {
    Small,         // Compact
    Medium,        // Default
    Large,         // Expanded
    FullScreen,    // Mobile/immersive
}

/// Guide step structure
pub struct GuideStep {
    pub order: i32,
    pub title: String,
    pub description: String,
    pub image: Option<String>,
    pub video: Option<String>,
}

/// Guide content from CMS
pub struct GuideContent {
    pub title: String,
    pub description: String,
    pub steps: Vec<GuideStep>,
    pub difficulty: Option<String>,
    pub duration_minutes: Option<i32>,
    pub thumbnail: Option<String>,
}
```

### CSS Class Convention

```css
/* Base */
.fx-guide-ant { }
.fx-guide-ant-carousel { }
.fx-guide-ant-accordion { }
.fx-guide-ant-drawer { }

/* Sizes */
.fx-guide-ant-sm { }
.fx-guide-ant-md { }
.fx-guide-ant-lg { }
.fx-guide-ant-fullscreen { }

/* Sub-elements */
.fx-guide-ant-overlay { }
.fx-guide-ant-container { }
.fx-guide-ant-header { }
.fx-guide-ant-content { }
.fx-guide-ant-image { }
.fx-guide-ant-step { }
.fx-guide-ant-step-title { }
.fx-guide-ant-step-desc { }
.fx-guide-ant-progress { }
.fx-guide-ant-dots { }
.fx-guide-ant-controls { }
.fx-guide-ant-close { }

/* States */
.fx-guide-ant-active { }
.fx-guide-ant-completed { }
.fx-guide-ant-loading { }

/* Responsive */
@media (max-width: 768px) {
    .fx-guide-ant-carousel {
        /* Full screen on mobile */
    }
}
```

---

## Mobile-First Responsive Approach

### Breakpoints

```css
/* Mobile first - base styles for mobile */
.fx-guide-ant-carousel {
    position: fixed;
    inset: 0;
    /* Full screen by default */
}

/* Tablet and up */
@media (min-width: 768px) {
    .fx-guide-ant-carousel {
        inset: 5vh 10vw;
        border-radius: var(--fx-radius-lg);
    }
}

/* Desktop */
@media (min-width: 1024px) {
    .fx-guide-ant-carousel {
        inset: 10vh 20vw;
        max-width: 800px;
        margin: auto;
    }
}
```

### Touch Gestures

- **Swipe left/right** - Navigate steps
- **Swipe down** - Dismiss (mobile)
- **Pinch** - Zoom images
- **Tap** - Play/pause auto-advance

---

## Which option would you like to implement?

Please choose from:
1. **Carousel Overlay** - Immersive, swipeable
2. **Side Panel Stepper** - Non-blocking, drawer style
3. **Inline Accordion** - Embedded, expandable
4. **Spotlight Tour** - Contextual UI highlights

Or a combination of multiple variants in one component?
