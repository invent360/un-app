# UNO-APP Homepage Implementation Roadmap

**Version:** 1.0
**Date:** 2 October 2026
**Based on:** Consolidated Review Feedback & Earnings Calculator Design

---

## Executive Summary

This roadmap implements a simplified, task-based homepage that:
- Replaces unsupported $5-75 earnings tiers with actual task data
- Uses a pluggable earnings calculator module
- Removes authentication and KYC barriers from the claim flow
- Streamlines the user journey to: Click Start → Accept Terms → Earning

---

## Implementation Phases Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        IMPLEMENTATION TIMELINE                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  PHASE 1          PHASE 2          PHASE 3          PHASE 4          PHASE 5│
│  Foundation       Earnings         Journey          Homepage         i18n & │
│  & Content        System           Simplify         Redesign         Launch │
│                                                                             │
│  ┌────────┐      ┌────────┐      ┌────────┐      ┌────────┐      ┌────────┐ │
│  │ HP-01  │──────│ HP-05  │──────│ HP-07  │──────│ HP-03  │──────│ HP-10  │ │
│  │ HP-02  │      │ HP-06  │      │ HP-04  │      │ HP-09  │      │ HP-11  │ │
│  │        │      │        │      │        │      │        │      │ HP-12  │ │
│  └────────┘      └────────┘      └────────┘      └────────┘      └────────┘ │
│                                                                             │
│  Week 1-2        Week 2-3        Week 3-4        Week 4-5        Week 5-6   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Phase 1: Foundation & Content Truth (HP-01, HP-02)

### Objectives
- Remove unsupported claims ($5-75 tiers)
- Define sponsored offer clearly
- Establish claim evidence system

### Tasks

```
┌─────────────────────────────────────────────────────────────────┐
│ 1.1 Audit and Remove Unsupported Claims                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  REMOVE                          REPLACE WITH                   │
│  ────────────────────────────────────────────────────────────── │
│  "Earn $5-15/month"         →   "Earn per task completed"       │
│  "Earn $15-35/month"        →   "$0.10/day per active task"     │
│  "Earn $35-75/month"        →   "Multiple tasks stack"          │
│  "Most Popular" badge       →   (remove entirely)               │
│  "Thousands of users"       →   (remove or cite source)         │
│  "100% secure & private"    →   "Permissions explained below"   │
│  Demo availability banner   →   (remove from production)        │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Files to Modify

| File | Changes |
|------|---------|
| `src/routes/home.rs` | Remove earnings tier sections |
| `src/locales/*.rs` | Remove tier translation keys |
| `src/components/home/earnings_section.rs` | Replace with task-based section |

### Deliverables
- [ ] Claim evidence register created
- [ ] Unsupported earnings removed from all 12 locales
- [ ] Sponsored offer copy finalized

---

## Phase 2: Task-Based Earnings System (HP-05, HP-06)

### Objectives
- Implement pluggable task catalogue
- Replace earnings tiers with transparent task info
- Integrate earnings calculator

### Task Catalogue Data Model

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           TASK CATALOGUE SCHEMA                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌──────────────────────────────────────────────────────────────────────┐   │
│  │ TaskDefinition                                                       │   │
│  ├──────────────────────────────────────────────────────────────────────┤   │
│  │ id: String              │ "telemetry" | "ugrid" | "entropy" | ...    │   │
│  │ name: String            │ "Extended Telemetry"                       │   │
│  │ description: String     │ "Earn by sharing device telemetry"         │   │
│  │ status: TaskStatus      │ Current | Projected                        │   │
│  │ rate_per_day: Decimal   │ 0.10 (USD)                                 │   │
│  │ rate_mode: RateMode     │ PerDay | PerCall                           │   │
│  │ supported_devices: Vec  │ [AndroidPlay, AndroidApk, IPhone]          │   │
│  │ requires_kyc: bool      │ true                                       │   │
│  │ requires_wifi: bool     │ true                                       │   │
│  │ conditions: String      │ "Stable internet connection required"      │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Current Task Matrix

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         TASK AVAILABILITY MATRIX                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│                        │ Android │ Android │        │ Windows │             │
│  TASK                  │  Play   │   APK   │ iPhone │   GPU   │   STATUS    │
│  ──────────────────────┼─────────┼─────────┼────────┼─────────┼─────────────│
│  Extended Telemetry    │   ✓     │    ✓    │   ✓    │         │  CURRENT    │
│  Ugrid                 │         │    ✓    │        │         │  CURRENT    │
│  Entropy               │   ✓     │    ✓    │   ✓    │         │  PROJECTED  │
│  Caller-ID Testing     │   ✓     │    ✓    │   ✓    │         │  LIMITED    │
│  GPU Contribution      │         │         │        │    ✓    │  PROJECTED  │
│  ──────────────────────┼─────────┼─────────┼────────┼─────────┼─────────────│
│  Rate per task/day     │  $0.10  │  $0.10  │  $0.10 │  $0.10  │             │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Earnings Calculator Integration

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      CALCULATOR COMPONENT ARCHITECTURE                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐        │
│  │   TaskStore     │────▶│  EarningsCalc   │────▶│  ResultsView    │        │
│  │  (Static Data)  │     │    (Engine)     │     │   (Display)     │        │
│  └─────────────────┘     └─────────────────┘     └─────────────────┘        │
│         │                        │                       │                  │
│         ▼                        ▼                       ▼                  │
│  ┌─────────────────────────────────────────────────────────────────┐        │
│  │                        USER INPUTS                              │        │
│  │  • Device count (Android Play, APK, iPhone, Windows GPU)        │        │
│  │  • Days to calculate                                            │        │
│  │  • KYC verified status                                          │        │
│  │  • Include projected tasks toggle                               │        │
│  └─────────────────────────────────────────────────────────────────┘        │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────┐        │
│  │                       CALCULATION OUTPUT                        │        │
│  │  • ULO total (participant share: 50%)                           │        │
│  │  • Breakdown by task                                            │        │
│  │  • Breakdown by device                                          │        │
│  │  • KYC threshold notification ($5 rule)                         │        │
│  └─────────────────────────────────────────────────────────────────┘        │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Reward Share Split Visualization

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         REWARD DISTRIBUTION (50/40/10)                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                      DISTRIBUTABLE POOL (100%)                      │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                    │                                        │
│           ┌────────────────────────┼────────────────────────┐               │
│           ▼                        ▼                        ▼               │
│  ┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐      │
│  │   ULO (50%)     │      │   UNO (40%)     │      │ Referral (10%)  │      │
│  │   PARTICIPANT   │      │   OPERATOR      │      │   GROWTH        │      │
│  │   ═══════════   │      │   ═══════════   │      │   ═══════════   │      │
│  │  Your earnings  │      │  Platform ops   │      │  Referrer bonus │      │
│  │  Direct payout  │      │  Credits fund   │      │  If applicable  │      │
│  └─────────────────┘      └─────────────────┘      └─────────────────┘      │
│                                                                             │
│  Example: Task earning $0.10/day distributable pool                         │
│  ────────────────────────────────────────────────────                       │
│  • You receive: $0.05/day (ULO 50%)                                         │
│  • Operator gets: $0.04/day (UNO 40%)                                       │
│  • Referral pool: $0.01/day (10%)                                           │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Files to Create/Modify

| File | Action | Description |
|------|--------|-------------|
| `src/types/task.rs` | Create | Task definition types |
| `src/server/services/task_service.rs` | Create | Task catalogue service |
| `src/components/home/task_earnings.rs` | Create | New earnings display component |
| `src/components/calculator/mod.rs` | Create | Earnings calculator component |

---

## Phase 3: Simplified User Journey (HP-04, HP-07)

### Objectives
- Remove authentication barriers from claim flow
- Implement frictionless "Start Earning" → Lease → Claim flow
- KYC is informational only (reminder tooltip, not blocking)

### Simplified Eligibility Model

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                       SIMPLIFIED ELIGIBILITY RULES                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                      FRICTIONLESS CLAIM FLOW                        │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                             │
│  1. COUNTRY CHECK: ─────────────────────────────────────────────────────    │
│     │                                                                       │
│     └─▶ SKIP (only advertise in eligible countries)                         │
│                                                                             │
│  2. AUTHENTICATION: ────────────────────────────────────────────────────    │
│     │                                                                       │
│     └─▶ NOT REQUIRED before claiming                                        │
│                                                                             │
│  3. KYC REQUIREMENT: ───────────────────────────────────────────────────    │
│     │                                                                       │
│     └─▶ NOT ENFORCED - Show tooltip reminder only:                          │
│         "Note: KYC required for withdrawals over $5"                        │
│                                                                             │
│  4. DEVICE TASK AVAILABILITY: ──────────────────────────────────────────    │
│     │                                                                       │
│     ├─▶ Android (Play Store) → Telemetry, Entropy*, Caller-ID*              │
│     ├─▶ Android (APK)        → Telemetry, Ugrid, Entropy*, Caller-ID*       │
│     ├─▶ iPhone               → Telemetry, Entropy*, Caller-ID*              │
│     └─▶ Windows GPU          → GPU Contribution*                            │
│                                                                             │
│         * = Projected or limited availability                               │
│                                                                             │
│  5. UGRID SPECIAL RULE: ────────────────────────────────────────────────    │
│     │                                                                       │
│     └─▶ Only available on Android APK (not Play Store, not iOS)             │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Simplified User Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         SIMPLIFIED USER JOURNEY                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  BEFORE (Complex)                    AFTER (Frictionless)                   │
│  ═══════════════════                 ══════════════════════                 │
│                                                                             │
│  ┌─────────────────┐                 ┌─────────────────┐                    │
│  │ 1. Visit Page   │                 │ 1. Visit Page   │                    │
│  └────────┬────────┘                 └────────┬────────┘                    │
│           ▼                                   ▼                             │
│  ┌─────────────────┐                 ┌─────────────────┐                    │
│  │ 2. Check Country│                 │ 2. Click Start  │                    │
│  └────────┬────────┘                 │    Earning      │                    │
│           ▼                          └────────┬────────┘                    │
│  ┌─────────────────┐                          ▼                             │
│  │ 3. Check Device │                 ┌─────────────────┐                    │
│  └────────┬────────┘                 │ 3. Accept Terms │◀── Lease terms     │
│           ▼                          │    (+ tooltip)  │    + KYC reminder  │
│  ┌─────────────────┐                 └────────┬────────┘                    │
│  │ 4. Check Tasks  │                          ▼                             │
│  └────────┬────────┘                 ┌─────────────────┐                    │
│           ▼                          │ 4. Dashboard    │                    │
│  ┌─────────────────┐                 │    (earning)    │                    │
│  │ 5. Login        │                 └─────────────────┘                    │
│  └────────┬────────┘                                                        │
│           ▼                                                                 │
│  ┌─────────────────┐                                                        │
│  │ 6. Complete KYC │                                                        │
│  └────────┬────────┘                                                        │
│           ▼                                                                 │
│  ┌─────────────────┐                                                        │
│  │ 7. Accept Lease │                                                        │
│  └────────┬────────┘                                                        │
│           ▼                                                                 │
│  ┌─────────────────┐                                                        │
│  │ 8. Reserve      │                                                        │
│  └────────┬────────┘                                                        │
│           ▼                                                                 │
│  ┌─────────────────┐                                                        │
│  │ 9. Claim        │                                                        │
│  └────────┬────────┘                                                        │
│           ▼                                                                 │
│  ┌─────────────────┐                                                        │
│  │ 10. Install     │                                                        │
│  └────────┬────────┘                                                        │
│           ▼                                                                 │
│  ┌─────────────────┐                                                        │
│  │ 11. Start Earn  │                                                        │
│  └─────────────────┘                                                        │
│                                                                             │
│  11 STEPS ──────────────────────────▶ 4 STEPS (frictionless)                │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### "Start Earning" Button Flow

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                 SIMPLIFIED "START EARNING" BUTTON SEQUENCE                   │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  User            App              Backend           Provider                 │
│   │               │                  │                 │                     │
│   │──Click Start─▶│                  │                 │                     │
│   │               │                  │                 │                     │
│   │               │──Show Lease Terms│                 │                     │
│   │◀──Terms Modal─│  (+ eligibility  │                 │                     │
│   │    tooltip)   │   reminder)      │                 │                     │
│   │               │                  │                 │                     │
│   │   ┌─────────────────────────────────────────────┐  │                     │
│   │   │  💡 Tooltip: "Note: KYC required for        │  │                     │
│   │   │     withdrawals over $5"                    │  │                     │
│   │   └─────────────────────────────────────────────┘  │                     │
│   │               │                  │                 │                     │
│   │──Accept──────▶│                  │                 │                     │
│   │               │──Atomic Claim───▶│                 │                     │
│   │               │                  │──Reserve───────▶│                     │
│   │               │                  │◀──Confirmed─────│                     │
│   │               │◀──Claim Receipt──│                 │                     │
│   │               │                  │                 │                     │
│   │◀──Dashboard───│                  │                 │                     │
│   │   (earning)   │                  │                 │                     │
│   │               │                  │                 │                     │
│                                                                              │
│  KEY CHANGES:                                                                │
│  ─────────────                                                               │
│  ✓ NO login required before claiming                                        │
│  ✓ NO KYC enforcement (just informational tooltip)                           │
│  ✓ Direct: Click → Terms → Accept → Earning                                 │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## Phase 4: Homepage Redesign (HP-03, HP-09)

### Page Structure

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         HOMEPAGE INFORMATION ARCHITECTURE                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │ HEADER                                                    [LANG] [?]│    │
│  │ ════════════════════════════════════════════════════════════════════│    │
│  │ DJED NODES                    Tasks   How it Works   Help   Sign In │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │ SECTION 1: HERO                                                     │    │
│  │ ════════════════════════════════════════════════════════════════════│    │
│  │                                                                     │    │
│  │  Earn rewards with a device        ┌─────────────────────────┐      │    │
│  │  you already own                   │                         │      │    │
│  │                                    │    [Phone Screenshot]   │      │    │
│  │  Complete tasks on your phone      │    Real app dashboard   │      │    │
│  │  or laptop. We fund your licence   │    (anonymized)         │      │    │
│  │  - you receive 50% of rewards.     │                         │      │    │
│  │                                    └─────────────────────────┘      │    │
│  │  ┌──────────────────┐  ┌──────────────────┐                         │    │
│  │  │  Start Earning   │  │  How it Works    │                         │    │
│  │  └──────────────────┘  └──────────────────┘                         │    │
│  │                                                                     │    │
│  │  • No upfront licence fee                                           │    │
│  │  • Stable internet required                                         │    │
│  │  • Rewards depend on available tasks                                │    │
│  │                                                                     │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Hero Section Mockup

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              HERO SECTION                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                                                                       │  │
│  │     ████████████████████████████████████████████████████████████      │  │
│  │     █                                                          █      │  │
│  │     █   Earn rewards with a           ┌────────────────────┐   █      │  │
│  │     █   device you already own        │ ┌────────────────┐ │   █      │  │
│  │     █                                 │ │ █ DJED NODES   │ │   █      │  │
│  │     █   Complete tasks on your phone  │ │                │ │   █      │  │
│  │     █   or laptop. DJED Nodes funds   │ │  Your Balance  │ │   █      │  │
│  │     █   your licence credits.         │ │  ────────────  │ │   █      │  │
│  │     █                                 │ │  $XX.XX        │ │   █      │  │
│  │     █   You receive 50% of rewards.   │ │                │ │   █      │  │
│  │     █                                 │ │  Active Tasks  │ │   █      │  │
│  │     █   ┌─────────────────────────┐   │ │  ● Telemetry   │ │   █      │  │
│  │     █   │     Start Earning  ▶    │   │ │  ● Ugrid       │ │   █      │  │
│  │     █   └─────────────────────────┘   │ │                │ │   █      │  │
│  │     █                                 │ └────────────────┘ │   █      │  │
│  │     █   How it Works ↓                │     (SAMPLE)       │   █      │  │
│  │     █                                 └────────────────────┘   █      │  │
│  │     █                                                          █      │  │
│  │     █   ┌────────┐ ┌────────┐ ┌────────┐                       █      │  │
│  │     █   │ ✓ No   │ │ ✓ Earn │ │ ✓ Your │                       █      │  │
│  │     █   │ upfront│ │ $0.10+ │ │ 50%    │                       █      │  │
│  │     █   │ fee    │ │ / task │ │ share  │                       █      │  │
│  │     █   └────────┘ └────────┘ └────────┘                       █      │  │
│  │     █                                                          █      │  │
│  │     ████████████████████████████████████████████████████████████      │  │
│  │                                                                       │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Available Tasks Section

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          SECTION 2: AVAILABLE TASKS                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                                                                       │  │
│  │     Available Tasks                                                   │  │
│  │     ════════════════                                                  │  │
│  │     Earn by completing these tasks on your device                     │  │
│  │                                                                       │  │
│  │     ┌─────────────────────────────────────────────────────────────┐   │  │
│  │     │  CURRENT TASKS                                              │   │  │
│  │     ├─────────────────────────────────────────────────────────────┤   │  │
│  │     │                                                             │   │  │
│  │     │  ┌───────────────────┐  ┌───────────────────┐               │   │  │
│  │     │  │ 📊 Extended       │  │ ⚡ Ugrid           │               │   │  │
│  │     │  │    Telemetry      │  │                   │               │   │  │
│  │     │  │ ───────────────── │  │ ───────────────── │               │   │  │
│  │     │  │ $0.10 / day       │  │ $0.10 / day       │               │   │  │
│  │     │  │                   │  │                   │               │   │  │
│  │     │  │ ✓ Android Play    │  │ ✓ Android APK     │               │   │  │
│  │     │  │ ✓ Android APK     │  │   only            │               │   │  │
│  │     │  │ ✓ iPhone          │  │                   │               │   │  │
│  │     │  │                   │  │ Requires direct   │               │   │  │
│  │     │  │ Requires:         │  │ APK install       │               │   │  │
│  │     │  │ • KYC verified    │  │                   │               │   │  │
│  │     │  │ • Wi-Fi           │  │ Requires:         │               │   │  │
│  │     │  │                   │  │ • KYC verified    │               │   │  │
│  │     │  └───────────────────┘  └───────────────────┘               │   │  │
│  │     │                                                             │   │  │
│  │     └─────────────────────────────────────────────────────────────┘   │  │
│  │                                                                       │  │
│  │     ┌─────────────────────────────────────────────────────────────┐   │  │
│  │     │  COMING SOON                                    [PROJECTED] │   │  │
│  │     ├─────────────────────────────────────────────────────────────┤   │  │
│  │     │                                                             │   │  │
│  │     │  ┌───────────────────┐  ┌───────────────────┐               │   │  │
│  │     │  │ 🔮 Entropy        │  │ 🖥️ GPU            │               │   │  │
│  │     │  │                   │  │    Contribution   │               │   │  │
│  │     │  │ ───────────────── │  │ ───────────────── │               │   │  │
│  │     │  │ ~$0.10 / day      │  │ ~$0.10 / day      │               │   │  │
│  │     │  │ (estimated)       │  │ (estimated)       │               │   │  │
│  │     │  │                   │  │                   │               │   │  │
│  │     │  │ All mobile        │  │ Windows GPU       │               │   │  │
│  │     │  │ devices           │  │ laptop only       │               │   │  │
│  │     │  └───────────────────┘  └───────────────────┘               │   │  │
│  │     │                                                             │   │  │
│  │     └─────────────────────────────────────────────────────────────┘   │  │
│  │                                                                       │  │
│  │     💡 Tasks stack! Run multiple tasks to earn more.                  │  │
│  │                                                                       │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Earnings Calculator Section

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     SECTION 3: EARNINGS CALCULATOR                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                                                                       │  │
│  │     Estimate Your Earnings                                            │  │
│  │     ══════════════════════════                                        │  │
│  │     See what you could earn with your devices                         │  │
│  │                                                                       │  │
│  │     ┌─────────────────────────────────────────────────────────────┐   │  │
│  │     │                                                             │   │  │
│  │     │  Your Devices                         Your Estimate         │   │  │
│  │     │  ────────────────                     ──────────────        │   │  │
│  │     │                                                             │   │  │
│  │     │  Android (Play Store)  [ 0 ] ▼        ┌─────────────────┐   │   │  │
│  │     │                                       │                 │   │   │  │
│  │     │  Android (APK)         [ 1 ] ▼        │  Monthly ULO    │   │   │  │
│  │     │                                       │  ═══════════    │   │   │  │
│  │     │  iPhone                [ 0 ] ▼        │                 │   │   │  │
│  │     │                                       │    $3.00        │   │   │  │
│  │     │  Windows GPU           [ 0 ] ▼        │                 │   │   │  │
│  │     │                                       │  (current tasks)│   │   │  │
│  │     │  ────────────────────────────         │                 │   │   │  │
│  │     │                                       │    $4.50        │   │   │  │
│  │     │  ☑ Include projected tasks            │                 │   │   │  │
│  │     │  ☑ KYC verified                       │  (with Entropy) │   │   │  │
│  │     │                                       │                 │   │   │  │
│  │     │                                       └─────────────────┘   │   │  │
│  │     │                                                             │   │  │
│  │     │  [     Open Full Calculator     ]                           │   │  │
│  │     │                                                             │   │  │
│  │     └─────────────────────────────────────────────────────────────┘   │  │
│  │                                                                       │  │
│  │     ⚠️ Actual earnings depend on task availability, device            │  │
│  │        activity, and network conditions. This is an estimate only.    │  │
│  │                                                                       │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### How It Works Section (Simplified)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      SECTION 4: HOW IT WORKS (SIMPLIFIED)                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                                                                       │  │
│  │     How It Works                                                      │  │
│  │     ════════════════                                                  │  │
│  │     Start earning in 3 simple steps                                   │  │
│  │                                                                       │  │
│  │     ┌─────────────┐      ┌─────────────┐      ┌─────────────┐         │  │
│  │     │      1      │      │      2      │      │      3      │         │  │
│  │     │     📱      │ ───▶ │     ▶️      │ ───▶ │     💰      │         │  │
│  │     │             │      │             │      │             │         │  │
│  │     │  Download   │      │   Start     │      │    Get      │         │  │
│  │     │   the App   │      │  Earning    │      │    Paid     │         │  │
│  │     └─────────────┘      └─────────────┘      └─────────────┘         │  │
│  │                                                                       │  │
│  │     Get the DJED         Click Start         Rewards paid             │  │
│  │     Nodes app from       Earning, accept     to your account          │  │
│  │     Play Store,          terms, and begin    weekly                   │  │
│  │     App Store, or        earning instantly                            │  │
│  │     direct APK                                                        │  │
│  │                                                                       │  │
│  │     ──────────────────────────────────────────────────────────────    │  │
│  │                                                                       │  │
│  │     What We Cover                    What You Provide                 │  │
│  │     ══════════════                   ════════════════                 │  │
│  │     ✓ Licence fee (funded)           • Compatible device              │  │
│  │     ✓ Activation credits             • Stable internet                │  │
│  │     ✓ Platform operations            • Device activity                │  │
│  │                                                                       │  │
│  │     What You Receive                                                  │  │
│  │     ════════════════                                                  │  │
│  │     50% of distributable rewards (ULO share)                          │  │
│  │                                                                       │  │
│  │     💡 Note: KYC verification required for withdrawals over $5        │  │
│  │                                                                       │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### FAQ Section

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           SECTION 5: FAQ                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                                                                       │  │
│  │     Frequently Asked Questions                                        │  │
│  │     ══════════════════════════════                                    │  │
│  │                                                                       │  │
│  │     ┌─────────────────────────────────────────────────────────────┐   │  │
│  │     │ ▼ How much can I earn?                                      │   │  │
│  │     ├─────────────────────────────────────────────────────────────┤   │  │
│  │     │ Earnings depend on available tasks and device activity.     │   │  │
│  │     │ Current tasks pay $0.10/day each. Multiple tasks can run    │   │  │
│  │     │ simultaneously. Use the calculator above to estimate.       │   │  │
│  │     └─────────────────────────────────────────────────────────────┘   │  │
│  │                                                                       │  │
│  │     ┌─────────────────────────────────────────────────────────────┐   │  │
│  │     │ ▶ Why is KYC required?                                      │   │  │
│  │     └─────────────────────────────────────────────────────────────┘   │  │
│  │                                                                       │  │
│  │     ┌─────────────────────────────────────────────────────────────┐   │  │
│  │     │ ▶ What is Ugrid and why does it need APK?                   │   │  │
│  │     └─────────────────────────────────────────────────────────────┘   │  │
│  │                                                                       │  │
│  │     ┌─────────────────────────────────────────────────────────────┐   │  │
│  │     │ ▶ How do I get paid?                                        │   │  │
│  │     └─────────────────────────────────────────────────────────────┘   │  │
│  │                                                                       │  │
│  │     ┌─────────────────────────────────────────────────────────────┐   │  │
│  │     │ ▶ Can I use multiple devices?                               │   │  │
│  │     └─────────────────────────────────────────────────────────────┘   │  │
│  │                                                                       │  │
│  │     ┌─────────────────────────────────────────────────────────────┐   │  │
│  │     │ ▶ How do I stop participating?                              │   │  │
│  │     └─────────────────────────────────────────────────────────────┘   │  │
│  │                                                                       │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Mobile Layout

```
┌───────────────────────────────┐
│ MOBILE LAYOUT (320px-480px)   │
├───────────────────────────────┤
│                               │
│  ┌─────────────────────────┐  │
│  │ ☰  DJED NODES    🌐  ?  │  │
│  └─────────────────────────┘  │
│                               │
│  ┌─────────────────────────┐  │
│  │                         │  │
│  │   Earn rewards with     │  │
│  │   a device you          │  │
│  │   already own           │  │
│  │                         │  │
│  │   We fund your licence. │  │
│  │   You get 50% rewards.  │  │
│  │                         │  │
│  │  ┌───────────────────┐  │  │
│  │  │  Start Earning ▶  │  │  │
│  │  └───────────────────┘  │  │
│  │                         │  │
│  │   How it Works ↓        │  │
│  │                         │  │
│  └─────────────────────────┘  │
│                               │
│  ┌─────────────────────────┐  │
│  │ ┌─────┐ ┌─────┐ ┌─────┐ │  │
│  │ │No   │ │$0.10│ │ 50% │ │  │
│  │ │fee  │ │/task│ │share│ │  │
│  │ └─────┘ └─────┘ └─────┘ │  │
│  └─────────────────────────┘  │
│                               │
│        ▼ SCROLL ▼             │
│                               │
│  ┌─────────────────────────┐  │
│  │   [Phone Screenshot]    │  │
│  │                         │  │
│  │    Sample dashboard     │  │
│  └─────────────────────────┘  │
│                               │
│  ┌─────────────────────────┐  │
│  │ Available Tasks         │  │
│  │ ────────────────        │  │
│  │                         │  │
│  │ ┌─────────────────────┐ │  │
│  │ │ 📊 Telemetry        │ │  │
│  │ │ $0.10/day           │ │  │
│  │ │ Android, iPhone     │ │  │
│  │ └─────────────────────┘ │  │
│  │                         │  │
│  │ ┌─────────────────────┐ │  │
│  │ │ ⚡ Ugrid             │ │  │
│  │ │ $0.10/day           │ │  │
│  │ │ Android APK only    │ │  │
│  │ └─────────────────────┘ │  │
│  │                         │  │
│  └─────────────────────────┘  │
│                               │
└───────────────────────────────┘
```

---

## Phase 5: Localization & Launch (HP-10, HP-11, HP-12)

### Objectives
- Update all 12 locale translations
- Integrate calculator
- Measure activation funnel
- Controlled pilot launch

### Locale Update Requirements

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         LOCALE UPDATE MATRIX                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  LOCALE  │ PRIORITY │ REMOVE                      │ ADD                     │
│  ────────┼──────────┼─────────────────────────────┼─────────────────────────│
│  en      │ P0       │ earnings tier keys          │ task-based keys         │
│  tl (PH) │ P0       │ tier translations           │ task translations       │
│  sw (NG) │ P0       │ tier translations           │ task translations       │
│  hi      │ P1       │ tier translations           │ task translations       │
│  bn      │ P1       │ tier translations           │ task translations       │
│  es      │ P2       │ tier translations           │ task translations       │
│  pt      │ P2       │ tier translations           │ task translations       │
│  fr      │ P2       │ tier translations           │ task translations       │
│  ar      │ P2       │ tier translations (RTL)     │ task translations (RTL) │
│  id      │ P2       │ tier translations           │ task translations       │
│  th      │ P2       │ tier translations           │ task translations       │
│  vi      │ P2       │ tier translations           │ task translations       │
│                                                                             │
│  New Keys Required:                                                         │
│  ─────────────────────────────────────────────────────────────────────────  │
│  • home.tasks.title = "Available Tasks"                                     │
│  • home.tasks.current = "Current Tasks"                                     │
│  • home.tasks.projected = "Coming Soon"                                     │
│  • home.tasks.telemetry.name = "Extended Telemetry"                         │
│  • home.tasks.telemetry.rate = "$0.10/day"                                  │
│  • home.tasks.ugrid.name = "Ugrid"                                          │
│  • home.tasks.ugrid.note = "Android APK only"                               │
│  • home.calculator.title = "Estimate Your Earnings"                         │
│  • home.calculator.devices = "Your Devices"                                 │
│  • home.calculator.estimate = "Your Estimate"                               │
│  • home.kyc.tooltip = "Note: KYC required for withdrawals over $5"          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Analytics Event Schema

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                          ANALYTICS FUNNEL EVENTS                             │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  EVENT                      │ TRIGGER                    │ PROPERTIES        │
│  ───────────────────────────┼────────────────────────────┼───────────────────│
│  landing_view               │ Page load complete         │ locale, source    │
│  calculator_used            │ Device count changed       │ devices[], tasks[]│
│  start_earning_clicked      │ CTA button click           │ -                 │
│  lease_terms_viewed         │ Terms modal opened         │ offer_version     │
│  lease_accepted             │ Accept button clicked      │ offer_version     │
│  claim_succeeded            │ Atomic claim committed     │ claim_id, device  │
│  provider_activated         │ Provider confirms active   │ task_ids[]        │
│  first_reward               │ First ULO credited         │ amount, task_id   │
│  d7_active                  │ 7 days post-activation     │ total_ulo         │
│  d30_retained               │ 30 days post-activation    │ total_ulo, active │
│  kyc_completed              │ KYC verification success   │ duration_ms       │
│  withdrawal_requested       │ User requests withdrawal   │ amount, kyc_status│
│                                                                              │
│  Funnel Visualization (Simplified - No Auth/KYC Blocking):                   │
│  ─────────────────────────────────────────────────────────                   │
│                                                                              │
│  landing_view ────────────────────────────────────────────▶ 100%             │
│       │                                                                      │
│       ▼                                                                      │
│  start_earning_clicked ───────────────────────────────────▶  40%             │
│       │                                                                      │
│       ▼                 (no auth/KYC barrier)                                │
│  lease_accepted ──────────────────────────────────────────▶  35%  ⬆ +15%     │
│       │                                                                      │
│       ▼                                                                      │
│  claim_succeeded ─────────────────────────────────────────▶  32%  ⬆ +14%     │
│       │                                                                      │
│       ▼                                                                      │
│  provider_activated ──────────────────────────────────────▶  28%  ⬆ +13%     │
│       │                                                                      │
│       ▼                                                                      │
│  d7_active ───────────────────────────────────────────────▶  22%  ⬆ +10%     │
│       │                                                                      │
│       ▼                                                                      │
│  d30_retained ────────────────────────────────────────────▶  15%  ⬆ +7%      │
│                                                                              │
│  KYC tracked separately (deferred to withdrawal):                            │
│  kyc_completed ──────────────────────────────────────▶ (at withdrawal >$5)   │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## Complete System Sequence Diagram

```
┌──────────────────────────────────────────────────────────────────────────────┐
│              SIMPLIFIED USER JOURNEY SEQUENCE DIAGRAM                        │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  User        Homepage               Lease        Provider                    │
│   │             │                     │             │                        │
│   │──Visit────▶│                      │             │                        │
│   │            │                      │             │                        │
│   │◀──Render───│                      │             │                        │
│   │  (tasks,   │                      │             │                        │
│   │   calc)    │                      │             │                        │
│   │            │                      │             │                        │
│   │──Use Calc─▶│                      │             │                        │
│   │◀─Estimate──│                      │             │                        │
│   │            │                      │             │                        │
│   │──Start ───▶│                      │             │                        │
│   │  Earning   │                      │             │                        │
│   │            │                      │             │                        │
│   │            │    ┌────────────────────────────────────┐                   │
│   │            │    │ Show Lease Terms + KYC tooltip     │                   │
│   │◀──Terms────│◀───│ "Note: KYC required for            │                   │
│   │  + tooltip │    │  withdrawals over $5"              │                   │
│   │            │    └────────────────────────────────────┘                   │
│   │            │                      │             │                        │
│   │──Accept───▶│                      │             │                        │
│   │            │─────Atomic Claim────▶│             │                        │
│   │            │                      │──Reserve───▶│                        │
│   │            │                      │◀──OK────────│                        │
│   │            │◀────Claim Receipt────│             │                        │
│   │◀──Receipt──│                      │             │                        │
│   │            │                      │             │                        │
│   │            │                      │──Activate──▶│                        │
│   │            │                      │◀──Active────│                        │
│   │            │                      │             │                        │
│   │◀─Dashboard─│◀─────────────────────│             │                        │
│   │  (earning) │                      │             │                        │
│   │            │                      │             │                        │
│   │            │                      │──Rewards───▶│                        │
│   │            │                      │◀──ULO───────│                        │
│   │            │                      │             │                        │
│   │◀──Balance──│◀─────────────────────│             │                        │
│   │  update    │                      │             │                        │
│   │            │                      │             │                        │
│                                                                              │
│  NOTE: Auth and KYC removed from critical path                               │
│  ─────────────────────────────────────────────                               │
│  • No login required before claiming                                         │
│  • KYC only shown as informational tooltip                                   │
│  • User can complete KYC later when withdrawing >$5                          │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## Earnings Calculation Flow

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                     EARNINGS CALCULATION SEQUENCE                            │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Calculator      TaskStore       Engine          Display                     │
│      │               │              │               │                        │
│      │──Get Tasks───▶│              │               │                        │
│      │◀──TaskList────│              │               │                        │
│      │               │              │               │                        │
│      │    User inputs devices       │               │                        │
│      │    ─────────────────         │               │                        │
│      │    android_apk: 1            │               │                        │
│      │    include_projected: true   │               │                        │
│      │    kyc_verified: true        │               │                        │
│      │               │              │               │                        │
│      │──────────────────Calculate──▶│               │                        │
│      │               │              │               │                        │
│      │               │    ┌─────────────────────┐   │                        │
│      │               │    │ For each device:    │   │                        │
│      │               │    │   For each task:    │   │                        │
│      │               │    │     If supported:   │   │                        │
│      │               │    │       pool += rate  │   │                        │
│      │               │    │       × activity    │   │                        │
│      │               │    │       × days        │   │                        │
│      │               │    └─────────────────────┘   │                        │
│      │               │              │               │                        │
│      │               │    ┌─────────────────────┐   │                        │
│      │               │    │ Apply shares:       │   │                        │
│      │               │    │   ulo = pool × 0.5  │   │                        │
│      │               │    │   uno = pool × 0.4  │   │                        │
│      │               │    │   ref = pool × 0.1  │   │                        │
│      │               │    └─────────────────────┘   │                        │
│      │               │              │               │                        │
│      │◀─────────────────Result──────│               │                        │
│      │               │              │               │                        │
│      │    Result:                   │               │                        │
│      │    ─────────                 │               │                        │
│      │    pool: $6.00               │               │                        │
│      │    ulo:  $3.00 (50%)         │               │                        │
│      │    uno:  $2.40 (40%)         │               │                        │
│      │    ref:  $0.60 (10%)         │               │                        │
│      │               │              │               │                        │
│      │───────────────────────────────────Render────▶│                        │
│      │               │              │               │                        │
│      │               │              │    ┌──────────────────┐                │
│      │               │              │    │  Monthly ULO     │                │
│      │               │              │    │  ═══════════     │                │
│      │               │              │    │    $3.00         │                │
│      │               │              │    │  (current tasks) │                │
│      │               │              │    └──────────────────┘                │
│      │               │              │               │                        │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## Implementation Checklist

### Phase 1 Checklist
- [ ] Create claim evidence register document
- [ ] Audit all 12 locale files for unsupported claims
- [ ] Remove earnings tier components from `home.rs`
- [ ] Remove tier translation keys from all locales
- [ ] Define sponsored offer copy
- [ ] Get stakeholder approval on new messaging

### Phase 2 Checklist
- [ ] Create `TaskDefinition` type in `src/types/task.rs`
- [ ] Create `TaskStore` static data module
- [ ] Create `TaskCard` component
- [ ] Create simplified `EarningsCalculator` component (Leptos)
- [ ] Integrate full calculator HTML as iframe/link option
- [ ] Add task-based translation keys to all locales

### Phase 3 Checklist
- [ ] Implement "Start Earning" button flow (direct to terms)
- [ ] Create lease terms acceptance modal with KYC tooltip
- [ ] Add non-blocking eligibility reminder tooltip
- [ ] Implement atomic claim reservation (no auth required)
- [ ] Create journey state tracking
- [ ] Add deferred KYC flow for withdrawals >$5

### Phase 4 Checklist
- [ ] Redesign hero section with real screenshot
- [ ] Create task grid component
- [ ] Create inline calculator preview
- [ ] Simplify "How it Works" to 4 steps
- [ ] Update FAQ with task-based answers
- [ ] Mobile layout testing (320px)
- [ ] RTL layout testing (Arabic)

### Phase 5 Checklist
- [ ] Update all 12 locale translations
- [ ] Implement analytics events
- [ ] Set up funnel dashboard
- [ ] Nigeria pilot deployment
- [ ] Philippines pilot deployment
- [ ] D7/D30 retention tracking
- [ ] Launch gate verification

---

## Technical Dependencies

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         TECHNICAL DEPENDENCIES                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  uno-app (Frontend - Leptos)                                                │
│  ├── src/types/task.rs ────────────────────── Task definitions              │
│  ├── src/components/home/                                                   │
│  │   ├── task_grid.rs ─────────────────────── Task cards display            │
│  │   ├── earnings_calculator.rs ───────────── Inline calculator             │
│  │   ├── hero_section.rs ──────────────────── Updated hero                  │
│  │   └── how_it_works.rs ──────────────────── Simplified steps              │
│  ├── src/routes/home.rs ───────────────────── Main page composition         │
│  └── src/locales/*.rs ─────────────────────── Updated translations          │
│                                                                             │
│  uno-api (Backend)                                                          │
│  ├── src/services/task_service.rs ─────────── Task catalogue API            │
│  ├── src/services/kyc_service.rs ──────────── KYC status check              │
│  └── src/services/lease_service.rs ────────── Lease acceptance              │
│                                                                             │
│  uno-admin (Admin Panel)                                                    │
│  ├── Task management UI ───────────────────── Add/edit tasks                │
│  └── Analytics dashboard ──────────────────── Funnel metrics                │
│                                                                             │
│  External                                                                   │
│  ├── KYC Provider ─────────────────────────── Identity verification         │
│  └── UNetwork Provider ────────────────────── Task/reward activation        │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Calculator estimates differ from actual | Clear disclaimers, use "estimate" language |
| KYC provider downtime | Graceful degradation, retry flow |
| Task availability changes | Dynamic task catalogue, CMS-managed |
| Locale translation errors | Human review for critical terms |
| Mobile performance | <500KB initial transfer, lazy loading |

---

## Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| CTA → Lease acceptance | >35% | Analytics funnel (no auth/KYC barrier) |
| Lease → Claim success | >90% | Analytics funnel |
| D7 retention | >60% | Cohort tracking |
| D30 retention | >40% | Cohort tracking |
| Mobile LCP | <2.5s | Lighthouse |
| Translation coverage | 100% | Build check |
| KYC at withdrawal | tracked | Deferred metric |

---

## Appendix: File Changes Summary

| Phase | Files Created | Files Modified |
|-------|---------------|----------------|
| Phase 1 | 1 (evidence register) | 13 (home.rs + 12 locales) |
| Phase 2 | 4 (task types, service, components) | 2 (home.rs, mod.rs) |
| Phase 3 | 3 (KYC, lease, journey services) | 2 (auth, handlers) |
| Phase 4 | 5 (hero, tasks, calc, how, faq components) | 3 (home.rs, styles) |
| Phase 5 | 2 (analytics, dashboard) | 12 (all locales) |

**Total: ~15 new files, ~32 modified files**

---

*Document prepared: 2 October 2026*
*Next review: After Phase 1 completion*
