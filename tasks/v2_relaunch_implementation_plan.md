# UNO-APP v2 Relaunch Implementation Plan

**Created:** 30 September 2026
**Based on:** UNO_APP_V2_RELAUNCH_REQUIREMENTS_v2.md review
**Target:** Address all R3-xx and R4-xx tickets to achieve relaunch readiness

---

## Executive Summary

This plan addresses 27 tracked tickets (R3-01 through R3-17, R4-01 through R4-10) organized into 6 milestone gates. Each milestone has clear dependencies and exit criteria.

**Estimated scope:** ~150-200 files modified/created across 6 milestones

---

## Milestone Overview

| Gate | Name | Tickets | Dependencies | Exit Criteria |
|------|------|---------|--------------|---------------|
| A | Executable Foundation | R3-01, R3-02, R3-12, R4-01, R4-03, R4-10 | None | PostgreSQL builds, CRUD tests pass |
| B | Trusted Boundaries | R3-03, R3-04, R3-05, R4-02, R4-08 | Gate A | Auth/revocation/replay tests pass |
| C | Safe Obligations | R3-07, R3-08, R3-09, R3-10, R4-04, R4-05, R4-06 | Gate B | Settlement, jobs, backup tests pass |
| D | Correct Distribution | R3-06, R3-11 | Gate C | Import/publication/media integrated |
| E | Usable Product | R3-13, R3-14, R3-15, R3-16, R4-07, R4-09 | Gate D | Journeys, CMS, i18n complete |
| F | Operational Governance | R3-17 | Gate E | Gates, audit, retention verified |

---

## Gate A: Executable Foundation

### R3-01 — Verify PostgreSQL as the sole production database

**Priority:** P0
**Owner:** Platform/Backend
**Affected files:**
- `uno-admin/Cargo.toml`
- `uno-app/Cargo.toml`
- `.github/workflows/ci.yml`
- `uno-admin/src/db/mod.rs`
- `deployment/target.json`

**Implementation steps:**

1. **Verify feature flags** (PostgreSQL is now the only database)
   ```toml
   # uno-app/Cargo.toml - PostgreSQL via sqlx is included in ssr
   ssr = [
       # ... existing deps ...
       "dep:sqlx",
   ]
   ```

2. **Verify CI tests PostgreSQL**
   ```yaml
   # .github/workflows/ci.yml
   - name: Test admin
     run: cargo test --locked -p uno-admin

   - name: Compile worker
     run: cargo build --release -p uno-app --bin worker --features ssr
   ```

3. **Add startup validation**
   ```rust
   // uno-app/src/main.rs
   fn validate_database_config() -> Result<(), AppError> {
       // Ensure DATABASE_URL is set for PostgreSQL
       if std::env::var("DATABASE_URL").is_err() {
           return Err(AppError::ConfigError("DATABASE_URL required for PostgreSQL".into()));
       }
       Ok(())
   }
   ```

**Acceptance tests:**
- [ ] `cargo build --features ssr` compiles with PostgreSQL support
- [ ] App starts with `DATABASE_URL` set
- [ ] Worker binary compiles and starts
- [ ] CI passes all PostgreSQL tests

---

### R3-02 — Reconcile repository CRUD with migrated schema

**Priority:** P0
**Owner:** Database/Backend
**Affected files:**
- `uno-app/src/server/repositories/license_repository.rs`
- `uno-admin/src/repository/postgres/license_repository.rs`
- `uno-app/migrations/00002_licenses.up.sql`
- `uno-app/migrations/00019_admin_tables.up.sql`

**Implementation steps:**

1. **Fix portal license INSERT** - Add required columns or make nullable
   ```sql
   -- Migration to make node_id and owner_wallet_address nullable
   ALTER TABLE licenses
       ALTER COLUMN node_id DROP NOT NULL,
       ALTER COLUMN owner_wallet_address DROP NOT NULL;
   ```

2. **Fix admin repository column references**
   ```rust
   // Change licenses.license_id to licenses.id or add alias
   sqlx::query_as!(
       License,
       r#"SELECT id, lease_code, ... FROM licenses WHERE id = $1"#,
       id
   )
   ```

3. **Fix DECIMAL/f64 mapping**
   ```rust
   // Use rust_decimal::Decimal instead of f64
   use rust_decimal::Decimal;

   #[derive(sqlx::FromRow)]
   pub struct AllocationRecord {
       pub ulo_bps: i32,          // Integer basis points, not DECIMAL
       pub uno_bps: i32,
       pub referral_bps: i32,
   }
   ```

4. **Fix JSONB/String mapping**
   ```rust
   use sqlx::types::Json;

   #[derive(sqlx::FromRow)]
   pub struct NodeSettings {
       pub settings: Json<serde_json::Value>,  // Not String
   }
   ```

5. **Create reconciliation migration**
   ```sql
   -- 00051_schema_reconciliation.up.sql
   -- Ensure all repository queries have matching columns
   ```

**Acceptance tests:**
- [ ] Fresh database runs all migrations
- [ ] Upgraded database runs migrations without checksum errors
- [ ] All repository CRUD operations succeed
- [ ] No f64 used for monetary values

---

### R4-01 — Make route composition explicit and testable

**Priority:** P0
**Owner:** Backend/Platform
**Affected files:**
- `uno-app/src/main.rs`
- `uno-app/src/server/app/service_factory.rs`
- `uno-app/src/server/handlers/mod.rs`
- All handler files using `Data<Dyn...>` extractors

**Implementation steps:**

1. **Option A: Register individual Data<T> types** (Recommended)
   ```rust
   // uno-app/src/main.rs
   fn configure_app(cfg: &mut web::ServiceConfig, factory: &ServiceFactory) {
       cfg.app_data(web::Data::new(factory.clone()))
          .app_data(web::Data::from(factory.cohort_repository.clone()))
          .app_data(web::Data::from(factory.support_repository.clone()))
          .app_data(web::Data::from(factory.exit_repository.clone()))
          .app_data(web::Data::from(factory.forecast_service.clone()))
          .app_data(web::Data::from(factory.media_asset_service.clone()))
          // ... all other services
   }
   ```

2. **Option B: Change handlers to use factory** (Alternative)
   ```rust
   // Change handlers from:
   pub async fn get_cohort(
       repo: web::Data<DynCohortRepository>,
   ) -> impl Responder { ... }

   // To:
   pub async fn get_cohort(
       factory: web::Data<ServiceFactory>,
   ) -> impl Responder {
       let repo = &factory.cohort_repository;
       ...
   }
   ```

3. **Fix path/handler mismatches**
   ```rust
   // uno-app/src/server/handlers/support_handler.rs
   // Change from literal route to parameterized
   // Before: .route("/support/my-tickets", ...)
   // After: .route("/support/tickets/{user_id}", ...)
   ```

4. **Add startup validation**
   ```rust
   fn validate_route_composition(cfg: &ServiceConfig) -> Result<(), AppError> {
       // Verify all required Data<T> types are registered
   }
   ```

**Acceptance tests:**
- [ ] All routes respond without extraction errors
- [ ] Missing dependency causes startup failure
- [ ] Route inventory matches handler signatures

---

### R4-03 — Repair SQL contracts and retention evidence

**Priority:** P0
**Owner:** Database/Backend/Analytics
**Affected files:**
- `uno-app/src/server/repositories/cohort_repository.rs`
- `uno-app/migrations/00042_cohort_tracking.up.sql`
- `uno-app/migrations/00044_voluntary_exit.up.sql`

**Implementation steps:**

1. **Fix cohort_repository column mismatches**
   ```rust
   // Current (wrong):
   INSERT INTO daily_activity (day_number, data_collected_bytes, recorded_at)

   // Should be (matching migration):
   INSERT INTO daily_activity (activity_date, data_shared_bytes, created_at)
   ```

2. **Fix calculate_exit_balance function**
   ```sql
   -- Migration fix for 00044
   CREATE OR REPLACE FUNCTION calculate_exit_balance(p_license_id UUID)
   RETURNS TABLE (balance BIGINT) AS $$
   SELECT COALESCE(SUM(ulo_micros), 0) as balance  -- Not ulo_amount_micros
   FROM allocation_ledger
   WHERE license_id = p_license_id  -- Not user_id
   $$;
   ```

3. **Add idempotent activity recording**
   ```rust
   // Make replay safe - use UPSERT with proper conflict handling
   sqlx::query!(
       r#"
       INSERT INTO daily_activity (license_id, activity_date, ...)
       VALUES ($1, $2, ...)
       ON CONFLICT (license_id, activity_date)
       DO UPDATE SET ...
       WHERE daily_activity.event_id < EXCLUDED.event_id  -- Idempotency
       "#
   )
   ```

4. **Derive D7 from reward events**
   ```rust
   pub async fn check_d7_qualification(&self, license_id: Uuid) -> Result<bool, AppError> {
       let rewarded_days = sqlx::query_scalar!(
           r#"
           SELECT COUNT(DISTINCT DATE(reward_date))
           FROM allocation_ledger
           WHERE license_id = $1
             AND reward_date >= NOW() - INTERVAL '7 days'
             AND ulo_micros > 0
           "#,
           license_id
       ).fetch_one(&self.pool).await?;

       Ok(rewarded_days >= 4)  // 4 of 7 days
   }
   ```

**Acceptance tests:**
- [ ] Activity recording is idempotent
- [ ] D7 derived from 4 reward days in 7
- [ ] Exit balance calculation works
- [ ] Schema and queries match exactly

---

### R3-12 / R4-10 — Exact-release tests and CI evidence

**Priority:** P0
**Owner:** QA/Platform
**Affected files:**
- `.github/workflows/ci.yml`
- `uno-app/tests/phase0_http.rs`
- `uno-app/tests/phase9_acceptance.rs`
- `uno-app/tests/harness/mod.rs`

**Implementation steps:**

1. **Fix phase0_http claim assertion**
   ```rust
   // Current (wrong): expects 503 for unauthenticated
   // Should be: expects 401 for unauthenticated, 503 for authenticated but paused

   #[actix_web::test]
   async fn claim_without_auth_returns_401() {
       let response = client.post("/api/v1/licenses/claim", &json!({...})).await;
       assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
   }

   #[actix_web::test]
   async fn claim_while_paused_returns_503() {
       let response = authenticated_client.post("/api/v1/licenses/claim", &json!({...})).await;
       assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
   }
   ```

2. **Add phase targets to CI**
   ```yaml
   # .github/workflows/ci.yml
   integration-tests:
     services:
       postgres:
         image: postgres:16
         env:
           POSTGRES_PASSWORD: test
     steps:
       - name: Run Phase 9 acceptance tests
         env:
           DATABASE_URL: postgres://postgres:test@localhost/test
         run: |
           cargo test --locked -p uno-app --features ssr \
             --test phase9_acceptance -- --ignored
   ```

3. **Replace placeholder concurrent tests**
   ```rust
   // uno-app/tests/phase4_concurrent_claims.rs
   // Replace placeholder with real implementation
   #[tokio::test]
   async fn concurrent_claims_respect_capacity() {
       let harness = TestHarness::new().await;

       // Create 10 licenses
       for i in 0..10 {
           harness.fixtures.create_available_license(&format!("CONC-{:02}", i)).await;
       }

       // Launch 100 concurrent claim attempts
       let handles: Vec<_> = (0..100).map(|i| {
           let client = harness.client.clone();
           tokio::spawn(async move {
               client.post("/api/v1/licenses/claim", &json!({
                   "device_id": format!("device-{}", i)
               })).await
           })
       }).collect();

       let results = futures::future::join_all(handles).await;
       let successes = results.iter().filter(|r| r.as_ref().unwrap().status().is_success()).count();

       assert_eq!(successes, 10, "Exactly 10 claims should succeed");
   }
   ```

**Acceptance tests:**
- [ ] CI runs PostgreSQL integration tests
- [ ] Worker binary compiles in CI
- [ ] Phase 9 tests execute (not just compile)
- [ ] Failed test blocks promotion

---

## Gate B: Trusted Boundaries

### R3-03 — Unify identity, session revocation and authorization

**Priority:** P0
**Owner:** Security/Backend
**Affected files:**
- `uno-api/src/auth/session.rs`
- `uno-api/src/auth/web.rs`
- `uno-app/src/server/extractors/auth.rs`
- `uno-app/src/server/services/session_service.rs`
- `uno-app/src/server/middleware/auth_middleware.rs`

**Implementation steps:**

1. **Create centralized authorization boundary**
   ```rust
   // uno-app/src/server/auth/authorization.rs
   pub struct AuthorizationContext {
       pub principal: Principal,
       pub session: Option<Session>,
       pub permissions: Vec<Permission>,
   }

   impl AuthorizationContext {
       pub async fn from_request(
           req: &HttpRequest,
           session_service: &SessionService,
       ) -> Result<Self, AuthError> {
           let token = extract_bearer_token(req)?;
           let claims = verify_jwt(&token)?;

           // Check session revocation
           if let Some(session_id) = claims.session_id {
               let session = session_service.get_session(session_id).await?;
               if session.is_none() || session.unwrap().revoked {
                   return Err(AuthError::SessionRevoked);
               }
           }

           // Check account status
           let account_status = session_service.get_account_status(&claims.subject).await?;
           if account_status != AccountStatus::Active {
               return Err(AuthError::AccountSuspended);
           }

           Ok(Self { ... })
       }
   }
   ```

2. **Fail closed on database errors**
   ```rust
   // Change from fail-open to fail-closed
   pub async fn validate_session(&self, session_id: Uuid) -> Result<bool, AppError> {
       match self.repo.get_session(session_id).await {
           Ok(Some(session)) => Ok(!session.revoked && session.expires_at > Utc::now()),
           Ok(None) => Ok(false),  // Unknown session = invalid
           Err(e) => {
               tracing::error!("Session validation failed: {}", e);
               Err(AppError::ServiceUnavailable("Auth service unavailable".into()))
           }
       }
   }
   ```

3. **Define provider subject mapping**
   ```rust
   // uno-app/src/server/services/identity_service.rs
   pub struct IdentityService {
       // Map (issuer, subject) to local UUID
       pub async fn resolve_or_create_identity(
           &self,
           issuer: &str,
           subject: &str,
       ) -> Result<Uuid, AppError> {
           // Check for existing mapping
           if let Some(identity) = self.repo.find_by_provider(issuer, subject).await? {
               return Ok(identity.local_id);
           }

           // Create new identity
           let local_id = Uuid::new_v4();
           self.repo.create_identity(local_id, issuer, subject).await?;
           Ok(local_id)
       }
   }
   ```

4. **Implement logout-all**
   ```rust
   pub async fn logout_all(&self, user_id: Uuid) -> Result<(), AppError> {
       // Revoke all sessions
       self.repo.revoke_all_sessions(user_id).await?;

       // Update valid_after timestamp
       self.repo.update_valid_after(user_id, Utc::now()).await?;

       Ok(())
   }
   ```

**Acceptance tests:**
- [ ] Logout denies next request
- [ ] Account suspension denies access
- [ ] Database outage returns 503, not success
- [ ] Wrong owner/role is denied
- [ ] JWT tampering fails validation

---

### R3-04 — Consume nonces in machine boundary

**Priority:** P0
**Owner:** API/Security
**Affected files:**
- `uno-app/src/server/repositories/nonce_repository.rs`
- `uno-api/src/auth/hmac.rs`
- `uno-app/src/server/handlers/review_admin_handler.rs`

**Implementation steps:**

1. **Wire nonce repository into HMAC verification**
   ```rust
   // uno-api/src/auth/hmac.rs
   pub async fn verify_signed_request(
       &self,
       req: &SignedRequest,
       nonce_repo: &DynNonceRepository,
   ) -> Result<(), AuthError> {
       // Verify signature first
       self.verify_signature(req)?;

       // Check timestamp within window
       let now = Utc::now();
       let request_time = req.timestamp;
       if (now - request_time).num_seconds().abs() > 300 {
           return Err(AuthError::RequestExpired);
       }

       // Check and consume nonce
       let consumed = nonce_repo.try_consume(&req.nonce, request_time).await?;
       if !consumed {
           return Err(AuthError::NonceReused);
       }

       Ok(())
   }
   ```

2. **Sign method and path in request**
   ```rust
   pub struct SignedRequest {
       pub method: String,
       pub path: String,
       pub query: String,
       pub body_hash: String,
       pub timestamp: DateTime<Utc>,
       pub nonce: String,
       pub client_id: String,
       pub signature: String,
   }

   impl SignedRequest {
       pub fn canonical_string(&self) -> String {
           format!(
               "{}\n{}\n{}\n{}\n{}\n{}",
               self.method,
               self.path,
               self.query,
               self.body_hash,
               self.timestamp.timestamp(),
               self.nonce
           )
       }
   }
   ```

3. **Share nonce state across replicas**
   ```sql
   -- Nonces stored in PostgreSQL, shared across replicas
   CREATE TABLE consumed_nonces (
       nonce VARCHAR(64) PRIMARY KEY,
       consumed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
       client_id VARCHAR(100) NOT NULL
   );

   -- Cleanup old nonces (retention > replay window)
   CREATE INDEX idx_nonces_consumed_at ON consumed_nonces(consumed_at);
   ```

**Acceptance tests:**
- [ ] Valid signed request succeeds
- [ ] Wrong method/path fails
- [ ] Same nonce replay fails
- [ ] Expired timestamp fails
- [ ] Two processes cannot replay same request

---

### R3-05 — Complete secure issuance before removing pause

**Priority:** P0
**Owner:** Distribution/Backend/Frontend
**Affected files:**
- `uno-app/src/server/services/license_service.rs`
- `uno-app/src/server/services/reservation_service.rs`
- `uno-app/src/server/services/ownership_service.rs`
- `uno-app/src/server/handlers/licenses_handler.rs`

**Implementation steps:**

1. **Create unified issuance transaction**
   ```rust
   pub async fn issue_license(
       &self,
       request: IssuanceRequest,
       principal: &Principal,
   ) -> Result<IssuanceResult, AppError> {
       let mut tx = self.pool.begin().await?;

       // 1. Verify ownership and session
       self.verify_owner(&request.reservation_id, principal, &mut tx).await?;

       // 2. Check consent
       self.verify_consent(principal.user_id, &mut tx).await?;

       // 3. Check eligibility (market, device, task)
       self.verify_eligibility(&request, &mut tx).await?;

       // 4. Check offer/publication state
       let offer = self.verify_offer_state(&request.offer_id, &mut tx).await?;

       // 5. Lock and verify capacity
       let license = sqlx::query_as!(
           License,
           r#"
           SELECT * FROM licenses
           WHERE id = $1 AND status = 'reserved'
           FOR UPDATE NOWAIT
           "#,
           request.license_id
       ).fetch_one(&mut *tx).await?;

       // 6. Check owner matches
       if license.reserved_by != Some(principal.user_id) {
           return Err(AppError::Forbidden("Not reservation owner".into()));
       }

       // 7. Revalidate dates
       if license.reservation_expires_at < Utc::now() {
           return Err(AppError::ValidationError("Reservation expired".into()));
       }

       // 8. Issue credential
       let credential = self.generate_credential(&license)?;

       // 9. Update license status
       sqlx::query!(
           "UPDATE licenses SET status = 'issued', issued_at = NOW() WHERE id = $1",
           license.id
       ).execute(&mut *tx).await?;

       // 10. Record issuance event
       sqlx::query!(
           "INSERT INTO issuance_events (license_id, user_id, issued_at) VALUES ($1, $2, NOW())",
           license.id, principal.user_id
       ).execute(&mut *tx).await?;

       // 11. Create outbox event
       self.outbox_repo.enqueue(&mut tx, OutboxEvent::LicenseIssued { ... }).await?;

       tx.commit().await?;

       Ok(IssuanceResult { credential, ... })
   }
   ```

2. **Freeze attribution at reservation**
   ```rust
   pub async fn create_reservation(
       &self,
       request: ReservationRequest,
       principal: &Principal,
   ) -> Result<Reservation, AppError> {
       let mut tx = self.pool.begin().await?;

       // Get first qualified referral from server state
       let referral = self.get_qualified_referral(principal.user_id, &mut tx).await?;

       // Lock license and create reservation atomically
       let reservation = sqlx::query_as!(
           Reservation,
           r#"
           INSERT INTO reservations (license_id, user_id, referral_id, agreement_id, expires_at)
           SELECT l.id, $1, $2, $3, NOW() + INTERVAL '15 minutes'
           FROM licenses l
           WHERE l.id = $4 AND l.status = 'available'
           FOR UPDATE SKIP LOCKED
           RETURNING *
           "#,
           principal.user_id,
           referral.map(|r| r.id),  // Frozen at reservation time
           request.agreement_id,
           request.license_id
       ).fetch_one(&mut *tx).await?;

       tx.commit().await?;
       Ok(reservation)
   }
   ```

3. **Enforce capacity ceiling**
   ```rust
   const MAX_OCCUPIED_LICENSES: i64 = 2500;

   pub async fn check_capacity(&self) -> Result<bool, AppError> {
       let occupied = sqlx::query_scalar!(
           r#"
           SELECT COUNT(*) FROM licenses
           WHERE status IN ('reserved', 'issued', 'active', 'release_pending')
           "#
       ).fetch_one(&self.pool).await?;

       Ok(occupied.unwrap_or(0) < MAX_OCCUPIED_LICENSES)
   }
   ```

**Acceptance tests:**
- [ ] 100 concurrent clients get max 10 licenses
- [ ] Wrong owner cannot claim another's reservation
- [ ] Expired reservation cannot be confirmed
- [ ] Attribution cannot be changed after reservation
- [ ] Capacity ceiling enforced

---

### R4-02 — Secure all new participant and operator interfaces

**Priority:** P0
**Owner:** Security/Backend
**Affected files:**
- `uno-app/src/server/handlers/support_handler.rs`
- `uno-app/src/server/handlers/cohort_handler.rs`
- `uno-app/src/server/handlers/dashboard_handler.rs`
- `uno-app/src/server/handlers/exit_handler.rs`
- `uno-app/src/server/handlers/finance_handler.rs`

**Implementation steps:**

1. **Derive user/actor from principal**
   ```rust
   // Before (insecure):
   pub async fn create_ticket(
       body: web::Json<CreateTicketRequest>,  // Contains user_id
   ) -> impl Responder { ... }

   // After (secure):
   pub async fn create_ticket(
       auth: AuthContext,  // Derives user from verified token
       body: web::Json<CreateTicketRequest>,
   ) -> impl Responder {
       let user_id = auth.principal.user_id;  // From token, not body
       ...
   }
   ```

2. **Verify ownership in queries**
   ```rust
   pub async fn get_dashboard(
       auth: AuthContext,
   ) -> impl Responder {
       // Query enforces ownership
       let dashboard = sqlx::query_as!(
           DashboardData,
           r#"
           SELECT * FROM participant_dashboard
           WHERE user_id = $1  -- Only owner's data
           "#,
           auth.principal.user_id
       ).fetch_optional(&pool).await?;

       ...
   }
   ```

3. **Replace X-Admin-Id header with principal**
   ```rust
   // Before (insecure):
   let actor = req.headers()
       .get("X-Admin-Id")
       .map(|h| h.to_str().unwrap_or("admin"))
       .unwrap_or("admin");

   // After (secure):
   let actor = auth.principal.user_id.to_string();
   ```

4. **Scope agent access to country/assignment**
   ```rust
   pub async fn get_agent_queue(
       auth: AuthContext,
       repo: web::Data<DynSupportRepository>,
   ) -> impl Responder {
       // Verify agent role
       auth.require_role(Role::Agent)?;

       // Get only assigned country's tickets
       let agent = repo.get_agent(auth.principal.user_id).await?;
       let tickets = repo.get_tickets_by_country(agent.country_code).await?;

       ...
   }
   ```

5. **Separate finance permissions**
   ```rust
   pub async fn execute_settlement(
       auth: AuthContext,
   ) -> impl Responder {
       // Require specific permission
       auth.require_permission(Permission::FinanceExecute)?;

       // Check preparer != executor (separation of duties)
       let settlement = repo.get_settlement(settlement_id).await?;
       if settlement.prepared_by == auth.principal.user_id {
           return Err(AppError::Forbidden("Cannot execute own preparation".into()));
       }

       ...
   }
   ```

**Acceptance tests:**
- [ ] Participant A cannot see participant B's dashboard
- [ ] Agent cannot access other country's tickets
- [ ] Client-supplied user_id is ignored
- [ ] Finance prepare/execute requires different users
- [ ] Missing auth returns 401, not 500

---

### R4-08 — Make communication and webhooks truthful

**Priority:** P0
**Owner:** Integration/Security
**Affected files:**
- `uno-app/src/server/services/webhook_service.rs`
- `uno-app/src/server/services/communication_service.rs`

**Implementation steps:**

1. **Fail on unconfigured providers**
   ```rust
   pub enum DeliveryResult {
       Sent { provider_id: String, message_id: String },
       Disabled { reason: String },
       Failed { error: String },
   }

   impl NullEmailProvider {
       pub async fn send(&self, _: &Email) -> DeliveryResult {
           // Don't return success for null provider
           DeliveryResult::Disabled {
               reason: "Email provider not configured".into()
           }
       }
   }
   ```

2. **Require known inbound source**
   ```rust
   pub async fn process_webhook(
       req: HttpRequest,
       body: web::Bytes,
       config: web::Data<WebhookConfig>,
   ) -> impl Responder {
       // Verify source is configured
       let source = req.headers().get("X-Webhook-Source")
           .and_then(|h| h.to_str().ok())
           .ok_or(AppError::Unauthorized("Missing source".into()))?;

       let source_config = config.sources.get(source)
           .ok_or(AppError::Unauthorized("Unknown source".into()))?;

       // Verify signature
       let signature = req.headers().get("X-Signature")
           .ok_or(AppError::Unauthorized("Missing signature".into()))?;

       verify_hmac_signature(&body, signature, &source_config.secret)?;

       // Verify timestamp
       let timestamp = req.headers().get("X-Timestamp")
           .and_then(|h| h.to_str().ok())
           .and_then(|s| s.parse::<i64>().ok())
           .ok_or(AppError::Unauthorized("Invalid timestamp".into()))?;

       let request_time = DateTime::from_timestamp(timestamp, 0)
           .ok_or(AppError::Unauthorized("Invalid timestamp".into()))?;

       if (Utc::now() - request_time).num_seconds().abs() > 300 {
           return Err(AppError::Unauthorized("Request expired".into()));
       }

       // Process with idempotency
       ...
   }
   ```

3. **Implement consent propagation**
   ```rust
   pub async fn handle_opt_out(
       &self,
       user_id: Uuid,
   ) -> Result<(), AppError> {
       // Update local consent
       self.consent_repo.revoke_marketing_consent(user_id).await?;

       // Cancel pending messages
       self.message_queue.cancel_for_user(user_id).await?;

       // Propagate to CRM if configured
       if let Some(crm) = &self.crm_client {
           crm.update_consent(user_id, ConsentStatus::OptedOut).await?;
       }

       Ok(())
   }
   ```

**Acceptance tests:**
- [ ] Unknown webhook source returns 401
- [ ] Invalid signature returns 401
- [ ] Replay returns 409 (already processed)
- [ ] Null provider returns disabled, not success
- [ ] Opt-out cancels queued messages

---

## Gate C: Safe Obligations

### R3-07 — Unify agents, referrals and 50/40/10 agreement

**Priority:** P0
**Owner:** Finance/Backend
**Affected files:**
- `uno-app/src/server/services/agent_service.rs`
- `uno-api/src/models/license.rs`
- `uno-api/src/models/revenue_split.rs`

**Implementation steps:**

1. **Replace 3% default with explicit agreement**
   ```rust
   pub struct RevenueAgreement {
       pub ulo_bps: i32,       // 5000 = 50%
       pub uno_bps: i32,       // 4000 = 40%
       pub referral_bps: i32,  // 1000 = 10%
   }

   impl RevenueAgreement {
       pub const V2_STANDARD: Self = Self {
           ulo_bps: 5000,
           uno_bps: 4000,
           referral_bps: 1000,
       };

       pub fn validate(&self) -> Result<(), ValidationError> {
           if self.ulo_bps + self.uno_bps + self.referral_bps != 10000 {
               return Err(ValidationError::InvalidSplit);
           }
           Ok(())
       }
   }
   ```

2. **Handle no-referral case with reserve**
   ```rust
   impl RevenueSplit {
       pub fn without_referral(pool: Decimal, agreement: &RevenueAgreement) -> Self {
           let ulo = pool * Decimal::from(agreement.ulo_bps) / Decimal::from(10000);
           let uno = pool * Decimal::from(agreement.uno_bps) / Decimal::from(10000);
           let reserve = pool * Decimal::from(agreement.referral_bps) / Decimal::from(10000);

           Self {
               ulo_amount: ulo,
               uno_amount: uno,
               referral_amount: Decimal::ZERO,
               reserve_amount: reserve,  // Goes to reserve, not UNO
           }
       }
   }
   ```

3. **Enforce agent suspension**
   ```rust
   pub async fn suspend_agent(
       &self,
       agent_id: Uuid,
       reason: &str,
       actor: &str,
   ) -> Result<(), AppError> {
       let mut tx = self.pool.begin().await?;

       // Update status with history
       sqlx::query!(
           r#"
           UPDATE agents
           SET status = 'suspended',
               suspended_at = NOW(),
               suspended_reason = $1
           WHERE id = $2
           "#,
           reason, agent_id
       ).execute(&mut *tx).await?;

       // Record history
       sqlx::query!(
           "INSERT INTO agent_status_history (agent_id, old_status, new_status, reason, actor) VALUES ($1, 'active', 'suspended', $2, $3)",
           agent_id, reason, actor
       ).execute(&mut *tx).await?;

       // Prevent reactivation through sync
       sqlx::query!(
           "INSERT INTO sync_overrides (entity_type, entity_id, field, override_value) VALUES ('agent', $1, 'status', 'suspended')",
           agent_id
       ).execute(&mut *tx).await?;

       tx.commit().await?;
       Ok(())
   }
   ```

**Acceptance tests:**
- [ ] Suspended agent cannot get new attributions
- [ ] Legacy 3% contracts preserved as history
- [ ] New offers use 50/40/10
- [ ] No-referral goes to reserve, not UNO

---

### R3-08 — Make finance authoritative

**Priority:** P0
**Owner:** Finance/Backend
**Affected files:**
- `uno-app/src/server/services/settlement_service.rs`
- `uno-app/src/server/repositories/allocation_repository.rs`

**Implementation steps:**

1. **Use integer basis points**
   ```rust
   #[derive(Debug, Clone)]
   pub struct AllocationRecord {
       pub id: Uuid,
       pub license_id: Uuid,
       pub reward_event_id: String,  // Provider's unique ID
       pub pool_micros: i64,         // Integer micros
       pub ulo_micros: i64,
       pub uno_micros: i64,
       pub referral_micros: i64,
       pub reserve_micros: i64,
       pub agreement_id: Uuid,
       pub created_at: DateTime<Utc>,
   }
   ```

2. **Prevent duplicate rewards**
   ```sql
   -- Unique constraint on provider event
   CREATE UNIQUE INDEX idx_allocations_provider_event
   ON allocation_ledger(provider_id, reward_event_id);
   ```

3. **Separate deletion from financial history**
   ```rust
   pub async fn delete_user_data(
       &self,
       user_id: Uuid,
   ) -> Result<DeletionResult, AppError> {
       let mut tx = self.pool.begin().await?;

       // Delete personal data
       sqlx::query!("DELETE FROM user_profiles WHERE user_id = $1", user_id)
           .execute(&mut *tx).await?;

       // Anonymize but preserve financial records
       sqlx::query!(
           r#"
           UPDATE allocation_ledger
           SET user_id = NULL, anonymized_at = NOW()
           WHERE user_id = $1
           "#,
           user_id
       ).execute(&mut *tx).await?;

       // Keep earned entitlements
       // DO NOT delete allocation_ledger rows

       tx.commit().await?;
       Ok(DeletionResult::Anonymized)
   }
   ```

**Acceptance tests:**
- [ ] Duplicate reward event rejected
- [ ] User deletion preserves financial records
- [ ] Integer arithmetic, no floating point
- [ ] Reversals reconcile all parties

---

### R4-04 — Settle liabilities per recipient and party

**Priority:** P0
**Owner:** Finance/Backend
**Affected files:**
- `uno-app/src/server/services/settlement_service.rs`

**Implementation steps:**

1. **Fix prepare_settlement duplicate detection**
   ```rust
   pub async fn prepare_settlement(
       &self,
       allocation_ids: Vec<Uuid>,
       actor: &str,
   ) -> Result<PreparedSettlement, AppError> {
       // Deduplicate and validate
       let unique_ids: HashSet<_> = allocation_ids.iter().collect();
       if unique_ids.len() != allocation_ids.len() {
           return Err(AppError::ValidationError("Duplicate allocation IDs".into()));
       }

       let mut tx = self.pool.begin().await?;

       // Lock and validate each allocation
       let allocations = sqlx::query_as!(
           Allocation,
           r#"
           SELECT * FROM allocation_ledger
           WHERE id = ANY($1)
           AND status = 'payable'
           FOR UPDATE NOWAIT
           "#,
           &allocation_ids
       ).fetch_all(&mut *tx).await?;

       // Validate same currency and recipient
       let currencies: HashSet<_> = allocations.iter().map(|a| &a.currency).collect();
       if currencies.len() > 1 {
           return Err(AppError::ValidationError("Mixed currencies".into()));
       }

       // Create settlement items per party
       for allocation in &allocations {
           // ULO item
           if allocation.ulo_micros > 0 {
               sqlx::query!(
                   "INSERT INTO settlement_items (settlement_id, allocation_id, party, amount_micros) VALUES ($1, $2, 'ulo', $3)",
                   settlement_id, allocation.id, allocation.ulo_micros
               ).execute(&mut *tx).await?;
           }
           // ... similar for uno, referral
       }

       tx.commit().await?;
       Ok(prepared)
   }
   ```

2. **Settle parties independently**
   ```rust
   pub async fn execute_settlement(
       &self,
       settlement_id: Uuid,
       party: Party,
       provider_reference: &str,
   ) -> Result<(), AppError> {
       let mut tx = self.pool.begin().await?;

       // Update only the specific party's items
       sqlx::query!(
           r#"
           UPDATE settlement_items
           SET status = 'paid',
               provider_reference = $1,
               paid_at = NOW()
           WHERE settlement_id = $2 AND party = $3 AND status = 'pending'
           "#,
           provider_reference,
           settlement_id,
           party.to_string()
       ).execute(&mut *tx).await?;

       // DO NOT mark entire allocation as paid
       // Only mark allocation paid when all parties settled

       tx.commit().await?;
       Ok(())
   }
   ```

**Acceptance tests:**
- [ ] Duplicate IDs rejected
- [ ] Mixed currencies rejected
- [ ] Pay ULO doesn't mark UNO paid
- [ ] Concurrent preparation fails safely
- [ ] Provider timeout leaves unknown state

---

### R3-09 — Remove placeholder worker success

**Priority:** P0
**Owner:** Backend/Integration
**Affected files:**
- `uno-app/src/bin/worker.rs`
- `uno-app/src/server/services/worker_runner.rs`

**Implementation steps:**

1. **Replace log-only handlers with real work**
   ```rust
   pub async fn handle_sync_licenses(
       &self,
       job: &Job,
   ) -> Result<JobResult, AppError> {
       // Reject if no adapter configured
       let adapter = self.license_adapter.as_ref()
           .ok_or(AppError::ConfigError("License sync adapter not configured".into()))?;

       // Do actual sync
       let result = adapter.sync_licenses().await?;

       Ok(JobResult::Success {
           records_synced: result.count,
           provider_reference: result.sync_id,
       })
   }
   ```

2. **Fail on null publisher**
   ```rust
   impl OutboxPublisher for NullPublisher {
       async fn publish(&self, event: &OutboxEvent) -> Result<PublishResult, AppError> {
           // Return disabled, not success
           Err(AppError::ConfigError("Publisher not configured".into()))
       }
   }
   ```

3. **Disable unsupported job types**
   ```rust
   pub async fn process_job(&self, job: Job) -> Result<(), AppError> {
       match job.job_type.as_str() {
           "sync_licenses" if !self.is_sync_configured() => {
               self.reject_job(job.id, "Sync adapter not configured").await
           }
           "send_notification" if !self.is_notification_configured() => {
               self.reject_job(job.id, "Notification adapter not configured").await
           }
           // ... handle configured types
           _ => self.handle_job(job).await
       }
   }
   ```

**Acceptance tests:**
- [ ] Missing adapter fails job explicitly
- [ ] Null publisher returns disabled
- [ ] Success only with provider reference
- [ ] Optional CRM down doesn't block onboarding

---

### R3-10 — Lease, fence and recover jobs/events

**Priority:** P0
**Owner:** Operations/Backend
**Affected files:**
- `uno-app/src/server/repositories/outbox_repository.rs`
- `uno-app/src/server/services/job_service.rs`

**Implementation steps:**

1. **Add worker lease with fencing**
   ```sql
   ALTER TABLE outbox_events ADD COLUMN
       claimed_by UUID,
       claimed_at TIMESTAMPTZ,
       claim_expires_at TIMESTAMPTZ,
       claim_generation INT DEFAULT 0;
   ```

2. **Claim with lease**
   ```rust
   pub async fn claim_event(
       &self,
       worker_id: Uuid,
       lease_duration: Duration,
   ) -> Result<Option<OutboxEvent>, AppError> {
       sqlx::query_as!(
           OutboxEvent,
           r#"
           UPDATE outbox_events
           SET status = 'publishing',
               claimed_by = $1,
               claimed_at = NOW(),
               claim_expires_at = NOW() + $2::interval,
               claim_generation = claim_generation + 1
           WHERE id = (
               SELECT id FROM outbox_events
               WHERE status = 'pending'
                  OR (status = 'publishing' AND claim_expires_at < NOW())
               ORDER BY created_at
               LIMIT 1
               FOR UPDATE SKIP LOCKED
           )
           RETURNING *
           "#,
           worker_id,
           lease_duration.as_secs() as i64
       ).fetch_optional(&self.pool).await
   }
   ```

3. **Acknowledge with fence check**
   ```rust
   pub async fn acknowledge_event(
       &self,
       event_id: Uuid,
       worker_id: Uuid,
       expected_generation: i32,
   ) -> Result<bool, AppError> {
       let result = sqlx::query!(
           r#"
           UPDATE outbox_events
           SET status = 'published', published_at = NOW()
           WHERE id = $1
             AND claimed_by = $2
             AND claim_generation = $3
           "#,
           event_id, worker_id, expected_generation
       ).execute(&self.pool).await?;

       Ok(result.rows_affected() == 1)
   }
   ```

4. **Recover abandoned events**
   ```rust
   pub async fn recover_abandoned_events(&self) -> Result<i64, AppError> {
       let result = sqlx::query!(
           r#"
           UPDATE outbox_events
           SET status = 'pending',
               claimed_by = NULL,
               attempts = attempts + 1
           WHERE status = 'publishing'
             AND claim_expires_at < NOW()
             AND attempts < max_attempts
           "#
       ).execute(&self.pool).await?;

       Ok(result.rows_affected() as i64)
   }
   ```

**Acceptance tests:**
- [ ] Crash after claim recovers event
- [ ] Stale worker cannot acknowledge
- [ ] Abandoned events are reclaimed
- [ ] Replay has one business effect

---

### R4-05 — Integrate metadata, privacy and stored-byte integrity

**Priority:** P0
**Owner:** Storage/CMS/Security
**Affected files:**
- `uno-app/src/server/handlers/file_handler.rs`
- `uno-app/src/server/services/media_asset_service.rs`

**Implementation steps:**

1. **Authorize owner from principal**
   ```rust
   pub async fn upload_file(
       auth: AuthContext,
       multipart: Multipart,
   ) -> impl Responder {
       let owner_id = auth.principal.user_id;  // From token

       // Enforce per-file and aggregate limits
       let quota = quota_service.check_quota(owner_id).await?;
       if !quota.can_upload(content_length) {
           return Err(AppError::QuotaExceeded);
       }

       ...
   }
   ```

2. **Store transformed file digest**
   ```rust
   pub async fn process_upload(
       &self,
       upload: Upload,
   ) -> Result<AssetMetadata, AppError> {
       // Decode and re-encode image
       let processed = self.image_processor.process(&upload.bytes)?;

       // Hash the STORED bytes, not input
       let stored_hash = sha256::hash(&processed.bytes);

       // Save to storage
       let key = self.storage.save(&processed.bytes).await?;

       // Create metadata with accurate hash
       let metadata = AssetMetadata {
           key,
           original_hash: sha256::hash(&upload.bytes),
           stored_hash,  // Hash of what's actually stored
           size: processed.bytes.len(),
           mime_type: processed.mime_type,
           dimensions: processed.dimensions,
       };

       self.repo.save_metadata(&metadata).await?;
       Ok(metadata)
   }
   ```

3. **Check visibility on every read**
   ```rust
   pub async fn get_file(
       auth: Option<AuthContext>,
       asset_id: web::Path<Uuid>,
   ) -> impl Responder {
       let metadata = self.repo.get_metadata(*asset_id).await?
           .ok_or(AppError::NotFound)?;

       // Check visibility
       match metadata.visibility {
           Visibility::Public => { /* OK */ }
           Visibility::Private | Visibility::Draft => {
               let auth = auth.ok_or(AppError::Unauthorized)?;
               if metadata.owner_id != auth.principal.user_id {
                   return Err(AppError::Forbidden);
               }
           }
       }

       // Serve file
       ...
   }
   ```

**Acceptance tests:**
- [ ] Draft files return 401 anonymously
- [ ] Private files return 403 for wrong owner
- [ ] Stored hash matches actual file
- [ ] Quota exceeded blocks upload

---

### R4-06 — Implement real backup and restore

**Priority:** P0
**Owner:** Platform/Storage
**Affected files:**
- `uno-app/src/server/services/media_backup_service.rs`

**Implementation steps:**

1. **Copy actual bytes to backup destination**
   ```rust
   pub async fn create_backup(
       &self,
       destination: &BackupDestination,
   ) -> Result<BackupResult, AppError> {
       let mut manifest = BackupManifest::new();
       let mut copied_count = 0;
       let mut total_bytes = 0;

       // Stream assets with pagination
       let mut cursor = None;
       loop {
           let batch = self.repo.get_assets_batch(cursor, 1000).await?;
           if batch.is_empty() { break; }

           for asset in &batch {
               // Read actual file
               let bytes = self.storage.read(&asset.key).await?;

               // Verify stored hash
               let hash = sha256::hash(&bytes);
               if hash != asset.stored_hash {
                   manifest.add_error(asset.id, "Hash mismatch");
                   continue;
               }

               // Copy to destination
               destination.write(&asset.key, &bytes).await?;

               manifest.add_asset(asset.id, asset.key.clone(), hash);
               copied_count += 1;
               total_bytes += bytes.len();
           }

           cursor = batch.last().map(|a| a.id);
       }

       // Save manifest
       let manifest_bytes = serde_json::to_vec(&manifest)?;
       destination.write("manifest.json", &manifest_bytes).await?;

       Ok(BackupResult {
           assets_copied: copied_count,
           bytes_copied: total_bytes,
           manifest_hash: sha256::hash(&manifest_bytes),
       })
   }
   ```

2. **Implement restore with verification**
   ```rust
   pub async fn restore_backup(
       &self,
       source: &BackupSource,
   ) -> Result<RestoreResult, AppError> {
       // Read manifest
       let manifest_bytes = source.read("manifest.json").await?;
       let manifest: BackupManifest = serde_json::from_slice(&manifest_bytes)?;

       let mut restored_count = 0;

       for entry in &manifest.assets {
           // Read from backup
           let bytes = source.read(&entry.key).await?;

           // Verify hash
           let hash = sha256::hash(&bytes);
           if hash != entry.hash {
               return Err(AppError::BackupCorrupted(format!(
                   "Hash mismatch for {}", entry.key
               )));
           }

           // Write to storage
           self.storage.write(&entry.key, &bytes).await?;
           restored_count += 1;
       }

       Ok(RestoreResult {
           assets_restored: restored_count,
       })
   }
   ```

**Acceptance tests:**
- [ ] Backup copies actual bytes
- [ ] Restore works on empty volume
- [ ] Hash mismatch detected
- [ ] Missing asset fails backup

---

## Gate D: Correct Distribution

### R3-06 — Integrate import, publication and synchronization

**Priority:** P0
**Owner:** Distribution/Integration
**Affected files:**
- `uno-admin/src/logic/marketplace_service.rs`
- `uno-api/src/services/csv_import.rs`
- `uno-app/src/server/repositories/publication_repository.rs`

**Implementation steps:**

1. **Remove artificial lease codes**
   ```rust
   // Before: fabricated codes
   let lease_code = format!("LEASE-{}", Uuid::new_v4());

   // After: require real codes
   pub async fn import_license(
       &self,
       row: &CsvRow,
   ) -> Result<ImportResult, ImportError> {
       let lease_code = row.lease_code.as_ref()
           .ok_or(ImportError::MissingLeaseCode(row.row_number))?;

       // Validate format
       if !is_valid_lease_code(lease_code) {
           return Err(ImportError::InvalidLeaseCode(row.row_number, lease_code.clone()));
       }

       ...
   }
   ```

2. **Return per-item outcomes**
   ```rust
   pub async fn batch_publish(
       &self,
       license_ids: Vec<Uuid>,
   ) -> Result<BatchResult, AppError> {
       let mut results = Vec::new();

       for id in license_ids {
           let result = match self.publish_single(id).await {
               Ok(()) => ItemResult::Success { id },
               Err(e) => ItemResult::Failed { id, error: e.to_string() },
           };
           results.push(result);
       }

       Ok(BatchResult {
           total: results.len(),
           succeeded: results.iter().filter(|r| r.is_success()).count(),
           failed: results.iter().filter(|r| r.is_failed()).count(),
           items: results,
       })
   }
   ```

3. **Implement cursor-based pagination**
   ```rust
   pub async fn sync_events(
       &self,
       checkpoint: Option<SyncCheckpoint>,
   ) -> Result<SyncBatch, AppError> {
       let cursor = checkpoint.unwrap_or_default();

       let events = sqlx::query_as!(
           SyncEvent,
           r#"
           SELECT * FROM sync_events
           WHERE (event_time, id) > ($1, $2)
           ORDER BY event_time, id
           LIMIT 100
           "#,
           cursor.event_time,
           cursor.last_id
       ).fetch_all(&self.pool).await?;

       let new_checkpoint = events.last().map(|e| SyncCheckpoint {
           event_time: e.event_time,
           last_id: e.id,
       });

       Ok(SyncBatch {
           events,
           checkpoint: new_checkpoint,
           has_more: events.len() == 100,
       })
   }
   ```

**Acceptance tests:**
- [ ] Missing lease code fails import
- [ ] Batch returns per-item status
- [ ] >2500 events paginate correctly
- [ ] Unpublished cannot be reserved

---

### R3-11 — Finish local-media deployment

**Priority:** P0
**Owner:** Storage/Platform/CMS
**Affected files:**
- `deployment/target.json`
- `uno-app/src/server/handlers/file_handler.rs`

**Implementation steps:**

1. **Add volume verification**
   ```rust
   pub fn verify_volume() -> Result<(), AppError> {
       let mount_path = std::env::var("MEDIA_VOLUME_PATH")
           .unwrap_or("/var/lib/uno-app/media".into());

       // Check marker file
       let marker_path = Path::new(&mount_path).join(".uno-volume-id");
       if !marker_path.exists() {
           return Err(AppError::ConfigError(
               "Media volume not mounted or missing marker".into()
           ));
       }

       // Verify write access
       let test_path = Path::new(&mount_path).join(".write-test");
       std::fs::write(&test_path, b"test")?;
       std::fs::remove_file(&test_path)?;

       Ok(())
   }
   ```

2. **Centralize storage client lifecycle**
   ```rust
   // uno-app/src/server/app/service_factory.rs
   impl ServiceFactory {
       pub fn new(pool: ConnectionPool) -> Self {
           // Create storage client once
           let storage_config = StorageConfig::from_env()?;
           let storage_client: Arc<dyn StorageClient> = Arc::new(
               LocalStorageClient::new(storage_config)?
           );

           // Share across all services
           let media_service = MediaAssetService::new(
               storage_client.clone(),
               media_repo.clone(),
           );

           ...
       }
   }
   ```

**Acceptance tests:**
- [ ] Missing volume fails readiness
- [ ] No GCS/S3 credentials needed
- [ ] Container restart preserves files
- [ ] Read-only volume blocks writes only

---

## Gate E: Usable Product

### R3-13 — Implement complete redesigned journey

**Priority:** P1
**Owner:** Product/Design/Frontend/Support
**Affected screens:**
- Landing → Eligibility → Economics → Account → Setup → Reservation → Activation → Home

**Implementation steps:**

1. **Create journey state machine**
   ```rust
   pub enum OnboardingState {
       Landing,
       Eligibility { progress: EligibilityProgress },
       Economics { consent_version: Uuid },
       Account { provider: String },
       Setup { step: SetupStep },
       Reservation { license_id: Uuid, expires_at: DateTime<Utc> },
       Activation { license_id: Uuid },
       Active,
   }

   impl OnboardingState {
       pub fn can_resume(&self) -> bool {
           matches!(self, Self::Eligibility { .. } | Self::Economics { .. } | Self::Setup { .. })
       }
   }
   ```

2. **Implement resumable progress**
   ```rust
   pub async fn get_or_resume_onboarding(
       &self,
       user_id: Uuid,
   ) -> Result<OnboardingState, AppError> {
       // Check for existing progress
       if let Some(progress) = self.repo.get_progress(user_id).await? {
           if progress.can_resume() {
               return Ok(progress);
           }
       }

       // Start fresh
       Ok(OnboardingState::Landing)
   }
   ```

**Acceptance tests:**
- [ ] User can resume after disconnect
- [ ] Back navigation preserves state
- [ ] All screens have error states
- [ ] No false "earning" status

---

### R3-14 — Preserve CMS and all ten locales

**Priority:** P1
**Owner:** CMS/Localization/Frontend
**Affected files:**
- `uno-app/src/locales/*.rs`
- All locale JSON bundles

**Implementation steps:**

1. **Add Bangla to lazy loader**
   ```rust
   // uno-app/src/locales/lazy_loader.rs
   pub fn get_locale(code: &str) -> Option<&'static Locale> {
       match code {
           "en" => Some(&EN),
           "es" => Some(&ES),
           "bn" => Some(&BN),  // Add Bangla
           // ... other locales
           _ => None
       }
   }
   ```

2. **Complete missing translation keys**
   ```json
   // For each of the 7 incomplete bundles (tl, hi, sw, pt, fr, ar, id)
   // Add economics/reservation keys
   {
       "economics.ulo_share": "...",
       "economics.uno_share": "...",
       "economics.referral_share": "...",
       "reservation.expires_at": "...",
       ...
   }
   ```

**Acceptance tests:**
- [ ] All 10 locales complete journey
- [ ] Arabic RTL works correctly
- [ ] No raw key names shown
- [ ] CMS preview/publish works

---

### R3-15 — Integrate pluggable forecasting

**Priority:** P1
**Owner:** Finance/Product/Backend

See R4-07 for detailed implementation.

---

### R4-07 — Unify forecast engines

**Priority:** P0
**Owner:** Finance/Analytics/Backend
**Affected files:**
- `uno-app/src/server/services/forecast_service.rs`
- `uno-api/src/services/forecast.rs`

**Implementation steps:**

1. **Create unified calculation core**
   ```rust
   pub struct ForecastEngine {
       pub model_version: String,
   }

   impl ForecastEngine {
       pub fn calculate_week(
           &self,
           cohort: &Cohort,
           tasks: &[Task],
           costs: &Costs,
       ) -> WeekResult {
           // Shared cohort - don't duplicate per task
           let active_devices = cohort.active_devices;

           let mut pool = Decimal::ZERO;
           for task in tasks {
               let eligible = self.eligible_for_task(cohort, task);
               let work_units = std::cmp::min(eligible, task.supply_cap);
               pool += work_units * task.rate;
           }

           // Split 50/40/10
           let ulo = pool * dec!(0.50);
           let uno = pool * dec!(0.40);
           let referral_or_reserve = pool * dec!(0.10);

           // Calculate expenses (count users ONCE)
           let expenses = Expenses {
               credits: cohort.active_count * costs.credit_per_user,
               support: cohort.active_count * costs.support_per_user,
               acquisition: cohort.new_this_week * costs.acquisition_per_user,
               software: costs.software_fixed,
               hosting: costs.hosting_fixed,
               // ... other costs
           };

           WeekResult {
               pool,
               ulo,
               uno,
               referral_or_reserve,
               expenses: expenses.total(),
               profit: uno - expenses.total(),
           }
       }
   }
   ```

2. **Enforce inventory ceiling**
   ```rust
   fn validate_forecast(&self, forecast: &Forecast) -> Result<(), ValidationError> {
       for week in &forecast.weeks {
           if week.occupied_licenses > 2500 {
               return Err(ValidationError::ExceedsCapacity(week.week_number));
           }
       }
       Ok(())
   }
   ```

3. **Separate actual from projected**
   ```rust
   pub enum TaskStatus {
       Actual { last_measured: DateTime<Utc> },
       Projected { source: String },
   }

   impl Task {
       pub fn rate_label(&self) -> String {
           match &self.status {
               TaskStatus::Actual { last_measured } =>
                   format!("Measured {}", last_measured.format("%Y-%m-%d")),
               TaskStatus::Projected { source } =>
                   format!("Projected ({})", source),
           }
       }
   }
   ```

**Acceptance tests:**
- [ ] Adding task doesn't duplicate users
- [ ] Zero supply = zero revenue
- [ ] 2500 ceiling enforced
- [ ] All costs included once

---

### R4-09 — Connect redesigned screens to real state

**Priority:** P1
**Owner:** Product/Design/Frontend/CMS

Combines with R3-13 - see above.

---

### R3-16 — Country agents and optional marketing automation

**Priority:** P1
**Owner:** Growth/Support/Integration

**Implementation steps:**

1. **Implement weekly quotas**
   ```rust
   pub struct MarketQuota {
       pub country_code: String,
       pub weekly_target: i32,
       pub current_week_count: i32,
   }

   const MARKET_QUOTAS: &[(&str, i32)] = &[
       ("IN", 60),
       ("PH", 60),
       ("NG", 50),
       ("KE", 35),
       ("BD", 30),
       ("GH", 15),
   ];
   ```

2. **Deduplicate leads**
   ```rust
   pub async fn record_lead(
       &self,
       contact: &Contact,
       source: &LeadSource,
   ) -> Result<Lead, AppError> {
       // Check for existing lead
       if let Some(existing) = self.repo.find_by_contact(contact).await? {
           // Update source if newer
           if source.timestamp > existing.source_timestamp {
               self.repo.update_source(existing.id, source).await?;
           }
           return Ok(existing);
       }

       // Create new lead
       self.repo.create_lead(contact, source).await
   }
   ```

**Acceptance tests:**
- [ ] CRM down doesn't block onboarding
- [ ] Duplicate leads deduplicated
- [ ] Opt-out propagates
- [ ] Agent sees only their country

---

## Gate F: Operational Governance

### R3-17 — Operational governance

**Priority:** P0 for controls, P1 for analytics
**Owner:** Operations/Security/Product
**Affected files:**
- `uno-app/src/server/services/launch_gate_service.rs`
- `uno-app/src/server/services/consent_service.rs`

**Implementation steps:**

1. **Enforce gates at mutation time**
   ```rust
   pub async fn check_gate(
       &self,
       gate_name: &str,
       context: &GateContext,
   ) -> Result<bool, AppError> {
       let gate = self.repo.get_gate(gate_name).await?
           .ok_or(AppError::NotFound)?;

       // Check enabled
       if !gate.is_enabled {
           return Ok(false);
       }

       // Check expiry
       if let Some(expires_at) = gate.expires_at {
           if expires_at < Utc::now() {
               return Ok(false);
           }
       }

       // Check evidence
       if gate.requires_evidence {
           let evidence = self.evidence_repo.get_evidence(gate_name).await?;
           if !evidence.all_mandatory_present() {
               return Ok(false);
           }
       }

       Ok(true)
   }
   ```

2. **Implement retention policies**
   ```rust
   pub async fn apply_retention_policies(&self) -> Result<RetentionResult, AppError> {
       let mut deleted = 0;
       let mut anonymized = 0;

       // Delete expired marketing data
       deleted += self.repo.delete_expired_marketing().await?;

       // Anonymize but preserve financial records
       anonymized += self.repo.anonymize_old_allocations().await?;

       // Keep audit logs for required period
       // DO NOT delete within retention window

       Ok(RetentionResult { deleted, anonymized })
   }
   ```

3. **Redact secrets in logs**
   ```rust
   pub fn redact_sensitive(value: &str) -> String {
       // Redact lease codes
       let redacted = LEASE_CODE_PATTERN.replace_all(value, "[REDACTED_LEASE]");

       // Redact session tokens
       let redacted = SESSION_TOKEN_PATTERN.replace_all(&redacted, "[REDACTED_TOKEN]");

       redacted.to_string()
   }
   ```

**Acceptance tests:**
- [ ] Expired gate blocks mutations
- [ ] Deletion preserves financial records
- [ ] Logs contain no credentials
- [ ] Each exception has recovery owner

---

## Verification Checklist

### Before removing issuance pause:

- [ ] R3-05: Concurrent claim tests pass
- [ ] R4-02: All handlers derive owner from principal
- [ ] R4-03: Schema/query contracts match
- [ ] R3-10: Job fencing prevents duplicates

### Before pilot launch:

- [ ] All Gate A-D tests pass
- [ ] R4-06: Backup/restore rehearsed
- [ ] R3-14: Launch market locales complete
- [ ] R3-17: Gates and evidence in place

### Before scale expansion:

- [ ] Gate E complete
- [ ] R4-07: Forecast economics verified
- [ ] R3-16: Agent workspace functional
- [ ] 30 pilot participants at D30

---

## Appendix: File Change Summary

| Gate | New Files | Modified Files |
|------|-----------|----------------|
| A | 5 | 15 |
| B | 8 | 20 |
| C | 10 | 25 |
| D | 5 | 15 |
| E | 20 | 30 |
| F | 5 | 10 |
| **Total** | **~53** | **~115** |

---

## Implementation Order

1. **Week 1-2:** Gate A (foundation) - R3-01, R3-02, R4-01, R4-03
2. **Week 3-4:** Gate B (security) - R3-03, R3-04, R3-05, R4-02
3. **Week 5-6:** Gate C (obligations) - R3-07, R3-08, R3-09, R3-10, R4-04
4. **Week 7-8:** Gate D (distribution) - R3-06, R3-11, R4-05, R4-06
5. **Week 9-12:** Gate E (product) - R3-13, R3-14, R3-15, R4-07, R4-09
6. **Week 13-14:** Gate F (governance) - R3-17, R4-08, R4-10

**Note:** These are implementation sequencing estimates, not calendar commitments. Actual timeline depends on team capacity and dependency resolution.
