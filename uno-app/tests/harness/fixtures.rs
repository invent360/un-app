//! Test fixtures for database seeding

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

/// Test RSA private key for signing test JWTs
/// This is a TEST-ONLY key - never use in production
const TEST_RSA_PRIVATE_KEY: &str = r#"-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEAu1SU1LfVLPHCozMxH2Mo4lgOEePzNm0tRgeLezV6ffAt0gun
VTLw7onLRnrq0/IzW7yWR7QkrmBL7jTKEn5u+qKhbwKfBstIs+bMY2Zkp18gnTxk
LxUq9AQsz3L+NnOmvcLPqyHp6LJ6vHo0NNnCwv2z5zv8Y+Zg1Y7W8F5Yo3oUQsev
Xvbzck5uL2YhxQ9T2KZK4FG5eLv5JNKZbtFUvDf8LoXP5K5qfhpBdLzF84wbKmFM
VT3j+IW2OV1IHl5qGcS0Q7cKOBERJ28M8M5gJluCrojiS2PmNfD8vDy4Gfey/XJe
BpK3fvMoBqT0EU+EFXNvXhg7H2rH4CHNZ4O1NwIDAQABAoIBAC2+M2K4n/SXynLw
hXKhwmj7mEUm7RT8jL7Gg7hC8pnBCLZg5GKi8HzHO9DSeBcVPsWsEyPcJDUWqPHx
pB21G7XkWLf5fgKMvfb6D5KJ3aCxwC2gWMkP6WX0EfjC7fLk2Df8LjjCs5gC/a3T
EX6HsqWMSf0S1nFNFFEHJSCJwPf4FP0EV6o6LAOoPfnq3G1CBcJdIbLcFXR7bqA8
QTLhG9g4I5PJ3CEYJ7sJEVz1AXGV3BqEPfGBcGMr7F0aDnALhvIqAEP7M7aRH9bN
ztUl1T5CUV0fLqN2AINBQ9cbM1Fw9G2C1bhJAZU6qM2k2C0O6l2JPBj7DKDEP7s/
f3W6gWECgYEA7xHjRFzciLr8+EMt0zH4VgZFWpA2DAG5y9bJIf5P2z/lCr1e8l4a
X8MKvG8tX8nBqLcEzT7W6tRQbJAV5eNLp3A3EpgC4R9V5RQH3UfQJU7xT2vVgSaL
cF2V2X6W2k7Dz7E0nEIf7vKl7xI0PsGG8GPZs3AVV2aPGeP+4t+9/lcCgYEAyLgF
vxTF50gB3M7y1kVNpO3MDFC8ak2j/r6+hsKo+vdAkLoDo5OYmI2V1v5JNlCWHpFd
c0F0sWME3X/hTSdAhzX9kRLWhnLQ2OiXXM0E6mD7HRc7pRiMwKbNgJxKMRp6D8p7
bJFT6hGJQ0x7UhX2GJRR7iu5hk6QIeF0thXxqoECgYEA0j/WANvXpHxHQ3C8JXMV
EXAJmKCWLwkJgz7R2w1D8KC7qdwKcPO9xgcEKLnAhCx9Ms1Xq9XL2AB8G6v4BkZN
yoVYKLXJ8EqRzJ7kPLJh/JUq0P0BRMhGPq9M3c4Y8SvEGvCvCYvL3d8Gg1AX1YjV
vQoI+FCAvPQ7RqA5pOUwxSECgYEAxJjlaABbFUE8R1i7cSHIz4Q7jL8FVDV8sYbK
Y+Ah6WThpqFoB6j7yHM6xT3M8L8B5g7f8JHjRwkI0XJmNz5D3BpPw/2PmqSm3l4D
g7bLN3CDsB4GgBBgTD1K3E8R5e7l8tNU3v7M3SgKdKXpKxGkTmJ8I8m8h7jC5lX6
IgV6AgECgYBZxX0JFy/OHBsJX1LWiM1bAHm5gcoHpJwxSxAMk7F2F4N7DEjYKX0a
h0i3bFzYRrF0vJ3DBJpuLYni0kH3d+D2t0s9BbJeK5FjCW7YQFJ5pJ6X8T1GXCZ5
oSPBTu5GfI5Ng/EWS6rPlW1YD5GBpXOxQf5GZKY5qNWYZgPnFf/2oA==
-----END RSA PRIVATE KEY-----"#;

/// Test RSA public key for verifying test JWTs
const TEST_RSA_PUBLIC_KEY: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAu1SU1LfVLPHCozMxH2Mo
4lgOEePzNm0tRgeLezV6ffAt0gunVTLw7onLRnrq0/IzW7yWR7QkrmBL7jTKEn5u
+qKhbwKfBstIs+bMY2Zkp18gnTxkLxUq9AQsz3L+NnOmvcLPqyHp6LJ6vHo0NNnC
wv2z5zv8Y+Zg1Y7W8F5Yo3oUQsevXvbzck5uL2YhxQ9T2KZK4FG5eLv5JNKZbtFU
vDf8LoXP5K5qfhpBdLzF84wbKmFMVT3j+IW2OV1IHl5qGcS0Q7cKOBERJ28M8M5g
JluCrojiS2PmNfD8vDy4Gfey/XJeBpK3fvMoBqT0EU+EFXNvXhg7H2rH4CHNZ4O1
NwIDAQAB
-----END PUBLIC KEY-----"#;

/// Test key ID for JWT header
const TEST_KEY_ID: &str = "test-key-001";

/// JWT claims for test tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct TestJwtClaims {
    pub sub: String,
    pub role: String,
    pub exp: u64,
    pub iat: u64,
    pub iss: String,
    pub aud: String,
}

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

    /// Generate a test JWT token for a user
    ///
    /// This creates a valid JWT signed with the test RSA key that can be used
    /// for authenticated API requests in tests.
    pub fn generate_test_token(user_id: &str, role: &str) -> String {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let claims = TestJwtClaims {
            sub: user_id.to_string(),
            role: role.to_string(),
            iat: now,
            exp: now + 3600, // 1 hour
            iss: "test-issuer".to_string(),
            aud: "test-audience".to_string(),
        };

        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some(TEST_KEY_ID.to_string());

        let key = EncodingKey::from_rsa_pem(TEST_RSA_PRIVATE_KEY.as_bytes())
            .expect("Failed to create encoding key from test RSA key");

        encode(&header, &claims, &key).expect("Failed to encode test JWT")
    }

    /// Get the test public key configuration for session verification
    pub fn test_session_config() -> (String, String, String, String) {
        // Format: JSON map of key_id -> PEM public key
        let public_keys = serde_json::json!({
            TEST_KEY_ID: TEST_RSA_PUBLIC_KEY
        });

        (
            public_keys.to_string(),      // UNO_SESSION_PUBLIC_KEYS
            "test-issuer".to_string(),    // UNO_SESSION_ISSUER
            "test-audience".to_string(),  // UNO_SESSION_AUDIENCE
            "http://localhost:8080".to_string(), // UNO_PUBLIC_ORIGIN (will be overwritten with actual test server URL)
        )
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
