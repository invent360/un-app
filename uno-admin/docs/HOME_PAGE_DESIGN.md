# Home Page CMS - Generic Schema-Driven Design

## Overview

The Home Page CMS uses a **generic schema-driven architecture** that allows flexible content management without requiring code changes for each section type. All sections share a common structure with type-specific data stored in a flexible `data` field.

---

## Architecture

```
+-------------------------------------------+
|            HomePageData                   |
+-------------------------------------------+
| title: String                             |
| sections: Vec<Section>                    |
+-------------------------------------------+
              |
              v
+-------------------------------------------+
|             Section (Generic)              |
+-------------------------------------------+
| section_type: "hero" | "how_it_works" |   |
|               "earnings" | "testimonials" |
+-------------------------------------------+
| COMMON FIELDS:                            |
|   title: String                           |
|   description: String                     |
|   display_order: i32                      |
|   is_visible: bool                        |
|   highlights: Vec<String>     (optional)  |
|   images: Vec<String>         (optional)  |
|   videos: Vec<String>         (optional)  |
|   links: Vec<SectionLink>     (optional)  |
+-------------------------------------------+
| TYPE-SPECIFIC DATA:                       |
|   data: serde_json::Value                 |
|   (schema-driven, varies by section_type) |
+-------------------------------------------+
              |
              v
+-------------------------------------------+
|          SectionSchema                     |
+-------------------------------------------+
| Defines fields for each section_type      |
| - FieldDef[]                              |
| - Default values                          |
| - UI hints (icon, color)                  |
+-------------------------------------------+
```

---

## Data Model

### Section Struct (Generic)

```rust
pub struct Section {
    // === Type Identifier ===
    pub section_type: String,   // "hero", "how_it_works", "earnings", "testimonials"

    // === Common Fields (All Sections) ===
    pub title: String,          // Section heading
    pub description: String,    // Section subheading/description
    pub display_order: i32,     // Sort order (1, 2, 3...)
    pub is_visible: bool,       // Show/hide section

    // === Optional Common Fields ===
    pub highlights: Vec<String>,    // Feature bullets/highlights
    pub images: Vec<String>,        // Image URLs
    pub videos: Vec<String>,        // Video URLs
    pub links: Vec<SectionLink>,    // Download/action buttons

    // === Type-Specific Data ===
    pub data: serde_json::Value,  // Schema-driven flexible data
}
```

### SectionLink (Download Buttons)

```rust
pub enum LinkPlatform {
    AppStore,    // Apple App Store - icon: "apple"
    GooglePlay,  // Google Play Store - icon: "playstore"
    Apk,         // Direct APK download - icon: "android"
    Custom,      // Generic/custom link - icon: "link"
}

pub struct SectionLink {
    pub platform: LinkPlatform,  // Determines icon & default labels
    pub label: String,           // "App Store", "Google Play", "APK File"
    pub sublabel: String,        // "Download on the", "Get it on", "Direct download"
    pub href: String,            // The destination URL
    pub target: String,          // "_blank" (default - opens in new tab)
    pub rel: String,             // "noopener noreferrer" (default - security)
    pub icon: Option<String>,    // Custom icon override (optional)
    pub is_visible: bool,        // Show/hide toggle
}
```

**Platform Defaults:**

| Platform | Icon | Sublabel | Label |
|----------|------|----------|-------|
| `app_store` | apple | "Download on the" | "App Store" |
| `google_play` | playstore | "Get it on" | "Google Play" |
| `apk` | android | "Direct download" | "APK File" |
| `custom` | link | "" | "Link" |

### HomePageData

```rust
pub struct HomePageData {
    pub title: String,           // Admin reference title
    pub sections: Vec<Section>,  // All page sections
}
```

---

## Schema Definition System

### FieldType Enum

```rust
pub enum FieldType {
    Text,                        // Single-line text input
    TextArea,                    // Multi-line text input
    StringList,                  // List of strings (tags/highlights)
    MediaList,                   // List of media URLs (images/videos)
    Repeater(Vec<FieldDef>),     // List of objects with sub-fields
    Number,                      // Number input
    Boolean,                     // Toggle/checkbox
    OptionalText,                // Optional text field
}
```

### FieldDef

```rust
pub struct FieldDef {
    pub key: &'static str,       // JSON key
    pub label: &'static str,     // UI label
    pub field_type: FieldType,   // Field type
    pub required: bool,          // Validation
    pub placeholder: &'static str,
}
```

### SectionSchema

```rust
pub struct SectionSchema {
    pub section_type: &'static str,
    pub label: &'static str,       // "Hero", "How It Works", etc.
    pub description: &'static str,
    pub icon: &'static str,        // Icon name for UI
    pub color: &'static str,       // Accent color (hex)
    pub default_title: &'static str,
    pub fields: Vec<FieldDef>,     // Type-specific fields
}
```

---

## Section Schemas

### 1. Hero Section

```
Type: "hero"
Label: "Hero"
Icon: star
Color: #8b5cf6 (purple)

Common Fields:
  - title: "Your Phone Can Earn Money For Free"
  - description: Subheadline text
  - highlights: ["100% Free", "$5-15/month", "Works in Background"]
  - images: Hero carousel images

Type-Specific Fields (in data):
  - (none - uses common fields only)
```

### 2. How It Works Section

```
Type: "how_it_works"
Label: "How It Works"
Icon: order
Color: #3b82f6 (blue)

Common Fields:
  - title: "How It Works"
  - description: Section subtitle
  - links: Download buttons (App Store, Google Play, APK)

Type-Specific Fields (in data):
  - steps: Array of {
      icon: String,
      title: String,
      description: String
    }
```

### 3. Earnings Section

```
Type: "earnings"
Label: "Earnings"
Icon: revenue
Color: #22c55e (green)

Common Fields:
  - title: "Real Earnings"
  - description: Section subtitle

Type-Specific Fields (in data):
  - tiers: Array of {
      name: String,
      min_earnings: Number,
      max_earnings: Number,
      period: String,
      features: String[],
      is_popular: Boolean
    }
  - disclaimer: String
```

### 4. Testimonials Section

```
Type: "testimonials"
Label: "Testimonials"
Icon: user
Color: #f59e0b (amber)

Common Fields:
  - title: "Real People. Real Earnings."
  - description: Section subtitle

Type-Specific Fields (in data):
  - api_endpoint: String (optional, for dynamic loading)
  - testimonials: Array of {
      quote: String,
      author_name: String,
      author_location: String,
      rating: Number (1-5)
    }
```

---

## UI Component Hierarchy

```
HomePageEditor (Page)
    |
    +-- EditorHeader
    |     +-- Save Button
    |     +-- Preview Button
    |     +-- Publish Button
    |
    +-- HomeSectionEditor (Sections List)
          |
          +-- SectionPreview (Collapsed Card)
          |     +-- Icon + Title
          |     +-- Visibility Toggle
          |     +-- Drag Handle
          |     +-- Media/Highlights Count
          |
          +-- SectionEditorModal (Expanded Editor)
                |
                +-- CommonFieldsSection
                |     +-- Title Input
                |     +-- Description TextArea
                |     +-- Visibility Toggle
                |
                +-- SectionHighlightsField
                |     +-- Collapsible
                |     +-- String List Editor
                |
                +-- SectionMediaField (Images)
                |     +-- Collapsible
                |     +-- Media Upload List
                |
                +-- SectionMediaField (Videos)
                |     +-- Collapsible
                |     +-- Media Upload List
                |
                +-- SectionLinksField
                |     +-- Collapsible
                |     +-- Platform selector (App Store/Google Play/APK/Custom)
                |     +-- Link URL, label, sublabel inputs
                |     +-- Visibility toggle per link
                |
                +-- TypeSpecificFields
                      +-- Rendered from SectionSchema.fields
                      +-- RepeaterItem for nested arrays
```

---

## JSON Structure Example

```json
{
  "title": "Home Page",
  "sections": [
    {
      "section_type": "hero",
      "title": "Your Phone Can Earn Money For Free",
      "description": "Turn your smartphone into a passive income machine.",
      "display_order": 1,
      "is_visible": true,
      "highlights": ["100% Free", "$5-15/month", "Works in Background"],
      "images": ["images/hero/phone-mockup.png"],
      "videos": [],
      "data": {}
    },
    {
      "section_type": "how_it_works",
      "title": "How It Works",
      "description": "Start earning in 3 simple steps",
      "display_order": 2,
      "is_visible": true,
      "highlights": [],
      "images": [],
      "videos": [],
      "links": [
        {
          "platform": "app_store",
          "label": "App Store",
          "sublabel": "Download on the",
          "href": "https://apps.apple.com/gb/app/unity-network-app/id6755482738",
          "target": "_blank",
          "rel": "noopener noreferrer",
          "is_visible": true
        },
        {
          "platform": "google_play",
          "label": "Google Play",
          "sublabel": "Get it on",
          "href": "https://play.google.com/store/apps/details?id=io.unetwork.app",
          "target": "_blank",
          "rel": "noopener noreferrer",
          "is_visible": true
        },
        {
          "platform": "apk",
          "label": "APK File",
          "sublabel": "Direct download",
          "href": "https://releases.unetwork.io/android/",
          "target": "_blank",
          "rel": "noopener noreferrer",
          "is_visible": true
        }
      ],
      "data": {
        "steps": [
          {"icon": "download", "title": "Download App", "description": "Install the free app..."},
          {"icon": "lightning", "title": "Let It Run", "description": "App runs automatically..."},
          {"icon": "wallet", "title": "Get Paid", "description": "Withdraw to crypto..."}
        ]
      }
    },
    {
      "section_type": "earnings",
      "title": "Real Earnings",
      "description": "Honest numbers from real users",
      "display_order": 3,
      "is_visible": true,
      "highlights": [],
      "images": [],
      "videos": [],
      "data": {
        "tiers": [
          {"name": "1 Device", "min_earnings": 5, "max_earnings": 8, "period": "month", "features": ["Basic tasks"], "is_popular": false},
          {"name": "2-3 Devices", "min_earnings": 15, "max_earnings": 25, "period": "month", "features": ["All tasks"], "is_popular": true},
          {"name": "5+ Devices", "min_earnings": 40, "max_earnings": 60, "period": "month", "features": ["Max earnings"], "is_popular": false}
        ],
        "disclaimer": "Earnings vary based on location and uptime."
      }
    },
    {
      "section_type": "testimonials",
      "title": "Real People. Real Earnings.",
      "description": "Join 500+ active earners",
      "display_order": 4,
      "is_visible": true,
      "highlights": [],
      "images": [],
      "videos": [],
      "data": {
        "api_endpoint": null,
        "testimonials": [
          {"quote": "Finally a passive income app that pays!", "author_name": "Raj", "author_location": "India", "rating": 5}
        ]
      }
    }
  ]
}
```

---

## Adding a New Section Type

### Step 1: Define the Schema

Add a new case in `get_section_schema()` in `home_types.rs`:

```rust
"faq" => Some(SectionSchema {
    section_type: "faq",
    label: "FAQ",
    description: "Frequently asked questions",
    icon: "help",
    color: "#ec4899",  // pink
    default_title: "Frequently Asked Questions",
    fields: vec![
        FieldDef {
            key: "questions",
            label: "Questions & Answers",
            field_type: FieldType::Repeater(vec![
                FieldDef::text("question", "Question", "How does it work?"),
                FieldDef::textarea("answer", "Answer", "Our app..."),
            ]),
            required: true,
            placeholder: "",
        },
    ],
}),
```

### Step 2: Add Default Data

Add a case in `get_default_data()`:

```rust
"faq" => serde_json::json!({
    "questions": [
        {"question": "How does it work?", "answer": "Install the app and..."},
    ]
}),
```

### Step 3: Register Section Type

Add to `get_all_section_schemas()`:

```rust
vec!["hero", "how_it_works", "earnings", "testimonials", "faq"]
    .into_iter()
    .filter_map(get_section_schema)
    .collect()
```

### Step 4: Add Validation (Optional)

Add a case in `validate_home_section()` in `home_client.rs`:

```rust
"faq" => {
    let title = section.get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if title.trim().is_empty() {
        return Err(ServerFnError::new("FAQ section requires a title"));
    }
}
```

That's it! The generic UI components will automatically render the new section type based on its schema definition.

---

## File Structure

```
src/
├── api/
│   ├── home_types.rs         # Section, SectionLink, HomePageData, SectionSchema
│   └── home_client.rs        # Server functions (CRUD)
│
├── ui/
│   ├── components/
│   │   ├── common/
│   │   │   └── icon.rs       # IconName enum with all icons
│   │   └── forms/
│   │       └── home_section_editor.rs  # All section editor components
│   │
│   └── pages/
│       └── content/
│           └── home_editor.rs  # Main editor page
```

---

## Key Benefits of This Architecture

1. **No Code Changes for New Fields**: Add fields to schemas, not components
2. **Consistent UI**: All sections use the same editor components
3. **Type Safety**: Common fields are strongly typed
4. **Flexibility**: Type-specific data uses JSON for extensibility
5. **Maintainability**: ~1150 lines vs ~1700 lines for type-specific components
6. **Reusability**: Schema system can be reused for other content types

---

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/get_home_page` | POST | Fetch home page content |
| `/api/save_home_page` | POST | Create or update home page |
| `/api/publish_home_page` | POST | Publish home page |
| `/api/get_home_page_versions` | POST | Get version history |
| `/api/revert_home_page` | POST | Revert to specific version |

---

## Frontend Integration (uno-app)

### Overview

The uno-app frontend renders home page sections dynamically from CMS data. Each section type has a dedicated component that receives CMS data as props.

### Data Flow

```
+------------------+      +------------------+      +------------------+
|   uno-admin      |      |   Database       |      |   uno-app        |
|   (CMS Editor)   | ---> |   (PostgreSQL)   | ---> |   (Frontend)     |
+------------------+      +------------------+      +------------------+
        |                         |                         |
   Edit sections            Store JSON              Fetch & render
   Save/Publish             HomePageData           section components
```

### API Integration

**Endpoint:** `GET /api/v1/items/home`

**Response:**
```json
{
  "items": [{
    "id": "uuid",
    "schema_id": "home",
    "slug": "home",
    "data": { /* HomePageData */ },
    "status": "published"
  }]
}
```

### Frontend Component Architecture

```
uno-app/src/routes/home.rs
    |
    +-- HomePage (Container)
    |     +-- Fetches CMS data on mount
    |     +-- Provides HomePageState context
    |     +-- Renders sections by display_order
    |     +-- Filters by is_visible
    |
    +-- Section Components (CMS-Driven)
          |
          +-- CmsHeroSection
          |     Props: Section
          |     Renders: title, description, highlights, images (carousel)
          |
          +-- CmsHowItWorksSection
          |     Props: Section
          |     Renders: title, description, data.steps[], links[]
          |
          +-- CmsEarningsSection
          |     Props: Section
          |     Renders: title, description, data.tiers[], data.disclaimer
          |
          +-- CmsTestimonialsSection
                Props: Section
                Renders: title, description, data.testimonials[]
```

### Custom Section Components

#### 1. CmsHeroSection

**Purpose:** Renders the hero banner with carousel, headline, and CTA.

**Props:**
```rust
#[component]
fn CmsHeroSection(section: Section) -> impl IntoView
```

**Data Mapping:**

| CMS Field | UI Element |
|-----------|------------|
| `section.title` | Hero headline |
| `section.description` | Hero subtitle |
| `section.highlights[]` | Benefit badges (checkmark + text) |
| `section.images[]` | Phone carousel slides |
| `section.links[]` | CTA buttons (if any) |

**Fallback:** Uses `t("landing.hero.*")` translations when CMS data is empty.

---

#### 2. CmsHowItWorksSection

**Purpose:** Renders the 3-step process with download buttons.

**Props:**
```rust
#[component]
fn CmsHowItWorksSection(section: Section) -> impl IntoView
```

**Data Mapping:**

| CMS Field | UI Element |
|-----------|------------|
| `section.title` | Section heading |
| `section.description` | Section subtitle |
| `section.data.steps[]` | Step cards (icon, title, description) |
| `section.links[]` | Download buttons (App Store, Google Play, APK) |

**Step Object:**
```json
{
  "icon": "download",    // Icon name: download, lightning, wallet
  "title": "Download App",
  "description": "Install the free app..."
}
```

**Link Rendering:**
```rust
// Render download buttons from CMS links
for link in section.links.iter().filter(|l| l.is_visible) {
    view! {
        <a href={&link.href} target={&link.target} rel={&link.rel} class="app-button">
            <PlatformIcon platform={&link.platform} />
            <div class="app-button-text">
                <span class="small">{&link.sublabel}</span>
                <span class="large">{&link.label}</span>
            </div>
        </a>
    }
}
```

---

#### 3. CmsEarningsSection

**Purpose:** Renders earnings tiers as pricing cards.

**Props:**
```rust
#[component]
fn CmsEarningsSection(section: Section) -> impl IntoView
```

**Data Mapping:**

| CMS Field | UI Element |
|-----------|------------|
| `section.title` | Section heading |
| `section.description` | Section subtitle |
| `section.data.tiers[]` | Earnings cards |
| `section.data.disclaimer` | Footer disclaimer text |

**Tier Object:**
```json
{
  "name": "2-3 Devices",
  "min_earnings": 15,
  "max_earnings": 25,
  "period": "month",
  "features": ["All task types enabled", "Use old phones too"],
  "is_popular": true
}
```

**Card Rendering:**
```rust
for tier in tiers {
    let amount = format!("${}-{}", tier.min_earnings, tier.max_earnings);
    let period = format!("/{}", tier.period);

    view! {
        <div class="earnings-card" class:featured={tier.is_popular}>
            {if tier.is_popular {
                view! { <div class="earnings-card-badge">"Most Popular"</div> }
            }}
            <div class="earnings-card-header">
                <span class="device-count">{&tier.name}</span>
            </div>
            <div class="earnings-card-amount">{amount}</div>
            <div class="earnings-card-period">{period}</div>
            <ul class="earnings-card-features">
                {tier.features.iter().map(|f| view! {
                    <li><CheckIcon/>{f}</li>
                }).collect_view()}
            </ul>
        </div>
    }
}
```

---

#### 4. CmsTestimonialsSection

**Purpose:** Renders testimonial carousel from CMS data.

**Props:**
```rust
#[component]
fn CmsTestimonialsSection(section: Section) -> impl IntoView
```

**Data Mapping:**

| CMS Field | UI Element |
|-----------|------------|
| `section.title` | Section heading |
| `section.description` | Section subtitle |
| `section.data.testimonials[]` | Testimonial cards |

**Testimonial Object:**
```json
{
  "quote": "Finally a passive income app that actually pays!",
  "author_name": "Raj",
  "author_location": "India",
  "author_avatar": null,
  "rating": 5
}
```

**Integration with TestimonialCarousel:**
```rust
let testimonials: Vec<TestimonialData> = section.data.testimonials
    .iter()
    .enumerate()
    .map(|(i, t)| {
        TestimonialData::new(&format!("t{}", i), &t.quote)
            .author(&t.author_name)
            .location(&t.author_location)
            .rating(t.rating)
    })
    .collect();

view! {
    <TestimonialCarousel
        testimonials=testimonials
        title={section.title.clone()}
        subtitle={section.description.clone()}
    />
}
```

---

### Section Renderer (Dynamic)

The main `HomePage` component dynamically renders sections based on CMS data:

```rust
#[component]
pub fn HomePage() -> impl IntoView {
    let sections = use_home_page_state().sections;

    view! {
        <div class="landing-page">
            <For
                each=move || {
                    let mut s = sections.get();
                    s.sort_by_key(|s| s.display_order);
                    s.into_iter().filter(|s| s.is_visible)
                }
                key=|s| s.section_type.clone()
                children=|section| {
                    match section.section_type.as_str() {
                        "hero" => view! { <CmsHeroSection section=section /> }.into_any(),
                        "how_it_works" => view! { <CmsHowItWorksSection section=section /> }.into_any(),
                        "earnings" => view! { <CmsEarningsSection section=section /> }.into_any(),
                        "testimonials" => view! { <CmsTestimonialsSection section=section /> }.into_any(),
                        _ => view! { <div>"Unknown section"</div> }.into_any(),
                    }
                }
            />
            // FAQ section (static, not in CMS yet)
            <FaqSection/>
        </div>
    }
}
```

---

### File Structure (uno-app)

```
uno-app/src/
├── routes/
│   └── home.rs              # HomePage + section components
│
├── types/
│   └── cms.rs               # Section, SectionLink, HomePageData types
│
└── components/
    └── sections/            # (optional) If extracted to separate files
        ├── mod.rs
        ├── hero.rs
        ├── how_it_works.rs
        ├── earnings.rs
        └── testimonials.rs
```

---

### Implementation Checklist

#### Phase 1: uno-admin (CMS Editor)
- [ ] Verify Home schema has correct fields (sections, not slides)
- [ ] Test creating/editing Home content
- [ ] Verify JSON structure matches spec
- [ ] Test save and publish flow

#### Phase 2: uno-app (Frontend Components)
- [ ] Add CMS types (Section, SectionLink, HomePageData)
- [ ] Create CmsHeroSection component
- [ ] Create CmsHowItWorksSection component
- [ ] Create CmsEarningsSection component
- [ ] Create CmsTestimonialsSection component

#### Phase 3: Integration
- [ ] Add HomePageState context provider
- [ ] Implement CMS data fetching
- [ ] Update HomePage to use dynamic section rendering
- [ ] Test fallback behavior when CMS unavailable
- [ ] Verify download buttons work from CMS links
