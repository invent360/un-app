//! RBAC service for permission management

use crate::server::repositories::DynRbacRepository;
use crate::types::{AppError, rbac::{Role, UserRole, UserPermissions}};

/// RBAC service for managing roles and permissions
#[derive(Clone)]
pub struct RbacServiceImpl {
    rbac_repo: DynRbacRepository,
}

impl RbacServiceImpl {
    pub fn new(rbac_repo: DynRbacRepository) -> Self {
        Self { rbac_repo }
    }

    /// Check if a user has a specific permission
    pub async fn check_permission(&self, user_id: &str, permission: &str) -> Result<bool, AppError> {
        self.rbac_repo.has_permission(user_id, permission).await
    }

    /// Require a permission, returning an error if not present
    pub async fn require_permission(&self, user_id: &str, permission: &str) -> Result<(), AppError> {
        if !self.check_permission(user_id, permission).await? {
            return Err(AppError::Forbidden(format!(
                "Missing required permission: {}", permission
            )));
        }
        Ok(())
    }

    /// Get all permissions for a user
    pub async fn get_user_permissions(&self, user_id: &str) -> Result<UserPermissions, AppError> {
        self.rbac_repo.get_user_permissions(user_id).await
    }

    /// Get roles for a user
    pub async fn get_user_roles(&self, user_id: &str) -> Result<Vec<UserRole>, AppError> {
        self.rbac_repo.get_user_roles(user_id).await
    }

    /// Assign a role to a user
    pub async fn assign_role(&self, user_id: &str, role_name: &str, assigned_by: &str) -> Result<(), AppError> {
        self.rbac_repo.assign_role(user_id, role_name, assigned_by).await
    }

    /// Remove a role from a user
    pub async fn remove_role(&self, user_id: &str, role_name: &str) -> Result<(), AppError> {
        self.rbac_repo.remove_role(user_id, role_name).await
    }

    /// List all available roles
    pub async fn list_roles(&self) -> Result<Vec<Role>, AppError> {
        self.rbac_repo.list_roles().await
    }
}
