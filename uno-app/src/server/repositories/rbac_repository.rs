//! RBAC repository for database operations

use async_trait::async_trait;
use std::sync::Arc;
use crate::server::db::ConnectionPool;
use crate::types::{AppError, rbac::{Role, UserRole, UserPermissions}};

/// Dynamic type alias for RbacRepository trait object
pub type DynRbacRepository = Arc<dyn RbacRepository + Send + Sync>;

/// RBAC repository trait defining database operations
#[async_trait]
pub trait RbacRepository: Send + Sync {
    /// Get user permissions (roles and permissions)
    async fn get_user_permissions(&self, user_id: &str) -> Result<UserPermissions, AppError>;

    /// Check if user has a specific permission
    async fn has_permission(&self, user_id: &str, permission: &str) -> Result<bool, AppError>;

    /// Assign a role to a user
    async fn assign_role(&self, user_id: &str, role_name: &str, assigned_by: &str) -> Result<(), AppError>;

    /// Remove a role from a user
    async fn remove_role(&self, user_id: &str, role_name: &str) -> Result<(), AppError>;

    /// List all roles
    async fn list_roles(&self) -> Result<Vec<Role>, AppError>;

    /// Get roles for a user
    async fn get_user_roles(&self, user_id: &str) -> Result<Vec<UserRole>, AppError>;
}

/// Concrete implementation of RbacRepository
pub struct RbacRepositoryImpl {
    db_pool: ConnectionPool,
}

impl RbacRepositoryImpl {
    pub fn new(db_pool: ConnectionPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl RbacRepository for RbacRepositoryImpl {
    async fn get_user_permissions(&self, user_id: &str) -> Result<UserPermissions, AppError> {
        // Get roles
        let roles: Vec<String> = sqlx::query_scalar(
            r#"
            SELECT r.name
            FROM user_roles ur
            JOIN roles r ON r.id = ur.role_id
            WHERE ur.user_id = $1
            "#
        )
        .bind(user_id)
        .fetch_all(&self.db_pool)
        .await?;

        // Get permissions
        let permissions: Vec<String> = sqlx::query_scalar(
            r#"
            SELECT DISTINCT p.name
            FROM user_roles ur
            JOIN role_permissions rp ON rp.role_id = ur.role_id
            JOIN permissions p ON p.id = rp.permission_id
            WHERE ur.user_id = $1
            "#
        )
        .bind(user_id)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(UserPermissions {
            user_id: user_id.to_string(),
            roles,
            permissions,
        })
    }

    async fn has_permission(&self, user_id: &str, permission: &str) -> Result<bool, AppError> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM user_roles ur
            JOIN role_permissions rp ON rp.role_id = ur.role_id
            JOIN permissions p ON p.id = rp.permission_id
            WHERE ur.user_id = $1 AND p.name = $2
            "#
        )
        .bind(user_id)
        .bind(permission)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(count.0 > 0)
    }

    async fn assign_role(&self, user_id: &str, role_name: &str, assigned_by: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO user_roles (user_id, role_id, assigned_by)
            SELECT $1, r.id, $3
            FROM roles r WHERE r.name = $2
            ON CONFLICT (user_id, role_id) DO NOTHING
            "#
        )
        .bind(user_id)
        .bind(role_name)
        .bind(assigned_by)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    async fn remove_role(&self, user_id: &str, role_name: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM user_roles
            WHERE user_id = $1 AND role_id = (SELECT id FROM roles WHERE name = $2)
            "#
        )
        .bind(user_id)
        .bind(role_name)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    async fn list_roles(&self) -> Result<Vec<Role>, AppError> {
        let roles = sqlx::query_as::<_, Role>(
            "SELECT id, name, description, is_system, created_at, updated_at FROM roles ORDER BY name"
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(roles)
    }

    async fn get_user_roles(&self, user_id: &str) -> Result<Vec<UserRole>, AppError> {
        let roles = sqlx::query_as::<_, UserRole>(
            r#"
            SELECT ur.user_id, ur.role_id, r.name as role_name, ur.assigned_by, ur.assigned_at
            FROM user_roles ur
            JOIN roles r ON r.id = ur.role_id
            WHERE ur.user_id = $1
            "#
        )
        .bind(user_id)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(roles)
    }
}
