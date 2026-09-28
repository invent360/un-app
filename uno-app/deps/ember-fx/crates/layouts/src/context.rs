//! Layout context for shared state management.

use leptos::prelude::*;
use crate::types::{LayoutMode, NavPosition};

/// Layout context for managing layout state.
///
/// This context is automatically provided by `MainLayout` and can be accessed
/// anywhere in the component tree using `use_layout()`.
///
/// # Example
///
/// ```ignore
/// let ctx = use_layout();
///
/// // Toggle sidebar
/// ctx.toggle_collapsed();
///
/// // Check if mobile menu is open
/// if ctx.mobile_menu_open.get() {
///     // ...
/// }
/// ```
#[derive(Clone, Copy)]
pub struct LayoutContext {
    /// Whether the sidebar is collapsed (desktop).
    pub collapsed: RwSignal<bool>,
    /// Whether the mobile menu is open.
    pub mobile_menu_open: RwSignal<bool>,
    /// Current layout mode.
    pub mode: RwSignal<LayoutMode>,
    /// Currently selected nav key.
    pub selected_key: RwSignal<Option<String>>,
    /// Currently open submenu keys.
    pub open_keys: RwSignal<Vec<String>>,
    /// Whether icon boxes are enabled for nav items.
    pub icon_box_enabled: RwSignal<bool>,
    /// Navigation position (left, right, top, or bottom).
    pub nav_position: RwSignal<NavPosition>,
    /// Whether the settings drawer is open.
    pub settings_open: RwSignal<bool>,
    /// Whether the user is on a mobile device (based on screen width).
    pub is_mobile: RwSignal<bool>,
    /// Whether the user has manually overridden the nav position.
    pub nav_position_override: RwSignal<bool>,
}

impl LayoutContext {
    /// Create a new layout context with default values.
    #[must_use]
    pub fn new() -> Self {
        Self {
            collapsed: RwSignal::new(false),
            mobile_menu_open: RwSignal::new(false),
            mode: RwSignal::new(LayoutMode::App),
            selected_key: RwSignal::new(None),
            open_keys: RwSignal::new(Vec::new()),
            icon_box_enabled: RwSignal::new(true),
            // Default will be set based on screen size in main_layout
            nav_position: RwSignal::new(NavPosition::Left),
            settings_open: RwSignal::new(false),
            is_mobile: RwSignal::new(false),
            nav_position_override: RwSignal::new(false),
        }
    }

    /// Initialize nav position based on screen size.
    /// Called from main_layout after detecting screen size.
    pub fn init_nav_position(&self, is_mobile: bool) {
        self.is_mobile.set(is_mobile);
        // Only set default if user hasn't manually overridden
        if !self.nav_position_override.get() {
            if is_mobile {
                self.nav_position.set(NavPosition::Bottom);
            } else {
                self.nav_position.set(NavPosition::Left);
            }
        }
    }

    /// Update mobile state and adjust nav position if not overridden.
    pub fn update_mobile_state(&self, is_mobile: bool) {
        let was_mobile = self.is_mobile.get();
        self.is_mobile.set(is_mobile);

        // Only update nav position if user hasn't overridden and screen size changed
        if !self.nav_position_override.get() && was_mobile != is_mobile {
            if is_mobile {
                self.nav_position.set(NavPosition::Bottom);
            } else {
                self.nav_position.set(NavPosition::Left);
            }
        }
    }

    /// Set navigation position (marks as user override).
    pub fn set_nav_position(&self, position: NavPosition) {
        self.nav_position_override.set(true);
        self.nav_position.set(position);
    }

    /// Reset nav position to screen-size default.
    pub fn reset_nav_position(&self) {
        self.nav_position_override.set(false);
        if self.is_mobile.get() {
            self.nav_position.set(NavPosition::Bottom);
        } else {
            self.nav_position.set(NavPosition::Left);
        }
    }

    /// Open settings drawer.
    pub fn open_settings(&self) {
        self.settings_open.set(true);
    }

    /// Close settings drawer.
    pub fn close_settings(&self) {
        self.settings_open.set(false);
    }

    /// Toggle settings drawer.
    pub fn toggle_settings(&self) {
        self.settings_open.update(|o| *o = !*o);
    }

    /// Toggle icon box display.
    pub fn toggle_icon_box(&self) {
        self.icon_box_enabled.update(|e| *e = !*e);
    }

    /// Toggle sidebar collapsed state.
    pub fn toggle_collapsed(&self) {
        self.collapsed.update(|c| *c = !*c);
    }

    /// Set sidebar collapsed state.
    pub fn set_collapsed(&self, collapsed: bool) {
        self.collapsed.set(collapsed);
    }

    /// Toggle mobile menu.
    pub fn toggle_mobile_menu(&self) {
        self.mobile_menu_open.update(|o| *o = !*o);
    }

    /// Open mobile menu.
    pub fn open_mobile_menu(&self) {
        self.mobile_menu_open.set(true);
    }

    /// Close mobile menu.
    pub fn close_mobile_menu(&self) {
        self.mobile_menu_open.set(false);
    }

    /// Set layout mode.
    pub fn set_mode(&self, mode: LayoutMode) {
        self.mode.set(mode);
    }

    /// Set selected nav key.
    pub fn select(&self, key: impl Into<String>) {
        self.selected_key.set(Some(key.into()));
    }

    /// Clear selection.
    pub fn clear_selection(&self) {
        self.selected_key.set(None);
    }

    /// Toggle a submenu open/closed.
    pub fn toggle_submenu(&self, key: &str) {
        self.open_keys.update(|keys| {
            if keys.contains(&key.to_string()) {
                keys.retain(|k| k != key);
            } else {
                keys.push(key.to_string());
            }
        });
    }

    /// Open a submenu.
    pub fn open_submenu(&self, key: &str) {
        self.open_keys.update(|keys| {
            if !keys.contains(&key.to_string()) {
                keys.push(key.to_string());
            }
        });
    }

    /// Close a submenu.
    pub fn close_submenu(&self, key: &str) {
        self.open_keys.update(|keys| {
            keys.retain(|k| k != key);
        });
    }

    /// Close all submenus.
    pub fn close_all_submenus(&self) {
        self.open_keys.set(Vec::new());
    }

    /// Check if a submenu is open.
    #[must_use]
    pub fn is_submenu_open(&self, key: &str) -> bool {
        self.open_keys.get().contains(&key.to_string())
    }
}

impl Default for LayoutContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Use the layout context from anywhere in the component tree.
///
/// # Panics
///
/// Panics if `MainLayout` is not in the component tree above this call.
///
/// # Example
///
/// ```ignore
/// let ctx = use_layout();
/// ctx.toggle_collapsed();
/// ```
#[must_use]
pub fn use_layout() -> LayoutContext {
    use_context::<LayoutContext>().expect(
        "LayoutContext not found. Make sure MainLayout is in the component tree.",
    )
}

/// Try to use layout context, returning None if not available.
///
/// This is useful when a component might be used both inside and outside
/// a layout context.
///
/// # Example
///
/// ```ignore
/// if let Some(ctx) = try_use_layout() {
///     // We're inside a layout
///     ctx.toggle_collapsed();
/// }
/// ```
#[must_use]
pub fn try_use_layout() -> Option<LayoutContext> {
    use_context::<LayoutContext>()
}

/// Provide layout context to children.
///
/// This is called automatically by `MainLayout`, but can be used manually
/// if you need to provide a custom context.
pub fn provide_layout(ctx: LayoutContext) {
    provide_context(ctx);
}
