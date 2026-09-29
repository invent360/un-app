# UNO-APP v2 Pilot Readiness Checklist

**Version:** 1.0
**Date:** 29 September 2026
**Target:** 30-user pilot across 2 markets

---

## Pre-Pilot Gates

### Software Gates (Must Pass)

| ID | Gate | Status | Verified By | Date |
|----|------|--------|-------------|------|
| SW-01 | All 24 acceptance tests pass | [ ] | | |
| SW-02 | Security audit findings addressed | [ ] | | |
| SW-03 | Concurrent claim test passes (ACC-02) | [ ] | | |
| SW-04 | 50/40/10 split survives lifecycle (ACC-05) | [ ] | | |
| SW-05 | Replay protection active (ACC-06) | [ ] | | |
| SW-06 | 2,500+ events reconcile (ACC-07) | [ ] | | |
| SW-07 | Path traversal rejected (ACC-08) | [ ] | | |
| SW-08 | Release artifacts contain no tokens (ACC-12) | [ ] | | |

### Commercial Gates (Must Pass)

| ID | Gate | Status | Verified By | Date |
|----|------|--------|-------------|------|
| CM-01 | Terms of Service finalized | [ ] | | |
| CM-02 | Privacy Policy published | [ ] | | |
| CM-03 | User Agreement version locked | [ ] | | |
| CM-04 | Referral program rules documented | [ ] | | |
| CM-05 | Credit cost/payer disclosure approved | [ ] | | |

### Operational Gates (Must Pass)

| ID | Gate | Status | Verified By | Date |
|----|------|--------|-------------|------|
| OP-01 | Support queue operational | [ ] | | |
| OP-02 | Escalation contacts confirmed | [ ] | | |
| OP-03 | Monitoring dashboards active | [ ] | | |
| OP-04 | Alert thresholds configured | [ ] | | |
| OP-05 | Incident response plan documented | [ ] | | |

---

## Pilot Configuration

### Market 1: Philippines

| Item | Value | Verified |
|------|-------|----------|
| Locale | `tl` (Tagalog) | [ ] |
| Currency | PHP | [ ] |
| Time Zone | Asia/Manila (UTC+8) | [ ] |
| Pilot Users | 15 | [ ] |
| Support Contact | TBD | [ ] |

### Market 2: Bangladesh

| Item | Value | Verified |
|------|-------|----------|
| Locale | `bn` (Bangla) | [ ] |
| Currency | BDT | [ ] |
| Time Zone | Asia/Dhaka (UTC+6) | [ ] |
| Pilot Users | 15 | [ ] |
| Support Contact | TBD | [ ] |

---

## User Provisioning Checklist

For each pilot user, verify:

- [ ] Unique identifier assigned
- [ ] Email/contact verified
- [ ] Device type documented (Android/iOS/Windows)
- [ ] Agreement version acknowledged
- [ ] Support contact provided
- [ ] Onboarding materials sent

### User Tracking Spreadsheet

| User ID | Market | Device | Invited | Registered | Reserved | Claimed | Activated | Notes |
|---------|--------|--------|---------|------------|----------|---------|-----------|-------|
| P001 | PH | | | | | | | |
| P002 | PH | | | | | | | |
| ... | | | | | | | | |
| P015 | PH | | | | | | | |
| B001 | BD | | | | | | | |
| B002 | BD | | | | | | | |
| ... | | | | | | | | |
| B015 | BD | | | | | | | |

---

## Data Verification Checklist

### Pre-Pilot Data Check

| Check | Expected | Actual | Status |
|-------|----------|--------|--------|
| Available licenses for pilot | 30 | | [ ] |
| Split configuration | 50/40/10 | | [ ] |
| Agreement version | v1 | | [ ] |
| Upstream sync active | Yes | | [ ] |
| Health check passing | Yes | | [ ] |

### Daily Pilot Data Checks

Run these checks daily during pilot:

1. **Record Count Match**
   ```sql
   SELECT COUNT(*) FROM licenses WHERE claimed = true;
   -- Compare with upstream dashboard
   ```

2. **Split Integrity**
   ```sql
   SELECT
     agreement_version,
     COUNT(*) as license_count
   FROM licenses
   WHERE claimed = true
   GROUP BY agreement_version;
   -- All should be v1 (50/40/10)
   ```

3. **Sync Lag**
   ```sql
   SELECT MAX(synced_at) FROM sync_checkpoints;
   -- Should be within last 15 minutes
   ```

4. **Error Rate**
   ```sql
   SELECT
     DATE(created_at) as date,
     COUNT(*) FILTER (WHERE level = 'error') as errors
   FROM logs
   WHERE created_at > NOW() - INTERVAL '24 hours'
   GROUP BY DATE(created_at);
   ```

---

## Support Readiness

### Response Time SLAs

| Priority | Description | Response Time | Resolution Time |
|----------|-------------|---------------|-----------------|
| P1 - Critical | Unable to claim/activate | < 1 hour | < 4 hours |
| P2 - High | Feature not working | < 4 hours | < 24 hours |
| P3 - Medium | UI/UX issues | < 24 hours | < 72 hours |
| P4 - Low | Questions/enhancement | < 48 hours | Next release |

### Support Escalation Path

```
Level 1: Support Queue
    └── Level 2: Technical Support
        └── Level 3: Engineering
            └── Level 4: Product/Legal (if required)
```

### Known Issues List

Document any known issues before pilot:

| ID | Issue | Workaround | Planned Fix |
|----|-------|------------|-------------|
| | | | |

---

## Monitoring Checklist

### Dashboards

- [ ] Application health dashboard
- [ ] Claim funnel metrics
- [ ] Error rate monitoring
- [ ] Sync status dashboard
- [ ] User journey tracking

### Alerts Configured

| Alert | Threshold | Notification |
|-------|-----------|--------------|
| Error rate spike | > 5% in 15 min | Slack + Email |
| Sync lag | > 30 min | Slack |
| Failed claims | > 3 in 1 hour | Email |
| Health check fail | Any | PagerDuty |

---

## Communication Plan

### Pilot User Communications

| Touchpoint | Timing | Channel | Template Ready |
|------------|--------|---------|----------------|
| Invitation | D-7 | Email | [ ] |
| Onboarding | D-1 | Email + SMS | [ ] |
| Day 1 Check | D+1 | In-app + Email | [ ] |
| Day 3 Check | D+3 | In-app | [ ] |
| Day 7 Check | D+7 | Email | [ ] |
| Day 30 Check | D+30 | Email | [ ] |

### Internal Communications

| Update | Frequency | Audience | Owner |
|--------|-----------|----------|-------|
| Daily Status | Daily | Pilot Team | |
| Issue Report | As needed | Engineering | |
| Weekly Summary | Weekly | Stakeholders | |

---

## Go/No-Go Decision

### Go Criteria (ALL must be met)

- [ ] All software gates pass (SW-01 through SW-08)
- [ ] All commercial gates pass (CM-01 through CM-05)
- [ ] All operational gates pass (OP-01 through OP-05)
- [ ] 30 pilot users provisioned and verified
- [ ] Both markets configured and tested
- [ ] Support team briefed and ready
- [ ] Monitoring and alerts active
- [ ] Rollback plan documented

### No-Go Triggers (ANY blocks pilot)

- Critical security vulnerability unresolved
- Acceptance tests failing
- Support team not ready
- Legal/compliance issues pending
- Upstream sync not working

### Decision Record

| Date | Decision | Rationale | Approver |
|------|----------|-----------|----------|
| | | | |

---

## Rollback Plan

If critical issues are discovered during pilot:

### Immediate Actions (< 1 hour)

1. Notify all pilot users via SMS/Email
2. Disable new claim flow (feature flag)
3. Preserve all data (no deletions)
4. Document issue details

### Investigation (1-4 hours)

1. Root cause analysis
2. Impact assessment (users affected)
3. Data integrity check
4. Stakeholder notification

### Resolution Options

1. **Fix Forward**: Deploy patch within 24 hours
2. **Feature Disable**: Turn off affected feature
3. **Full Rollback**: Revert to previous version
4. **Pilot Pause**: Suspend pilot until resolved

---

## Post-Pilot Review

Schedule post-pilot review for D+30:

### Review Topics

- [ ] Pilot success metrics
- [ ] User feedback summary
- [ ] Technical issues encountered
- [ ] Support ticket analysis
- [ ] Data quality assessment
- [ ] Lessons learned
- [ ] Recommendations for GA

### Success Metrics

| Metric | Target | Actual |
|--------|--------|--------|
| Claim success rate | > 95% | |
| Activation rate | > 90% | |
| D30 retention | > 70% | |
| Support tickets | < 1 per user | |
| Critical bugs | 0 | |
| Data discrepancies | 0 | |

---

## Signatures

| Role | Name | Signature | Date |
|------|------|-----------|------|
| Product Owner | | | |
| Engineering Lead | | | |
| Support Lead | | | |
| Legal/Compliance | | | |
