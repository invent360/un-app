//! Layout types and configuration structs.

/// Navigation item configuration.
#[derive(Clone, Debug)]
pub struct NavItem {
    /// Unique key for the item.
    pub key: String,
    /// Display label.
    pub label: String,
    /// Navigation path (None for groups).
    pub path: Option<String>,
    /// Icon SVG path data (the `d` attribute of an SVG path).
    pub icon: Option<String>,
    /// Active icon SVG path (when selected).
    pub active_icon: Option<String>,
    /// Child items (for groups/submenus).
    pub children: Vec<NavItem>,
    /// Whether the item is disabled.
    pub disabled: bool,
    /// Badge count (for notifications).
    pub badge: Option<u32>,
    /// Whether this is a danger/destructive action.
    pub danger: bool,
}

impl NavItem {
    /// Create a new navigation item.
    ///
    /// # Example
    ///
    /// ```ignore
    /// NavItem::new("dashboard", "Dashboard", "/dashboard")
    ///     .icon("<path d='M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z'/>")
    /// ```
    #[must_use]
    pub fn new(key: impl Into<String>, label: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            path: Some(path.into()),
            icon: None,
            active_icon: None,
            children: Vec::new(),
            disabled: false,
            badge: None,
            danger: false,
        }
    }

    /// Create a navigation group (parent for submenu).
    ///
    /// # Example
    ///
    /// ```ignore
    /// NavItem::group("settings", "Settings")
    ///     .icon(SETTINGS_ICON)
    ///     .children(vec![
    ///         NavItem::new("profile", "Profile", "/settings/profile"),
    ///         NavItem::new("security", "Security", "/settings/security"),
    ///     ])
    /// ```
    #[must_use]
    pub fn group(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            path: None,
            icon: None,
            active_icon: None,
            children: Vec::new(),
            disabled: false,
            badge: None,
            danger: false,
        }
    }

    /// Set the icon (SVG path data or full SVG content).
    #[must_use]
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the active icon (shown when item is selected).
    #[must_use]
    pub fn active_icon(mut self, icon: impl Into<String>) -> Self {
        self.active_icon = Some(icon.into());
        self
    }

    /// Add child items (creates submenu).
    #[must_use]
    pub fn children(mut self, children: Vec<NavItem>) -> Self {
        self.children = children;
        self
    }

    /// Set disabled state.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set badge count.
    #[must_use]
    pub fn badge(mut self, count: u32) -> Self {
        self.badge = Some(count);
        self
    }

    /// Mark as danger action.
    #[must_use]
    pub fn danger(mut self) -> Self {
        self.danger = true;
        self
    }

    /// Check if this is a group (has children).
    #[must_use]
    pub fn is_group(&self) -> bool {
        !self.children.is_empty()
    }

    /// Check if this item matches the given path.
    #[must_use]
    pub fn matches_path(&self, current_path: &str) -> bool {
        if let Some(ref path) = self.path {
            current_path == path || current_path.starts_with(&format!("{}/", path))
        } else {
            // For groups, check if any child matches
            self.children.iter().any(|child| child.matches_path(current_path))
        }
    }
}

/// Logo configuration.
#[derive(Clone, Debug, Default)]
pub struct LogoConfig {
    /// Logo text (short form like initials).
    pub icon_text: Option<String>,
    /// Full logo text.
    pub full_text: Option<String>,
    /// Logo image URL.
    pub image_url: Option<String>,
    /// Link destination when clicked.
    pub href: String,
}

impl LogoConfig {
    /// Create logo with text.
    ///
    /// # Example
    ///
    /// ```ignore
    /// LogoConfig::text("S", "Stax Board")
    /// ```
    #[must_use]
    pub fn text(icon: impl Into<String>, full: impl Into<String>) -> Self {
        Self {
            icon_text: Some(icon.into()),
            full_text: Some(full.into()),
            image_url: None,
            href: "/".to_string(),
        }
    }

    /// Create logo with image.
    ///
    /// # Example
    ///
    /// ```ignore
    /// LogoConfig::image("/logo.png")
    /// ```
    #[must_use]
    pub fn image(url: impl Into<String>) -> Self {
        Self {
            icon_text: None,
            full_text: None,
            image_url: Some(url.into()),
            href: "/".to_string(),
        }
    }

    /// Set link destination.
    #[must_use]
    pub fn href(mut self, href: impl Into<String>) -> Self {
        self.href = href.into();
        self
    }
}

/// User menu configuration.
#[derive(Clone, Debug, Default)]
pub struct UserConfig {
    /// User display name.
    pub name: Option<String>,
    /// User avatar URL.
    pub avatar_url: Option<String>,
    /// Avatar fallback text (initials).
    pub avatar_text: Option<String>,
    /// Email address.
    pub email: Option<String>,
}

impl UserConfig {
    /// Create user config.
    ///
    /// # Example
    ///
    /// ```ignore
    /// UserConfig::new("John Doe")
    ///     .avatar("/avatar.jpg")
    ///     .email("john@example.com")
    /// ```
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let avatar_text = name.chars().next().map(|c| c.to_string().to_uppercase());
        Self {
            name: Some(name),
            avatar_url: None,
            avatar_text,
            email: None,
        }
    }

    /// Set avatar URL.
    #[must_use]
    pub fn avatar(mut self, url: impl Into<String>) -> Self {
        self.avatar_url = Some(url.into());
        self
    }

    /// Set email.
    #[must_use]
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }
}

/// Layout mode for different page types.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LayoutMode {
    /// Full app layout with sidebar/header.
    #[default]
    App,
    /// Auth pages (no sidebar/header).
    Auth,
    /// Fullscreen mode (no chrome).
    Fullscreen,
}

/// Sidebar position.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarPosition {
    /// Sidebar on the left (default).
    #[default]
    Left,
    /// Sidebar on the right.
    Right,
}

/// Navigation position for the layout.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NavPosition {
    /// Sidebar on the left (default).
    #[default]
    Left,
    /// Sidebar on the right.
    Right,
    /// Horizontal navigation at the top (no sidebar).
    Top,
    /// Horizontal navigation at the bottom (no sidebar).
    Bottom,
}

impl NavPosition {
    /// Returns true if this is a horizontal nav position (top or bottom).
    pub fn is_horizontal(&self) -> bool {
        matches!(self, NavPosition::Top | NavPosition::Bottom)
    }

    /// Returns true if this is a sidebar position (left or right).
    pub fn is_sidebar(&self) -> bool {
        matches!(self, NavPosition::Left | NavPosition::Right)
    }
}

/// Mobile nav item for bottom navigation.
#[derive(Clone, Debug)]
pub struct MobileNavItem {
    /// Unique key.
    pub key: String,
    /// Display label.
    pub label: String,
    /// Navigation path.
    pub path: String,
    /// Icon SVG content.
    pub icon: String,
    /// Active icon SVG content (optional).
    pub active_icon: Option<String>,
    /// Badge count.
    pub badge: Option<u32>,
}

impl MobileNavItem {
    /// Create a mobile nav item.
    #[must_use]
    pub fn new(
        key: impl Into<String>,
        label: impl Into<String>,
        path: impl Into<String>,
        icon: impl Into<String>,
    ) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            path: path.into(),
            icon: icon.into(),
            active_icon: None,
            badge: None,
        }
    }

    /// Set active icon.
    #[must_use]
    pub fn active_icon(mut self, icon: impl Into<String>) -> Self {
        self.active_icon = Some(icon.into());
        self
    }

    /// Set badge count.
    #[must_use]
    pub fn badge(mut self, count: u32) -> Self {
        self.badge = Some(count);
        self
    }
}

/// Convert a list of NavItems to MobileNavItems (takes first 5 with paths).
#[must_use]
pub fn nav_to_mobile(nav_items: &[NavItem]) -> Vec<MobileNavItem> {
    nav_items
        .iter()
        .filter(|item| item.path.is_some())
        .take(5)
        .map(|item| MobileNavItem {
            key: item.key.clone(),
            label: item.label.clone(),
            path: item.path.clone().unwrap_or_default(),
            icon: item.icon.clone().unwrap_or_default(),
            active_icon: item.active_icon.clone(),
            badge: item.badge,
        })
        .collect()
}
