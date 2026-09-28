//! Section Type Definitions
//!
//! Generic Section type with schema-driven field definitions.
//! Used across all page types: Home, FAQ, and future content types.
//! All sections share common fields (title, description, display_order, is_visible)
//! with type-specific data stored in a flexible `data` field.

use serde::{Deserialize, Serialize};

/// Schema ID for home page content
pub const HOME_PAGE_SCHEMA_ID: &str = "home";

// ============================================
// GENERIC SECTION TYPE
// ============================================

/// Generic Section - all section types use this structure
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Section {
    /// Section type identifier (hero, how_it_works, earnings, testimonials)
    pub section_type: String,

    /// Section title (common to all sections)
    #[serde(default)]
    pub title: String,

    /// Section description (common to all sections, displayed after title)
    #[serde(default)]
    pub description: String,

    /// Display order for sorting
    #[serde(default)]
    pub display_order: i32,

    /// Whether section is visible on the page
    #[serde(default = "default_true")]
    pub is_visible: bool,

    /// Optional highlights/feature bullets for the section
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub highlights: Vec<String>,

    /// Optional images for the section (URLs)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<String>,

    /// Optional videos for the section (URLs)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub videos: Vec<String>,

    /// Optional links/buttons for the section (e.g., download buttons)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<SectionLink>,

    /// Type-specific data (schema-driven)
    #[serde(default)]
    pub data: serde_json::Value,
}

// ============================================
// SECTION LINK TYPE
// ============================================

/// Platform type for download buttons
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum LinkPlatform {
    /// Apple App Store
    #[default]
    AppStore,
    /// Google Play Store
    GooglePlay,
    /// Direct APK download
    Apk,
    /// Generic/custom link
    Custom,
}

impl LinkPlatform {
    /// Get the default icon name for this platform
    pub fn icon(&self) -> &'static str {
        match self {
            Self::AppStore => "apple",
            Self::GooglePlay => "playstore",
            Self::Apk => "android",
            Self::Custom => "link",
        }
    }

    /// Get the default sublabel for this platform
    pub fn default_sublabel(&self) -> &'static str {
        match self {
            Self::AppStore => "Download on the",
            Self::GooglePlay => "Get it on",
            Self::Apk => "Direct download",
            Self::Custom => "",
        }
    }

    /// Get the default label for this platform
    pub fn default_label(&self) -> &'static str {
        match self {
            Self::AppStore => "App Store",
            Self::GooglePlay => "Google Play",
            Self::Apk => "APK File",
            Self::Custom => "Link",
        }
    }
}

/// Link/button data for sections (e.g., App Store, Google Play, APK buttons)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SectionLink {
    /// Platform type (determines icon and default labels)
    #[serde(default)]
    pub platform: LinkPlatform,

    /// Display label (e.g., "App Store", "Google Play", "APK File")
    /// If empty, uses platform default
    #[serde(default)]
    pub label: String,

    /// Small text above label (e.g., "Download on the", "Get it on")
    /// If empty, uses platform default
    #[serde(default)]
    pub sublabel: String,

    /// URL/href - the actual link destination
    pub href: String,

    /// Link target (default: "_blank" for new tab)
    #[serde(default = "default_blank")]
    pub target: String,

    /// Rel attribute (default: "noopener noreferrer" for security)
    #[serde(default = "default_rel")]
    pub rel: String,

    /// Custom icon override (if different from platform default)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Whether the link is visible
    #[serde(default = "default_true")]
    pub is_visible: bool,
}

fn default_blank() -> String {
    "_blank".to_string()
}

fn default_rel() -> String {
    "noopener noreferrer".to_string()
}

impl SectionLink {
    /// Create a new App Store link
    pub fn app_store(href: &str) -> Self {
        Self {
            platform: LinkPlatform::AppStore,
            label: "App Store".to_string(),
            sublabel: "Download on the".to_string(),
            href: href.to_string(),
            target: "_blank".to_string(),
            rel: "noopener noreferrer".to_string(),
            icon: None,
            is_visible: true,
        }
    }

    /// Create a new Google Play link
    pub fn google_play(href: &str) -> Self {
        Self {
            platform: LinkPlatform::GooglePlay,
            label: "Google Play".to_string(),
            sublabel: "Get it on".to_string(),
            href: href.to_string(),
            target: "_blank".to_string(),
            rel: "noopener noreferrer".to_string(),
            icon: None,
            is_visible: true,
        }
    }

    /// Create a new APK download link
    pub fn apk(href: &str) -> Self {
        Self {
            platform: LinkPlatform::Apk,
            label: "APK File".to_string(),
            sublabel: "Direct download".to_string(),
            href: href.to_string(),
            target: "_blank".to_string(),
            rel: "noopener noreferrer".to_string(),
            icon: None,
            is_visible: true,
        }
    }

    /// Get the effective icon (custom or platform default)
    pub fn effective_icon(&self) -> &str {
        self.icon.as_deref().unwrap_or_else(|| self.platform.icon())
    }

    /// Get the effective label (custom or platform default)
    pub fn effective_label(&self) -> &str {
        if self.label.is_empty() {
            self.platform.default_label()
        } else {
            &self.label
        }
    }

    /// Get the effective sublabel (custom or platform default)
    pub fn effective_sublabel(&self) -> &str {
        if self.sublabel.is_empty() {
            self.platform.default_sublabel()
        } else {
            &self.sublabel
        }
    }
}

fn default_true() -> bool {
    true
}

impl Section {
    /// Create a new section with empty values (no defaults)
    pub fn new(section_type: &str, order: i32) -> Self {
        Self {
            section_type: section_type.to_string(),
            title: String::new(),
            description: String::new(),
            display_order: order,
            is_visible: true,
            highlights: Vec::new(),
            images: Vec::new(),
            videos: Vec::new(),
            links: Vec::new(),
            data: serde_json::json!({}),
        }
    }

    /// Check if section has any highlights
    pub fn has_highlights(&self) -> bool {
        !self.highlights.is_empty()
    }

    /// Check if section has any links
    pub fn has_links(&self) -> bool {
        !self.links.is_empty()
    }

    /// Check if section has any images
    pub fn has_images(&self) -> bool {
        !self.images.is_empty()
    }

    /// Check if section has any videos
    pub fn has_videos(&self) -> bool {
        !self.videos.is_empty()
    }

    /// Check if section has any media (images or videos)
    pub fn has_media(&self) -> bool {
        self.has_images() || self.has_videos()
    }

    /// Get display title for UI
    pub fn display_title(&self) -> String {
        let type_label = get_section_schema(&self.section_type)
            .map(|s| s.label)
            .unwrap_or("Section");

        if !self.title.is_empty() {
            format!("{}: {}", type_label, self.title)
        } else {
            format!("{} Section", type_label)
        }
    }

    /// Get a typed field value from data
    pub fn get_string(&self, key: &str) -> String {
        self.data.get(key)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string()
    }

    /// Get a string array from data
    pub fn get_string_array(&self, key: &str) -> Vec<String> {
        self.data.get(key)
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect())
            .unwrap_or_default()
    }

    /// Get an object array from data
    pub fn get_object_array(&self, key: &str) -> Vec<serde_json::Value> {
        self.data.get(key)
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default()
    }

    /// Set a field value in data
    pub fn set_field(&mut self, key: &str, value: serde_json::Value) {
        if let Some(obj) = self.data.as_object_mut() {
            obj.insert(key.to_string(), value);
        }
    }

    /// Convert section to JSON value
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ============================================
// SECTION SCHEMA DEFINITIONS
// ============================================

/// Field types for schema definition
#[derive(Debug, Clone)]
pub enum FieldType {
    /// Single line text
    Text,
    /// Multi-line text
    TextArea,
    /// List of strings (tags/highlights)
    StringList,
    /// List of media URLs
    MediaList,
    /// Repeater (list of objects with sub-fields)
    Repeater(Vec<FieldDef>),
    /// Number input
    Number,
    /// Boolean toggle
    Boolean,
    /// Optional text (for API endpoint, etc.)
    OptionalText,
    /// Select dropdown with options (value, label)
    Select(Vec<(&'static str, &'static str)>),
}

/// Field definition for schema
#[derive(Debug, Clone)]
pub struct FieldDef {
    pub key: &'static str,
    pub label: &'static str,
    pub field_type: FieldType,
    pub required: bool,
    pub placeholder: &'static str,
}

impl FieldDef {
    pub const fn text(key: &'static str, label: &'static str, placeholder: &'static str) -> Self {
        Self { key, label, field_type: FieldType::Text, required: false, placeholder }
    }

    pub const fn textarea(key: &'static str, label: &'static str, placeholder: &'static str) -> Self {
        Self { key, label, field_type: FieldType::TextArea, required: false, placeholder }
    }

    pub const fn string_list(key: &'static str, label: &'static str) -> Self {
        Self { key, label, field_type: FieldType::StringList, required: false, placeholder: "" }
    }

    pub const fn media_list(key: &'static str, label: &'static str) -> Self {
        Self { key, label, field_type: FieldType::MediaList, required: false, placeholder: "" }
    }

    pub const fn number(key: &'static str, label: &'static str) -> Self {
        Self { key, label, field_type: FieldType::Number, required: false, placeholder: "0" }
    }

    pub const fn boolean(key: &'static str, label: &'static str) -> Self {
        Self { key, label, field_type: FieldType::Boolean, required: false, placeholder: "" }
    }

    pub const fn optional_text(key: &'static str, label: &'static str, placeholder: &'static str) -> Self {
        Self { key, label, field_type: FieldType::OptionalText, required: false, placeholder }
    }

    pub fn select(key: &'static str, label: &'static str, options: Vec<(&'static str, &'static str)>) -> Self {
        Self { key, label, field_type: FieldType::Select(options), required: false, placeholder: "" }
    }
}

/// Section schema definition
#[derive(Debug, Clone)]
pub struct SectionSchema {
    pub section_type: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    pub color: &'static str,
    pub default_title: &'static str,
    pub fields: Vec<FieldDef>,
}

// ============================================
// SECTION SCHEMAS
// ============================================

/// Get schema for a section type
pub fn get_section_schema(section_type: &str) -> Option<SectionSchema> {
    match section_type {
        "hero" => Some(SectionSchema {
            section_type: "hero",
            label: "Hero",
            description: "Main landing section with headline and image carousel",
            icon: "star",
            color: "#8b5cf6",
            default_title: "Your Phone Can Earn Money For Free",
            fields: vec![
                FieldDef::string_list("highlights", "Feature Highlights"),
            ],
        }),
        "how_it_works" => Some(SectionSchema {
            section_type: "how_it_works",
            label: "How It Works",
            description: "Step-by-step process with 3 columns",
            icon: "order",
            color: "#3b82f6",
            default_title: "How It Works",
            fields: vec![
                FieldDef {
                    key: "steps",
                    label: "Steps (3 Sub-sections)",
                    field_type: FieldType::Repeater(vec![
                        FieldDef::text("icon", "Icon", "download"),
                        FieldDef::text("title", "Step Title", "Download App"),
                        FieldDef::textarea("description", "Step Description", "Install the free app..."),
                    ]),
                    required: true,
                    placeholder: "",
                },
            ],
        }),
        "earnings" => Some(SectionSchema {
            section_type: "earnings",
            label: "Earnings",
            description: "Pricing tiers with 3 columns",
            icon: "revenue",
            color: "#22c55e",
            default_title: "Real Earnings",
            fields: vec![
                FieldDef {
                    key: "tiers",
                    label: "Earning Tiers (3 Sub-sections)",
                    field_type: FieldType::Repeater(vec![
                        FieldDef::text("name", "Tier Name", "1 Device"),
                        FieldDef::number("min_earnings", "Min Earnings"),
                        FieldDef::number("max_earnings", "Max Earnings"),
                        FieldDef::text("period", "Period", "month"),
                        FieldDef::string_list("features", "Features"),
                        FieldDef::boolean("is_popular", "Most Popular"),
                    ]),
                    required: true,
                    placeholder: "",
                },
                FieldDef::textarea("disclaimer", "Disclaimer", "Earnings vary based on..."),
            ],
        }),
        "testimonials" => Some(SectionSchema {
            section_type: "testimonials",
            label: "Testimonials",
            description: "Customer reviews (can load from API)",
            icon: "user",
            color: "#f59e0b",
            default_title: "Real People. Real Earnings.",
            fields: vec![
                FieldDef::optional_text("api_endpoint", "API Endpoint (optional)", "/api/testimonials"),
                FieldDef {
                    key: "testimonials",
                    label: "Static Testimonials (used if no API)",
                    field_type: FieldType::Repeater(vec![
                        FieldDef::textarea("quote", "Quote", "Great app..."),
                        FieldDef::text("author_name", "Author Name", "John"),
                        FieldDef::text("author_location", "Location", "USA"),
                        FieldDef::number("rating", "Rating (1-5)"),
                    ]),
                    required: false,
                    placeholder: "",
                },
            ],
        }),
        "faq" => Some(SectionSchema {
            section_type: "faq",
            label: "FAQ",
            description: "Frequently Asked Questions with categorized Q&A",
            icon: "question",
            color: "#06b6d4",
            default_title: "Frequently Asked Questions",
            fields: vec![
                FieldDef {
                    key: "questions",
                    label: "Questions",
                    field_type: FieldType::Repeater(vec![
                        FieldDef::select("category", "Category", vec![
                            ("general", "General"),
                            ("earnings", "Earnings"),
                            ("setup", "Setup"),
                            ("security", "Security"),
                        ]),
                        FieldDef::text("question", "Question", "What is UNO?"),
                        FieldDef::textarea("answer", "Answer", "UNO is a platform that..."),
                        FieldDef::boolean("is_featured", "Featured"),
                        FieldDef::boolean("is_visible", "Visible"),
                    ]),
                    required: true,
                    placeholder: "",
                },
            ],
        }),
        _ => None,
    }
}

/// Get all available section types
pub fn get_all_section_schemas() -> Vec<SectionSchema> {
    vec!["hero", "how_it_works", "earnings", "testimonials", "faq"]
        .into_iter()
        .filter_map(get_section_schema)
        .collect()
}

/// Get default data for a section type
pub fn get_default_data(section_type: &str) -> serde_json::Value {
    match section_type {
        "hero" => serde_json::json!({
            "highlights": ["100% Free", "$5-15/month", "Works in Background"],
            "images": []
        }),
        "how_it_works" => serde_json::json!({
            "steps": [
                {"order": 1, "icon": "download", "title": "Download App", "description": "Install the free app from App Store or Play Store."},
                {"order": 2, "icon": "lightning", "title": "Let It Run", "description": "App runs automatically in the background."},
                {"order": 3, "icon": "wallet", "title": "Get Paid", "description": "Withdraw to BTC, ETH, USDC anytime."}
            ]
        }),
        "earnings" => serde_json::json!({
            "tiers": [
                {"name": "1 Device", "min_earnings": 5.0, "max_earnings": 8.0, "period": "month", "features": ["Basic connection tasks", "Minimal battery usage"], "is_popular": false},
                {"name": "2-3 Devices", "min_earnings": 15.0, "max_earnings": 25.0, "period": "month", "features": ["All task types enabled", "Higher uptime bonus"], "is_popular": true},
                {"name": "5+ Devices", "min_earnings": 40.0, "max_earnings": 60.0, "period": "month", "features": ["Maximum earnings", "Priority support"], "is_popular": false}
            ],
            "disclaimer": "Earnings vary based on location, uptime, and network demand."
        }),
        "testimonials" => serde_json::json!({
            "api_endpoint": null,
            "testimonials": []
        }),
        "faq" => serde_json::json!({
            "questions": [
                {
                    "category": "general",
                    "question": "What is UNO?",
                    "answer": "UNO is a platform that allows you to earn passive income by sharing your unused internet bandwidth.",
                    "display_order": 1,
                    "is_featured": true,
                    "is_visible": true
                }
            ]
        }),
        _ => serde_json::json!({})
    }
}

// ============================================
// HOME PAGE DATA STRUCTURE
// ============================================

/// Complete home page data structure
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct HomePageData {
    pub title: String,
    pub sections: Vec<Section>,
}

impl HomePageData {
    /// Create from JSON value
    pub fn from_json(value: &serde_json::Value) -> Self {
        serde_json::from_value(value.clone()).unwrap_or_default()
    }

    /// Convert to JSON value
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    /// Get sections sorted by display_order
    pub fn sorted_sections(&self) -> Vec<&Section> {
        let mut sections: Vec<_> = self.sections.iter().collect();
        sections.sort_by_key(|s| s.display_order);
        sections
    }

    /// Get only visible sections
    pub fn visible_sections(&self) -> Vec<&Section> {
        self.sorted_sections()
            .into_iter()
            .filter(|s| s.is_visible)
            .collect()
    }
}

// ============================================
// BACKWARD COMPATIBILITY - Type aliases
// ============================================

// These type aliases maintain backward compatibility with existing code
// that might reference the old specific types

pub type HeroSectionData = Section;
pub type HowItWorksSectionData = Section;
pub type EarningsSectionData = Section;
pub type TestimonialsSectionData = Section;

// Conversion helpers for the old HomeSection enum pattern
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "section_type", rename_all = "snake_case")]
pub enum HomeSection {
    Hero(Section),
    HowItWorks(Section),
    Earnings(Section),
    Testimonials(Section),
}

impl HomeSection {
    pub fn from_json(value: &serde_json::Value) -> Option<Self> {
        let section_type = value.get("section_type")?.as_str()?;
        let section: Section = serde_json::from_value(value.clone()).ok()?;

        match section_type {
            "hero" => Some(Self::Hero(section)),
            "how_it_works" => Some(Self::HowItWorks(section)),
            "earnings" => Some(Self::Earnings(section)),
            "testimonials" => Some(Self::Testimonials(section)),
            _ => None,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Self::Hero(s) | Self::HowItWorks(s) | Self::Earnings(s) | Self::Testimonials(s) => {
                serde_json::to_value(s).unwrap_or_default()
            }
        }
    }

    pub fn section_type(&self) -> &str {
        match self {
            Self::Hero(_) => "hero",
            Self::HowItWorks(_) => "how_it_works",
            Self::Earnings(_) => "earnings",
            Self::Testimonials(_) => "testimonials",
        }
    }

    pub fn display_order(&self) -> i32 {
        match self {
            Self::Hero(s) | Self::HowItWorks(s) | Self::Earnings(s) | Self::Testimonials(s) => s.display_order,
        }
    }

    pub fn is_visible(&self) -> bool {
        match self {
            Self::Hero(s) | Self::HowItWorks(s) | Self::Earnings(s) | Self::Testimonials(s) => s.is_visible,
        }
    }

    pub fn display_title(&self) -> String {
        match self {
            Self::Hero(s) | Self::HowItWorks(s) | Self::Earnings(s) | Self::Testimonials(s) => s.display_title(),
        }
    }
}

impl Default for HomeSection {
    fn default() -> Self {
        Self::Hero(Section::new("hero", 1))
    }
}

/// Create a default section by type (backward compatibility)
pub fn create_default_section(section_type: &str, order: i32) -> Option<HomeSection> {
    let section = Section::new(section_type, order);
    match section_type {
        "hero" => Some(HomeSection::Hero(section)),
        "how_it_works" => Some(HomeSection::HowItWorks(section)),
        "earnings" => Some(HomeSection::Earnings(section)),
        "testimonials" => Some(HomeSection::Testimonials(section)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_creation() {
        let section = Section::new("hero", 1);
        assert_eq!(section.section_type, "hero");
        assert_eq!(section.display_order, 1);
        assert!(section.is_visible);
    }

    #[test]
    fn test_get_schema() {
        let schema = get_section_schema("hero");
        assert!(schema.is_some());
        let schema = schema.unwrap();
        assert_eq!(schema.label, "Hero");
    }

    #[test]
    fn test_home_page_sorting() {
        let page = HomePageData {
            title: "Test".to_string(),
            sections: vec![
                Section::new("earnings", 3),
                Section::new("hero", 1),
                Section::new("how_it_works", 2),
            ],
        };

        let sorted = page.sorted_sections();
        assert_eq!(sorted[0].section_type, "hero");
        assert_eq!(sorted[1].section_type, "how_it_works");
        assert_eq!(sorted[2].section_type, "earnings");
    }
}
