//! Verified human identities. Machine API keys are never human login credentials.
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principal {
    pub sub: String,
    pub role: String,
    pub exp: u64,
    pub iat: u64,
    #[serde(default)]
    pub auth_time: Option<u64>,
    #[serde(default)]
    pub amr: Vec<String>,
}

/// Permissions that can be required for operations.
///
/// Note: Editorial and operator roles require MFA verification.
/// Participant and support roles do not require MFA for basic operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    // Operator permissions (full admin access)
    Operator,

    // Content management permissions
    ContentRead,
    ContentWrite,
    ContentReview,
    ContentPublish,
    ContentOverride,

    // License/claim permissions
    LicenseRead,
    LicenseClaim,
    LicenseAdmin,

    // Finance permissions
    FinanceRead,
    FinanceWrite,
    FinanceAdmin,

    // Support permissions
    SupportRead,
    SupportWrite,

    // User management
    UserRead,
    UserAdmin,

    // Agent/referral permissions
    AgentRead,
    AgentAdmin,

    // Integration/worker permissions
    WorkerExecute,
}

/// Roles that require MFA verification
const MFA_REQUIRED_ROLES: &[&str] = &[
    "operator",
    "content_author",
    "reviewer",
    "publisher",
    "finance",
    "agent",
    "country_agent",
];

impl Principal {
    /// Check if this principal has a specific permission
    pub fn require(&self, permission: Permission) -> Result<(), SessionError> {
        let (allowed, requires_mfa) = self.check_permission(permission);

        // Check MFA if required for this role/permission
        if requires_mfa && !self.amr.iter().any(|method| method == "mfa") {
            return Err(SessionError::Forbidden);
        }

        if !allowed {
            return Err(SessionError::Forbidden);
        }

        // ContentOverride requires recent authentication (15 minutes)
        if matches!(permission, Permission::ContentOverride) {
            let now = jsonwebtoken::get_current_timestamp();
            if !self
                .auth_time
                .is_some_and(|time| time <= now && now - time <= 900)
            {
                return Err(SessionError::Forbidden);
            }
        }

        Ok(())
    }

    /// Check if permission is allowed and whether MFA is required
    fn check_permission(&self, permission: Permission) -> (bool, bool) {
        let role = self.role.as_str();
        let requires_mfa = MFA_REQUIRED_ROLES.contains(&role);

        let allowed = match permission {
            // Operator has full access
            Permission::Operator | Permission::ContentOverride => role == "operator",

            // Content permissions
            Permission::ContentRead => matches!(
                role,
                "operator" | "content_author" | "reviewer" | "publisher" | "support"
            ),
            Permission::ContentWrite => matches!(role, "operator" | "content_author"),
            Permission::ContentReview => matches!(role, "operator" | "reviewer"),
            Permission::ContentPublish => matches!(role, "operator" | "publisher"),

            // License permissions
            Permission::LicenseRead => matches!(
                role,
                "operator" | "participant" | "support" | "agent" | "country_agent" | "finance"
            ),
            Permission::LicenseClaim => matches!(role, "operator" | "participant"),
            Permission::LicenseAdmin => matches!(role, "operator" | "finance"),

            // Finance permissions
            Permission::FinanceRead => matches!(role, "operator" | "finance" | "support"),
            Permission::FinanceWrite => matches!(role, "operator" | "finance"),
            Permission::FinanceAdmin => role == "operator",

            // Support permissions
            Permission::SupportRead => matches!(role, "operator" | "support" | "agent" | "country_agent"),
            Permission::SupportWrite => matches!(role, "operator" | "support"),

            // User management
            Permission::UserRead => matches!(role, "operator" | "support" | "agent" | "country_agent"),
            Permission::UserAdmin => role == "operator",

            // Agent permissions (country_agent has read access, not admin)
            Permission::AgentRead => matches!(role, "operator" | "agent" | "country_agent" | "finance"),
            Permission::AgentAdmin => matches!(role, "operator" | "finance"),

            // Worker permissions (for background jobs and integrations)
            Permission::WorkerExecute => matches!(role, "operator" | "worker" | "integration_worker"),
        };

        (allowed, requires_mfa)
    }

    /// Check if this principal has any of the specified roles
    pub fn has_role(&self, roles: &[&str]) -> bool {
        roles.contains(&self.role.as_str())
    }

    /// Check if this principal is an operator
    pub fn is_operator(&self) -> bool {
        self.role == "operator"
    }

    /// Check if this principal is a participant (end user)
    pub fn is_participant(&self) -> bool {
        self.role == "participant"
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("Authentication is not configured")]
    Configuration,
    #[error("Authentication required")]
    Missing,
    #[error("Invalid or expired session")]
    Invalid,
    #[error("Insufficient permission")]
    Forbidden,
}

#[derive(Clone)]
pub struct SessionVerifier {
    keys: HashMap<String, DecodingKey>,
    validation: Validation,
    pub origin: String,
}

impl SessionVerifier {
    pub fn from_env() -> Result<Self, SessionError> {
        let get = |name| std::env::var(name).map_err(|_| SessionError::Configuration);
        let keys: HashMap<String, String> = serde_json::from_str(&get("UNO_SESSION_PUBLIC_KEYS")?)
            .map_err(|_| SessionError::Configuration)?;
        Self::new(
            keys,
            &get("UNO_SESSION_ISSUER")?,
            &get("UNO_SESSION_AUDIENCE")?,
            &get("UNO_PUBLIC_ORIGIN")?,
        )
    }

    pub fn new(
        public_keys: HashMap<String, String>,
        issuer: &str,
        audience: &str,
        origin: &str,
    ) -> Result<Self, SessionError> {
        if public_keys.is_empty()
            || issuer.trim().is_empty()
            || audience.trim().is_empty()
            || !(origin.starts_with("https://")
                || origin.starts_with("http://localhost:")
                || origin.starts_with("http://127.0.0.1:"))
            || origin.ends_with('/')
        {
            return Err(SessionError::Configuration);
        }
        let keys = public_keys
            .into_iter()
            .map(|(id, pem)| {
                if id.trim().is_empty() {
                    return Err(SessionError::Configuration);
                }
                let key = DecodingKey::from_rsa_pem(pem.as_bytes())
                    .map_err(|_| SessionError::Configuration)?;
                Ok((id, key))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[issuer]);
        validation.set_audience(&[audience]);
        validation.set_required_spec_claims(&["exp", "iss", "aud", "sub"]);
        validation.validate_nbf = true;
        validation.leeway = 0;
        Ok(Self {
            keys,
            validation,
            origin: origin.to_string(),
        })
    }

    pub fn verify(&self, token: &str) -> Result<Principal, SessionError> {
        if token.len() > 16_384 {
            return Err(SessionError::Invalid);
        }
        let header = decode_header(token).map_err(|_| SessionError::Invalid)?;
        if header.alg != Algorithm::RS256 {
            return Err(SessionError::Invalid);
        }
        let key = header
            .kid
            .as_ref()
            .and_then(|id| self.keys.get(id))
            .ok_or(SessionError::Invalid)?;
        let principal = decode::<Principal>(token, key, &self.validation)
            .map_err(|_| SessionError::Invalid)?
            .claims;
        let now = jsonwebtoken::get_current_timestamp();
        if principal.sub.trim().is_empty()
            || principal.sub.len() > 255
            || principal.role.trim().is_empty()
            || principal.exp <= now
            || principal.iat > now.saturating_add(30)
            || principal.exp <= principal.iat
            || principal.exp - principal.iat > 3600
        {
            return Err(SessionError::Invalid);
        }
        Ok(principal)
    }
}
