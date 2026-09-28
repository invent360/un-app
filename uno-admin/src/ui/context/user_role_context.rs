//! User role context for role-based access control
//!
//! Provides context for the current user's role and permissions throughout the app.

use leptos::prelude::*;
#[cfg(feature = "ssr")]
use crate::api::ContentClient;

/// User roles in the CMS system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UserRole {
    /// Can create and edit content, but cannot publish
    #[default]
    Author,
    /// Can review content and approve/reject
    Reviewer,
    /// Can publish approved content
    Publisher,
    /// Full access to all features
    Admin,
}

impl UserRole {
    /// Get role from string
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "author" => UserRole::Author,
            "reviewer" => UserRole::Reviewer,
            "publisher" => UserRole::Publisher,
            "admin" => UserRole::Admin,
            _ => UserRole::Author,
        }
    }

    /// Get role as string
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Author => "author",
            UserRole::Reviewer => "reviewer",
            UserRole::Publisher => "publisher",
            UserRole::Admin => "admin",
        }
    }

    /// Get display name for the role
    pub fn display_name(&self) -> &'static str {
        match self {
            UserRole::Author => "Author",
            UserRole::Reviewer => "Reviewer",
            UserRole::Publisher => "Publisher",
            UserRole::Admin => "Administrator",
        }
    }

    /// Check if this role can edit content
    pub fn can_edit(&self) -> bool {
        true // All roles can edit
    }

    /// Check if this role can submit content for review
    pub fn can_submit_for_review(&self) -> bool {
        true // All roles can submit
    }

    /// Check if this role can review content
    pub fn can_review(&self) -> bool {
        matches!(self, UserRole::Reviewer | UserRole::Publisher | UserRole::Admin)
    }

    /// Check if this role can publish content
    pub fn can_publish(&self) -> bool {
        matches!(self, UserRole::Publisher | UserRole::Admin)
    }

    /// Check if this role can revert content versions
    pub fn can_revert(&self) -> bool {
        matches!(self, UserRole::Publisher | UserRole::Admin)
    }

    /// Check if this role can delete content
    pub fn can_delete(&self) -> bool {
        matches!(self, UserRole::Admin)
    }

    /// Check if this role can manage users
    pub fn can_manage_users(&self) -> bool {
        matches!(self, UserRole::Admin)
    }

    /// Check if this role can access settings
    pub fn can_access_settings(&self) -> bool {
        matches!(self, UserRole::Admin)
    }

    /// Check if this role can bulk import content
    pub fn can_bulk_import(&self) -> bool {
        matches!(self, UserRole::Publisher | UserRole::Admin)
    }
}

/// User information
#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: UserRole,
}

impl Default for CurrentUser {
    fn default() -> Self {
        Self {
            id: "admin".to_string(),
            name: "Admin User".to_string(),
            email: "admin@example.com".to_string(),
            role: UserRole::Admin, // Default to admin for development
        }
    }
}

/// User role context
#[derive(Clone, Copy)]
pub struct UserRoleContext {
    pub user: RwSignal<CurrentUser>,
    /// Permissions loaded from backend API
    pub backend_permissions: ReadSignal<Vec<String>>,
}

impl UserRoleContext {
    /// Get the current user's role
    pub fn role(&self) -> UserRole {
        self.user.get().role
    }

    /// Check if user can perform an action
    /// Uses backend permissions if available, falls back to role-based
    pub fn can(&self, permission: Permission) -> bool {
        let perms = self.backend_permissions.get();

        // If we have backend permissions, check against them
        if !perms.is_empty() {
            let perm_str = match permission {
                Permission::Edit => "content:edit",
                Permission::SubmitForReview => "content:submit_review",
                Permission::Review => "content:review",
                Permission::Publish => "content:publish",
                Permission::Revert => "content:revert",
                Permission::Delete => "content:delete",
                Permission::ManageUsers => "user:manage",
                Permission::AccessSettings => "system:settings",
                Permission::BulkImport => "content:import",
            };
            return perms.contains(&perm_str.to_string());
        }

        // Fall back to role-based permissions
        let role = self.role();
        match permission {
            Permission::Edit => role.can_edit(),
            Permission::SubmitForReview => role.can_submit_for_review(),
            Permission::Review => role.can_review(),
            Permission::Publish => role.can_publish(),
            Permission::Revert => role.can_revert(),
            Permission::Delete => role.can_delete(),
            Permission::ManageUsers => role.can_manage_users(),
            Permission::AccessSettings => role.can_access_settings(),
            Permission::BulkImport => role.can_bulk_import(),
        }
    }

    /// Set the current user
    pub fn set_user(&self, user: CurrentUser) {
        self.user.set(user);
    }

    /// Set the current user's role (for testing/development)
    pub fn set_role(&self, role: UserRole) {
        self.user.update(|u| u.role = role);
    }
}

/// Available permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    Edit,
    SubmitForReview,
    Review,
    Publish,
    Revert,
    Delete,
    ManageUsers,
    AccessSettings,
    BulkImport,
}

/// Server function to fetch user permissions from backend
#[server(GetMyPermissions, "/api")]
pub async fn get_my_permissions() -> Result<Vec<String>, ServerFnError> {
    let client = ContentClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    match client.get_my_permissions().await {
        Ok(perms) => Ok(perms.permissions),
        Err(e) => {
            // Log error but return empty permissions (default to author)
            tracing::warn!("Failed to fetch permissions: {}, defaulting to author", e);
            Ok(vec![])
        }
    }
}

/// Provide user role context to the app
#[component]
pub fn UserRoleContextProvider(children: Children) -> impl IntoView {
    let user = RwSignal::new(CurrentUser::default());
    let (backend_permissions, set_backend_permissions) = signal(Vec::<String>::new());

    // Try to load role from localStorage (for development role switching)
    #[cfg(feature = "hydrate")]
    {
        use gloo_storage::{LocalStorage, Storage};
        if let Ok(role_str) = LocalStorage::get::<String>("cms_user_role") {
            user.update(|u| u.role = UserRole::from_str(&role_str));
        }
    }

    // Fetch permissions from backend
    let _permissions_resource = Resource::new(
        || (),
        move |_| async move {
            match get_my_permissions().await {
                Ok(perms) => {
                    set_backend_permissions.set(perms.clone());
                    // Determine role from permissions
                    let role = if perms.contains(&"user:manage".to_string()) {
                        UserRole::Admin
                    } else if perms.contains(&"content:publish".to_string()) {
                        UserRole::Publisher
                    } else if perms.contains(&"content:review".to_string()) {
                        UserRole::Reviewer
                    } else {
                        UserRole::Author
                    };
                    user.update(|u| u.role = role);
                    perms
                }
                Err(_) => vec![]
            }
        }
    );

    let ctx = UserRoleContext { user, backend_permissions };
    provide_context(ctx);

    children()
}

/// Hook to get the user role context
pub fn use_user_role() -> UserRoleContext {
    expect_context::<UserRoleContext>()
}

/// Hook to check a permission
pub fn use_permission(permission: Permission) -> impl Fn() -> bool + Copy {
    let ctx = use_user_role();
    move || ctx.can(permission)
}
