//! Referral synchronization service.
//!
//! Handles bi-directional sync of referrals between uno-admin and uno-app.
//!
//! ## Phase 8 Sync Improvements
//!
//! - **SYN-04**: Filters referrals by approval status (only syncs "active" referrals)
//! - **SYN-05**: Preserves agent commission when updating (doesn't overwrite existing values)
//! - **SYN-06**: Tombstone handling for suspended referrals (logs warnings for existing agents)

use tracing::{debug, error, info, warn};

use crate::api::uno_client::{UnoLicenseClient, ReferralInput, ReferralDto};
use crate::db::DbPool;
use crate::models::entity::{AgentEntity, NewAgent};
use crate::repository::traits::AgentRepositoryTrait;

use crate::repository::postgres::PgAgentRepository as AgentRepository;

/// Referral status constants
const STATUS_ACTIVE: &str = "active";
const STATUS_SUSPENDED: &str = "suspended";
const STATUS_PENDING: &str = "pending";

/// Default commission for new agents (percentage)
const DEFAULT_COMMISSION_PERCENT: f64 = 3.0;

/// Service for referral synchronization.
pub struct ReferralSyncService {
    agent_repo: AgentRepository,
    uno_client: UnoLicenseClient,
}

impl ReferralSyncService {
    /// Create a new ReferralSyncService.
    pub fn new(pool: DbPool) -> Result<Self, String> {
        Ok(Self {
            agent_repo: AgentRepository::new(pool),
            uno_client: UnoLicenseClient::from_env()?,
        })
    }

    /// Create with explicit client configuration.
    pub fn with_client(pool: DbPool, uno_client: UnoLicenseClient) -> Self {
        Self {
            agent_repo: AgentRepository::new(pool),
            uno_client,
        }
    }

    /// Push agents with referral codes to uno-app as referrals.
    pub async fn push_agents_as_referrals(&self) -> Result<SyncResult, String> {
        info!("Pushing agents as referrals to uno-app");

        // Get all agents with referral codes
        let agents = self.agent_repo.list_agents().await?;
        let agents_with_codes: Vec<_> = agents
            .into_iter()
            .filter(|a| a.referral_code.is_some())
            .collect();

        if agents_with_codes.is_empty() {
            info!("No agents with referral codes to sync");
            return Ok(SyncResult {
                created: 0,
                updated: 0,
                failed: 0,
                errors: vec![],
            });
        }

        info!("Found {} agents with referral codes", agents_with_codes.len());

        // Convert to ReferralInput
        let referrals: Vec<ReferralInput> = agents_with_codes
            .iter()
            .map(|a| ReferralInput {
                username: a.name.clone(),
                email: a.email.clone(),
                country_code: a.country.clone(),
                referral_code: a.referral_code.clone().unwrap_or_default(),
                status: "active".to_string(), // Agents are typically active
            })
            .collect();

        // Sync to uno-app
        let result = self.uno_client
            .sync_referrals(referrals)
            .await
            .map_err(|e| {
                error!("Failed to push referrals: {}", e);
                e.to_string()
            })?;

        info!(
            "Push result: {} created, {} updated, {} failed",
            result.created, result.updated, result.failed
        );

        Ok(SyncResult {
            created: result.created,
            updated: result.updated,
            failed: result.failed,
            errors: result.errors,
        })
    }

    /// Pull referrals from uno-app and create/update agents.
    ///
    /// ## SYN-04: Filter by approval status
    /// Only referrals with status "active" are synced as agents.
    /// Pending referrals are skipped (awaiting approval).
    ///
    /// ## SYN-06: Tombstone handling
    /// Suspended referrals trigger a warning if they have existing agents,
    /// but no agent deletion occurs (soft tombstone tracking).
    pub async fn pull_referrals_as_agents(&self) -> Result<SyncResult, String> {
        info!("Pulling referrals from uno-app as agents");

        // Fetch referrals from uno-app
        let response = self.uno_client
            .get_referrals()
            .await
            .map_err(|e| {
                error!("Failed to fetch referrals: {}", e);
                e.to_string()
            })?;

        info!("Received {} referrals from uno-app", response.referrals.len());

        let mut created = 0;
        let mut updated = 0;
        let mut failed = 0;
        let mut skipped_pending = 0;
        let mut tombstoned = 0;
        let mut errors = Vec::new();

        for referral in &response.referrals {
            // SYN-04: Filter by approval status
            match referral.status.as_str() {
                STATUS_ACTIVE => {
                    // Process active referrals
                    match self.upsert_agent_from_referral(referral).await {
                        Ok(was_created) => {
                            if was_created {
                                created += 1;
                            } else {
                                updated += 1;
                            }
                        }
                        Err(e) => {
                            failed += 1;
                            errors.push(format!("Failed to upsert agent for {}: {}", referral.referral_code, e));
                            warn!("Failed to upsert agent for {}: {}", referral.referral_code, e);
                        }
                    }
                }
                STATUS_PENDING => {
                    // Skip pending referrals (not yet approved)
                    skipped_pending += 1;
                    debug!("Skipping pending referral: {} ({})", referral.referral_code, referral.email);
                }
                STATUS_SUSPENDED => {
                    // SYN-06: Tombstone handling
                    tombstoned += 1;
                    if let Ok(Some(agent)) = self.agent_repo.get_agent_by_email(&referral.email).await {
                        warn!(
                            "Tombstone: Referral {} suspended but agent {} exists. Consider manual review.",
                            referral.referral_code, agent.id
                        );
                        errors.push(format!(
                            "Suspended referral {} has existing agent {} - manual review recommended",
                            referral.referral_code, agent.id
                        ));
                    } else {
                        debug!("Skipping suspended referral: {} (no existing agent)", referral.referral_code);
                    }
                }
                status => {
                    // Unknown status
                    warn!("Unknown referral status '{}' for {}", status, referral.referral_code);
                    skipped_pending += 1;
                }
            }
        }

        info!(
            "Pull result: {} created, {} updated, {} failed, {} pending (skipped), {} suspended (tombstoned)",
            created, updated, failed, skipped_pending, tombstoned
        );

        Ok(SyncResult {
            created,
            updated,
            failed,
            errors,
        })
    }

    /// Upsert an agent from a referral.
    /// Returns true if created, false if updated.
    ///
    /// ## SYN-05: Preserve commission
    /// When updating an existing agent, their commission_percent is preserved.
    /// Only the referral code and basic info (name, country) are synced.
    async fn upsert_agent_from_referral(&self, referral: &ReferralDto) -> Result<bool, String> {
        // Check if agent exists by email
        let existing = self.agent_repo.get_agent_by_email(&referral.email).await?;

        if let Some(mut agent) = existing {
            // SYN-05: Preserve commission - only update referral code and basic info
            let mut changed = false;

            // Update referral code if different
            if agent.referral_code.as_deref() != Some(&referral.referral_code) {
                agent.referral_code = Some(referral.referral_code.clone());
                changed = true;
            }

            // Update name if different (but preserve commission)
            if agent.name != referral.username {
                agent.name = referral.username.clone();
                changed = true;
            }

            // Update country if different
            if agent.country != referral.country_code {
                agent.country = referral.country_code.clone();
                changed = true;
            }

            // Note: commission_percent is NOT updated (SYN-05)

            if changed {
                self.agent_repo.update_agent(agent).await?;
                debug!(
                    "Updated agent {} with referral code {} (commission preserved)",
                    referral.email, referral.referral_code
                );
            }
            Ok(false)
        } else {
            // Create new agent with default commission
            let new_agent = NewAgent {
                name: referral.username.clone(),
                email: referral.email.clone(),
                country: referral.country_code.clone(),
                commission_percent: DEFAULT_COMMISSION_PERCENT,
                referral_code: Some(referral.referral_code.clone()),
            };
            self.agent_repo.save_agent(new_agent).await?;
            debug!(
                "Created agent {} with referral code {} (commission: {}%)",
                referral.email, referral.referral_code, DEFAULT_COMMISSION_PERCENT
            );
            Ok(true)
        }
    }

    /// Bi-directional sync: push then pull.
    pub async fn sync(&self) -> Result<SyncResult, String> {
        info!("Starting bi-directional referral sync");

        // Push local agents to uno-app
        let push_result = self.push_agents_as_referrals().await?;

        // Pull referrals from uno-app
        let pull_result = self.pull_referrals_as_agents().await?;

        // Combine results
        Ok(SyncResult {
            created: push_result.created + pull_result.created,
            updated: push_result.updated + pull_result.updated,
            failed: push_result.failed + pull_result.failed,
            errors: [push_result.errors, pull_result.errors].concat(),
        })
    }
}

/// Result of a sync operation.
#[derive(Debug, Clone)]
pub struct SyncResult {
    pub created: i32,
    pub updated: i32,
    pub failed: i32,
    pub errors: Vec<String>,
}

impl SyncResult {
    /// Check if the sync was fully successful.
    pub fn is_success(&self) -> bool {
        self.failed == 0
    }

    /// Total number of items processed.
    pub fn total_processed(&self) -> i32 {
        self.created + self.updated + self.failed
    }
}
