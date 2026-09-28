//! Referral synchronization service.
//!
//! Handles bi-directional sync of referrals between uno-admin and uno-app.

use tracing::{debug, error, info, warn};

use crate::api::uno_client::{UnoLicenseClient, ReferralInput, ReferralDto};
use crate::db::DbPool;
use crate::models::entity::{AgentEntity, NewAgent};
use crate::repository::scylla::AgentRepository;
use crate::repository::traits::AgentRepositoryTrait;

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
        let mut errors = Vec::new();

        for referral in &response.referrals {
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

        info!(
            "Pull result: {} created, {} updated, {} failed",
            created, updated, failed
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
    async fn upsert_agent_from_referral(&self, referral: &ReferralDto) -> Result<bool, String> {
        // Check if agent exists by email
        let existing = self.agent_repo.get_agent_by_email(&referral.email).await?;

        if let Some(mut agent) = existing {
            // Update referral code if different
            if agent.referral_code.as_deref() != Some(&referral.referral_code) {
                agent.referral_code = Some(referral.referral_code.clone());
                self.agent_repo.update_agent(agent).await?;
                debug!("Updated agent {} with referral code {}", referral.email, referral.referral_code);
            }
            Ok(false)
        } else {
            // Create new agent
            let new_agent = NewAgent {
                name: referral.username.clone(),
                email: referral.email.clone(),
                country: referral.country_code.clone(),
                commission_percent: 3.0, // Default commission
                referral_code: Some(referral.referral_code.clone()),
            };
            self.agent_repo.save_agent(new_agent).await?;
            debug!("Created agent {} with referral code {}", referral.email, referral.referral_code);
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
