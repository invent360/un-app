//! Test fixtures for database seeding

use sqlx::PgPool;
use uuid::Uuid;

/// Test fixtures helper for seeding the database
pub struct TestFixtures {
    pool: PgPool,
}

/// A test license created by fixtures
#[derive(Debug, Clone)]
pub struct TestLicense {
    pub id: Uuid,
    pub lease_code: String,
    pub status: String,
}

/// A test user created by fixtures
#[derive(Debug, Clone)]
pub struct TestUser {
    pub id: String,
    pub external_user_id: String,
}

/// A test claim created by fixtures
#[derive(Debug, Clone)]
pub struct TestClaim {
    pub id: Uuid,
    pub license_id: Uuid,
    pub device_id: String,
}

impl TestFixtures {
    /// Create a new fixtures helper
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a test license with available status
    pub async fn create_available_license(&self, code_suffix: &str) -> TestLicense {
        let id = Uuid::new_v4();
        let lease_code = format!("TEST-{}", code_suffix);

        sqlx::query(
            r#"
            INSERT INTO licenses (id, lease_code, status, created_at, updated_at)
            VALUES ($1, $2, 'available', NOW(), NOW())
            "#,
        )
        .bind(id)
        .bind(&lease_code)
        .execute(&self.pool)
        .await
        .expect("Failed to create test license");

        TestLicense {
            id,
            lease_code,
            status: "available".to_string(),
        }
    }

    /// Create multiple test licenses
    pub async fn create_licenses(&self, count: usize) -> Vec<TestLicense> {
        let mut licenses = Vec::with_capacity(count);
        for i in 0..count {
            let license = self.create_available_license(&format!("BATCH-{}", i)).await;
            licenses.push(license);
        }
        licenses
    }

    /// Create a claimed license
    pub async fn create_claimed_license(&self, code_suffix: &str, device_id: &str) -> (TestLicense, TestClaim) {
        let id = Uuid::new_v4();
        let lease_code = format!("TEST-{}", code_suffix);

        sqlx::query(
            r#"
            INSERT INTO licenses (id, lease_code, status, created_at, updated_at)
            VALUES ($1, $2, 'claimed', NOW(), NOW())
            "#,
        )
        .bind(id)
        .bind(&lease_code)
        .execute(&self.pool)
        .await
        .expect("Failed to create test license");

        let claim_id = Uuid::new_v4();
        sqlx::query(
            r#"
            INSERT INTO claims (id, license_id, device_id, claimed_at, created_at, updated_at)
            VALUES ($1, $2, $3, NOW(), NOW(), NOW())
            "#,
        )
        .bind(claim_id)
        .bind(id)
        .bind(device_id)
        .execute(&self.pool)
        .await
        .expect("Failed to create test claim");

        let license = TestLicense {
            id,
            lease_code,
            status: "claimed".to_string(),
        };

        let claim = TestClaim {
            id: claim_id,
            license_id: id,
            device_id: device_id.to_string(),
        };

        (license, claim)
    }

    /// Create a test session
    pub async fn create_session(&self, user_id: &str) -> Uuid {
        let session_id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO sessions (id, user_id, session_token, ip_address, user_agent, created_at, expires_at, is_active)
            VALUES ($1, $2, $3, '127.0.0.1', 'test-harness', NOW(), NOW() + INTERVAL '1 hour', true)
            "#,
        )
        .bind(session_id)
        .bind(user_id)
        .bind(Uuid::new_v4().to_string())
        .execute(&self.pool)
        .await
        .expect("Failed to create test session");

        session_id
    }

    /// Create a test referral code
    pub async fn create_referral_code(&self, code: &str, user_id: &str) -> Uuid {
        let id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO referrals (id, code, user_id, is_active, created_at)
            VALUES ($1, $2, $3, true, NOW())
            "#,
        )
        .bind(id)
        .bind(code)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .expect("Failed to create test referral");

        id
    }

    /// Create a test allocation
    pub async fn create_allocation(
        &self,
        license_code: &str,
        basis_points: i32,
        role: &str,
    ) -> Uuid {
        let id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO allocations (id, license_code, allocation_bps, allocation_role, created_at)
            VALUES ($1, $2, $3, $4, NOW())
            "#,
        )
        .bind(id)
        .bind(license_code)
        .bind(basis_points)
        .bind(role)
        .execute(&self.pool)
        .await
        .expect("Failed to create test allocation");

        id
    }

    /// Create a launch gate
    pub async fn create_launch_gate(&self, name: &str, enabled: bool) -> Uuid {
        let id = Uuid::new_v4();

        // Try to update first, insert if not exists
        let result = sqlx::query(
            r#"
            UPDATE launch_gates SET is_enabled = $1, updated_at = NOW()
            WHERE gate_name = $2
            RETURNING id
            "#,
        )
        .bind(enabled)
        .bind(name)
        .fetch_optional(&self.pool)
        .await
        .expect("Failed to update launch gate");

        if result.is_none() {
            sqlx::query(
                r#"
                INSERT INTO launch_gates (id, gate_name, gate_description, is_enabled, created_at, updated_at)
                VALUES ($1, $2, $3, $4, NOW(), NOW())
                "#,
            )
            .bind(id)
            .bind(name)
            .bind(format!("Test gate: {}", name))
            .bind(enabled)
            .execute(&self.pool)
            .await
            .expect("Failed to create launch gate");
        }

        id
    }

    /// Get current license count
    pub async fn license_count(&self) -> i64 {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM licenses")
            .fetch_one(&self.pool)
            .await
            .expect("Failed to count licenses");
        count.0
    }

    /// Get current claim count
    pub async fn claim_count(&self) -> i64 {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM claims")
            .fetch_one(&self.pool)
            .await
            .expect("Failed to count claims");
        count.0
    }

    /// Delete all test data
    pub async fn cleanup(&self) {
        // Delete test data in order respecting foreign keys
        let _ = sqlx::query("DELETE FROM allocations WHERE license_code LIKE 'TEST-%'")
            .execute(&self.pool)
            .await;
        let _ = sqlx::query("DELETE FROM claims WHERE device_id LIKE 'test-device-%'")
            .execute(&self.pool)
            .await;
        let _ = sqlx::query("DELETE FROM licenses WHERE lease_code LIKE 'TEST-%'")
            .execute(&self.pool)
            .await;
        let _ = sqlx::query("DELETE FROM sessions WHERE user_id LIKE 'test-user-%'")
            .execute(&self.pool)
            .await;
        let _ = sqlx::query("DELETE FROM referrals WHERE code LIKE 'TEST-%'")
            .execute(&self.pool)
            .await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    #[actix_web::test]
    #[ignore = "requires database connection"]
    async fn test_fixtures_create_and_cleanup() {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL required");
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect(&database_url)
            .await
            .expect("Database connection required");

        let fixtures = TestFixtures::new(pool);

        // Create a test license
        let license = fixtures.create_available_license("FIXTURE-01").await;
        assert!(license.lease_code.starts_with("TEST-"));

        // Verify it exists
        let count_before = fixtures.license_count().await;
        assert!(count_before > 0);

        // Cleanup
        fixtures.cleanup().await;
    }
}
