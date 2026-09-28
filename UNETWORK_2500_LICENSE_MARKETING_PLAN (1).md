# Unetwork: a cost-controlled plan to activate 2,500 ULO licences

**Prepared:** 28 September 2026  
**Revision 2:** historical 50:50 interpretation; proposed ULO 50% / UNO 40% / referral 10%; UNO-funded credits; CSV-only reward evidence.  
**Objective:** build a repeatable acquisition operation capable of at least 250 net additional productive licences per week, until 2,500 are productively deployed.  
**Basis:** the supplied incentive CSV, the owner’s clarifications and public participation terms. No estimated daily task rates are used in this revision. Budgets, conversion targets and market allocations remain planning assumptions.

## 1. Recommended strategy and conditions for success

Use **trusted local community partners, adult digital-skills groups, small creators and one-level referrals**, supported by short onboarding clinics. Recruit people who already own suitable phones and already have affordable, stable internet. Begin in two markets; expand into six only after local operation and withdrawals are demonstrated. Retain international reach through an eligibility-screened waiting list.

Position the opportunity as **small supplementary rewards for optional device tasks**, with clearly disclosed costs. It is not employment, a salary, guaranteed daily income or a reason to buy a phone, broadband connection or GPU.

Design for **approximately 300 day-seven productive activations per week**, providing room for churn while pursuing **250 net additions**. A nominal distribution count is inadequate: codes handed out, registrations and inactive devices are not successful deployment.

The revised offer is **50% ULO / 40% UNO / 10% referral of the distributable licence reward pool**, with **the UNO paying all activation credits**. The ULO pays no licence-acquisition fee or platform credit charge under this offer. Internet, electricity, device wear, withdrawal costs and time may still matter. The referral percentage comes from the UNO’s previous 50% allocation, preserving the ULO’s historical 50% share.

The historical split is confirmed by the owner as 50:50, and the export covers only licences distributed within its timeframe. The calculations below **interpret `ups` as the UNO’s credited share**, because this is presented as a UNO incentive export. Confirm that column’s recipient definition against the ledger before treating reconstructed totals as verified pool amounts. If `ups` is already the combined pool, do not double it. There is no basis for treating the export as 2,500 fully active licences.

**Three conditions must be resolved before broad recruitment:**

1. **Promotion permission:** terms §5.6 broadly restrict outside-ecosystem promotion or facilitation of operator-licence transfers/sublicensing. Obtain a platform-approved recruitment/referral arrangement, permitted copy and official activation route. A free lease does not automatically remove this restriction. The external channels below are conditional on that arrangement; otherwise use authorised in-platform discovery and support only. [S2]
2. **Positive participant economics:** measure credited rewards against the actual paid licence-months of distributed devices. Confirm the `ups` recipient definition and actual credit charge. Do not apply another 75% haircut to already credited operator rewards.
3. **Proven participation:** verify device/country eligibility, credit pricing, actual task earnings, data use and withdrawal with a small pilot. Projected Entropy and Windows GPU rewards do not fund the base plan.

This is a target-driven operating plan, not a guarantee that markets will produce 250 activations weekly. If the conversion or economics gates fail, adjust the offer or timeline rather than conceal costs or recruit unsuitable participants.

## 2. What the incentive CSV actually establishes

### File audit

`statistics-incentives-All.csv` contains **475 data records** covering **303 dates**, from **30 November 2025 through 28 September 2026**. Columns are `date`, `task`, `ups`, and `allocations`. There are no duplicate date/task keys, negative numeric values or missing required fields in the parsed records. The file has no final newline; line-count utilities can therefore give a misleading record count.

| Recorded task label | Records | Total UP | Allocations |
|---|---:|---:|---:|
| Proof of Work | 302 | 1,155.696068 | 16,667 |
| Other | 9 | 75.000000 | 30 |
| Scout | 27 | 190.090000 | 3,545 |
| CLI / Caller-ID | 135 | 20.867400 | 373 |
| Runner Calls | 2 | 0.808750 | 13 |
| **Total** | **475** | **1,442.462218** | **20,628** |

“Allocations” is not documented here as a count of unique devices, licence-days or verified calls. Do not use it as any of those denominators. Likewise, “Proof of Work” cannot be confidently remapped to Extended Telemetry or Ugrid without an export definition. [S1]

| Comparison window | Recorded UP | Interpretation |
|---|---:|---|
| July 2026 | 249.031293 | Historical aggregate, mixed tasks |
| August 2026 | 46.585012 | Lower aggregate; cause unknown |
| 31 August–27 September, 28 calendar days | 32.385439 | About 1.1566 UP/day for the exported scope |
| 21–27 September, seven calendar days | 10.071396 | About 1.4388 UP/day for the exported scope |
| 28 September | 0.033500 | Latest date may be incomplete; excluded from recent completed-date windows |

The owner confirms a **historical 50% UNO / 50% ULO split** and that the export covers **licences distributed within the stated timeframe**, not all owned licences. It still does not identify distributed counts, deployment dates, paid licence-months, country, device or uptime. The `ups` recipient is interpreted as UNO credit for the reconstruction below, subject to ledger confirmation. The fall from July to August cannot be attributed to reduced rates, churn or demand without those fields. The recent totals are not a defensible portfolio revenue forecast.

### Additional data needed for a decision-grade pilot

Capture daily licence ID, pseudonymous operator ID, country, device/OS, enabled task, accepted task result, distributable reward, ULO share, UNO share, credits payer/cost, uptime, data usage and withdrawal status. Obtain definitions for allocation events and export scope. Keep KYC documents with the official verification provider, not in a marketing spreadsheet.

Measure net reward by device-day and by country/OS/task cohort. Separate rejected work, missing observations and zero-reward days. Keep the original export intact. Use actual medians and lower-quartile results for messaging, rather than the highest-performing device.

## 3. Task strategy: use recorded CSV evidence

This section replaces the supplied approximate daily rates with the export’s actual task labels and credits. No per-device yield or per-call rate can be calculated without the corresponding denominator. [S1]

| CSV task | Recorded UP, historical export | Latest recorded date | Treatment in promotion |
|---|---:|---|---|
| Proof of Work | 1,155.696068 | 27 Sep 2026 | Main recorded reward category; establish its mapping to current task names before promoting specific tasks |
| Scout | 190.090000 | 24 Jul 2026 | Historical only; verify whether currently available |
| Other | 75.000000 | 3 Jun 2026 | Unclassified; exclude from recurring task forecasts until identified |
| CLI / Caller-ID | 20.867400 | 28 Sep 2026 | Recently recorded but limited aggregate contribution; do not promise call volume |
| Runner Calls | 0.808750 | 23 Jul 2026 | Sparse historical evidence; no dependable recurring assumption |

In the 28-calendar-day window ending 27 September, **Proof of Work contributed 27.746939 UP and CLI 4.638500 UP**; the remaining categories contributed zero recorded credits. These are aggregate historical values, not device yields. Absence of recent credits does not establish that a task has been discontinued.

**Extended Telemetry, Ugrid, Entropy and GPU Contribution are not separately identified in this export.** Their earnings cannot be inferred by relabelling Proof of Work or adding estimated rates. The owner’s task names can guide compatibility tests, but marketing reward claims must use traceable task/device records. GPU and Entropy contribute zero to this plan’s revenue evidence; APK/iOS differences remain eligibility questions.

Use accurate copy: “The supplied historical export records Proof of Work and Caller-ID credits recently; individual results depend on eligible work.” Do not promote the portfolio aggregate as a typical participant outcome. Obtain a task-label mapping and recent device-level export before publishing a per-task earnings range.

For Windows, separately verify platform permission, licensing, actual credits and incremental electricity. The existing mobile-device terms need reconciliation with Windows participation. Without measured reward/power evidence, neither a GPU recruitment campaign nor GPU purchases are justified by this file. [S2]

## 4. Economics of ULO 50% / UNO 40% / referral 10%

### Allocation rules and historical reconstruction

Let `H` be the historical UNO credit in the CSV, on the working interpretation that `ups` records the UNO’s 50% share. The matching distributable pool is `P = 2H`. Replaying the same pool under the proposed split gives:

- ULO: `0.50P = H`, unchanged from its historical percentage.
- UNO: `0.40P = 0.80H`, before UNO-paid credits and support.
- Referral: `0.10P = 0.20H`.

The referral receives **10 percentage points of the entire distributable pool**, equivalent to 20% of the previous UNO credit. It is not 10% of the UNO’s new share. No additional upstream 75/25 deduction is applied to reconstructed credits. These are counterfactual allocations, not new realised payments.

| CSV category | Recorded H | Reconstructed pool P | New UNO 40% | ULO 50% | Referral 10% |
|---|---:|---:|---:|---:|---:|
| Proof of Work | 1,155.696068 | 2,311.392136 | 924.556854 | 1,155.696068 | 231.139214 |
| Other | 75.000000 | 150.000000 | 60.000000 | 75.000000 | 15.000000 |
| Scout | 190.090000 | 380.180000 | 152.072000 | 190.090000 | 38.018000 |
| CLI / Caller-ID | 20.867400 | 41.734800 | 16.693920 | 20.867400 | 4.173480 |
| Runner Calls | 0.808750 | 1.617500 | 0.647000 | 0.808750 | 0.161750 |
| **Total** | **1,442.462218** | **2,884.924436** | **1,153.969774** | **1,442.462218** | **288.492444** |

All values are UP. USD equivalence uses the documented 1 UP = $1 reference convention, not evidence of realised fiat proceeds. Credits and support are not included in the allocation columns. The period mixes deployments and task regimes; lifetime totals are unsuitable for monthly performance extrapolation.

For 31 August–27 September, `H = 32.385439`, giving a reconstructed pool of **64.770878 UP**, UNO **25.908351 UP**, ULO **32.385439 UP** and referral **6.477088 UP** for that distributed cohort and window. Normalising the UNO amount to 30 days gives about **27.758948 UP**, but does not forecast future revenue.

### Break-even conditions for UNO-funded credits

Use actual bills when available. The following retains **$1.99/month credit cost** as a planning input and **$0.25/month support allowance**, not facts established by the export. Let `P_m` be realised monthly reward pool per paid licence, including zero-reward paid licences in the denominator.

`UNO contribution = 0.40 × P_m − credit cost − support cost`

| Cost basis | Required pool per paid licence/month | Equivalent historical 50% UNO credit/month |
|---|---:|---:|
| $1.99 credits only | $4.975 | $2.4875 |
| $1.99 credits + $0.25 support | **$5.60** | **$2.80** |
| $3.99 credits + $0.25 support | **$10.60** | **$5.30** |

At the $5.60 threshold, ULO receives $2.80, referral $0.56, UNO $2.24 and UNO costs consume $2.24. The operator bears no credits charge, but the UNO has no margin left for acquisition, tax, fixed overhead or licence capital recovery.

Sensitivity values below are **arbitrary pool levels for break-even analysis**, not estimated task rates or CSV-derived device performance:

| Monthly pool per paid licence | ULO 50%, no credit deduction | Referral 10% | UNO 40% | UNO after $1.99 credits + $0.25 support |
|---|---:|---:|---:|---:|
| $3.00 | $1.50 | $0.30 | $1.20 | −$1.04 |
| $5.00 | $2.50 | $0.50 | $2.00 | −$0.24 |
| $5.60 | $2.80 | $0.56 | $2.24 | $0.00 |
| $7.50 | $3.75 | $0.75 | $3.00 | $0.76 |
| $10.00 | $5.00 | $1.00 | $4.00 | $1.76 |

The CSV does not supply paid licence-months, so it cannot show which row represents the distributed population. The 30-day-normalised recent UNO credit would cover at most about **13 fully billed licence-months of $1.99 credits alone**, or **12 including $0.25 support**, if it represented the entire comparable cohort’s income. This is a conditional budget bound, not an estimate of the actual number of distributed or active licences.

### Required denominator and acquisition cap

Record distribution/activation date, renewal dates, credits actually paid, task credits and releases per licence. For continuous proportional pricing, use billed licence-days divided by 30. If credits are sold in full-month blocks, use actual purchased blocks for cash profitability; do not prorate away non-refundable charges. The distribution timeframe alone is not enough to calculate these quantities.

For an observed historical cohort, `H_per_paid_month = total UNO credits / paid licence-months`. Then revised contribution is `0.8 × H_per_paid_month − C − S`.

A three-month simple acquisition-payback ceiling is `3 × positive monthly contribution`, with a further reduction for churn. A $2 acquisition target therefore requires a pool of about **$7.27/month** under the $1.99/$0.25 cost inputs. At break-even, sustainable acquisition spending is zero. The former $0.71 contribution and fixed $2 CAC justification have been withdrawn.

### Referral agreement

Use a **single-level 10% recurring share of accepted rewards from directly attributed licence participation**, subject to platform-supported settlement or an explicitly authorised operator accounting arrangement. No downstream commissions and no payment merely for recruiting another referrer. Pay from actual settled rewards, not projections.

Default proposal: the recurring share lasts while the specific attributed lease remains active, with no promised lifetime entitlement beyond that lease. State treatment of renewals, reassignment, reversals, payout threshold and disputes before enrolment. Preserve earned referral amounts if a lease ends. Do not claw back legitimate ULO earnings to fund referral adjustments.

For applicants with no referrer, hold the 10% in a separately tracked referral/support reserve until the offer specifies its destination; conservatively keep UNO at 40% in the model. Do not silently turn this into 50% UNO or fabricate a referrer. One licence has one attributable referral source, not several stacked 10% cuts.

**Assessment:** this arrangement is more accessible to ULOs and gives promoters recurring upside, but transfers credit and acquisition risk to the UNO. It is viable only if measured paid-licence rewards exceed the relevant threshold. No per-device profit claim can yet be justified from the supplied export alone.

## 5. Target audience and market sequencing

Recruit on **existing connectivity and device suitability**, not poverty alone. A low-income person with expensive mobile data may be a worse fit than a modest-income household with an existing fixed connection.

Best-fit profiles: adult vocational learners with home Wi-Fi; existing Android enthusiasts; salaried or self-employed adults seeking a small optional benefit; digital-skills community members; and households already using affordable broadband. Avoid dependency framing, emergency-income claims and recruitment of minors. The UNO pays credits; participants must not be asked for a deposit or to borrow money to join.

### Provisional six-market allocation

These are **campaign quotas**, not forecasts or country authorisations. The markets have public observation evidence in the earlier review; every task and withdrawal route still requires local validation. Low/middle-income localities can exist in multiple national income groups; use current World Bank classifications only as context, not as an eligibility proxy. [S4, S5]

| Market | Net weekly target | Ten-week net allocation | First partner hypothesis | Language approach |
|---|---:|---:|---|---|
| India | 60 | 600 | Adult vocational/digital-skills and Android communities | English plus one locally selected language |
| Philippines | 60 | 600 | Community tech groups and adult online-worker communities | English; Filipino adaptation after testing |
| Nigeria | 50 | 500 | Existing tech groups and adult training communities | English; local-language explainers where useful |
| Kenya | 35 | 350 | Digital-skills hubs and broadband-connected communities | English; Swahili support as needed |
| Bangladesh | 30 | 300 | Adult IT-training and Android groups | Bangla materials before expansion |
| Ghana | 15 | 150 | Local tech communities and training partners | English initially |
| **Total** | **250** | **2,500** | | |

Start with **two markets where the UNO can obtain trusted moderators and provide support**, provisionally Nigeria and the Philippines. This is an operational hypothesis, not a claim that they offer the best rates. Replace either if the pilot fails. Introduce remaining markets only with a local helper, proven eligibility and measured economics.

Second-wave discovery candidates: Indonesia, Vietnam, Nepal, Pakistan, Sri Lanka and suitable lower-income localities in Latin America. These receive **no launch quota** until tested. Maintain an international waiting list; do not label every unexcluded country “supported.”

Score each market out of 100: verified task/withdrawal access 25, participant net return 25, affordable existing connectivity 20, partner access 15, support/language fit 10, independent geographic distribution 5. Require 70+ and no failed eligibility/economics condition. These weights are planning choices.

## 6. Offer and distribution mechanics

The proposed offer: an official in-app licence lease with **no upfront licence-acquisition charge**, transparent 50% ULO / 40% UNO / 10% referral allocation and UNO-funded activation credits, task-specific eligibility and plain-language exit terms. Describe the licence and credits as UNO-funded; disclose remaining device, connection and withdrawal costs rather than promising cost-free earnings. Do not represent it as a gift of NFT ownership when it is a lease.

Use official claim/private-lease flows. Maintain three separate inventories: unassigned, assigned/in onboarding, and productive. Add expired/released/quarantined states for auditability. Every active or trial lease occupies inventory; concurrent occupancy must never exceed 2,500. More than 2,500 lifetime onboarding attempts are possible only by legitimately releasing and reassigning licences under actual platform/lease rules.

Use a unique partner/source code for attribution and a separate official licence code for activation. Do not post batches of reusable private codes in public groups. Reserve a code only after screening and provide a reasonable activation window; do not threaten arbitrary confiscation. Keep a standby queue once inventory is full.

Existing productive licences reduce the remaining recruitment requirement: `remaining = 2,500 − verified current productive licences`. The timetable below assumes none are counted initially because the CSV supplies no reliable active inventory. Complete that inventory check first.

## 7. Channel portfolio: breadth without spreading the budget thinly

All external execution is conditional on the platform-approved programme in Section 1. The figures below are management targets to test, not purchased reach or market benchmarks.

### Weekly engine at steady state

| Channel | Qualified opt-in prospects/week | Expected D7 productive at 30% end-to-end conversion | Concrete weekly activity |
|---|---:|---:|---|
| Partner organisations / community admins | 420 | 126 | 12 active partners × 35 opted-in prospects |
| Local ambassadors / small creators | 250 | 75 | 10 active ambassadors × 25 opted-in prospects |
| Existing-participant referrals | 180 | 54 | Approximately 90 referring participants × 2 opted-in prospects |
| Owned tutorials, community Q&A, organic discovery | 100 | 30 | Four useful tutorials, approved discussions, two group clinics |
| Small paid/boosted-content experiments | 50 | 15 | Capped controlled test; zero dependency until CAC proven |
| **Total** | **1,000** | **300** | No duplicate counting between sources |

“Opt-in prospect” means a person who actively requests information, not an impression or group membership. At launch there is no demonstrated referral base: replace the 180 referral prospects with approximately six extra partners generating 30 each. If that capacity cannot be signed, the 250/week target is not ready for launch. Likewise, partners and ambassadors must be distinct source cohorts when reporting.

### Channel execution and priority

| Strategy | Priority | Execution and cost control |
|---|---|---|
| Platform marketplace/community | Immediate | Optimise the authorised lease offer and support reputation; ask the platform for approved operator discovery. Best fallback if external promotion is not permitted. |
| Existing community admins | Core | Obtain moderator permission, give a transparent economics sheet, host one Q&A and track actual activation. Pay for retained outcomes, not member counts. |
| Adult training providers and alumni groups | Core | Offer an optional device/network literacy demonstration outside class obligations. Institution participation is not blanket student consent. |
| Small local tech creators | Core test | Demonstrate real installation and verified pilot results. Prefer modest performance fees over large upfront sponsorships. Disclose compensation. |
| One-level referrals | Core after proof | Reward verified retention, not recruiting recruiters; no downlines, deposits or earnings from recruitment fees. Cap cash liability. |
| Smartphone repair/accessory shops | Test | Approved leaflet/QR for customers who opt in; no staff installation without device-owner consent. Avoid commissions that exceed CAC ceiling. |
| Broadband/ISP community partnerships | Test | Ask for an approved informational placement to already-connected customers. Never imply internet service is paid for by the rewards. |
| Libraries, community centres, digital-inclusion groups | Test | Optional adult workshop and honest economics; individual daily connectivity is still required. Shared venues are for onboarding, not phone farms. |
| Facebook groups / local forums | Core distribution surface | Admin-approved local posts and Q&A; no repetitive group blasting. State costs before the click. |
| WhatsApp / Telegram | Onboarding and retention | User-initiated or opted-in conversations, country-specific announcements and help; no purchased lists, scraped members or unsolicited bulk DMs. |
| YouTube / short video / creator live sessions | Compounding channel | Screen-record setup, explain payouts and compare costs; archive current instructions. Test small creators before paying larger ones. |
| Reddit / Discord / technical forums | Selective | Answer relevant questions, disclose operator affiliation and obtain moderator permission. Do not treat communities as ad inventory. |
| Search/SEO and local-language tutorials | Supporting | Publish accurate task compatibility and troubleshooting pages. Valuable after launch but unlikely to guarantee immediate volume. |
| Opt-in email | Supporting | Explain eligibility, finish onboarding and send concise service updates. No purchased lists. |
| Existing newsletters | Test | Small placement with a unique link and cost cap; retain only if measured D30 CAC passes. |
| Paid social | Small experiment | Review channel rules for crypto-linked rewards; disclose the actual product, never disguise it to evade review. Kill unprofitable campaigns promptly. |
| Search ads | Low priority | High-intent queries only; small capped trial if platform-approved. Low margins rarely justify broad bidding. |
| Local radio / podcasts | Low priority | Unpaid interview or inexpensive attributable placement; avoid broad spend with no device screening. |
| Events / meetups / QR print | Selective | Piggyback existing gatherings; no paid venue or travel-heavy roadshows. Use one traceable QR per venue. |
| Press / earned media | Supporting | Pitch genuine digital participation findings; no paid “guaranteed income” articles. |
| Microtask / job boards | Usually avoid | This is not a conventional paid job. Only use a venue that permits this exact optional participation model and fee disclosures. |
| Influencer agencies / affiliate networks | Defer | Fixed retainers and generic incentive traffic are likely to exceed lifetime margin. Reconsider only with verified retained CAC. |
| Giveaways / contests | Avoid as main engine | Attract prize seekers and obscure long-term economics. A small onboarding pilot reimbursement is a separately budgeted expense. |
| Device farms / emulator traffic / bought accounts | Reject | Violates the intended real-device model and undermines geographic diversity; do not attempt network-origin evasion. |

WhatsApp business messaging requires suitable opt-in and respecting opt-outs; Reddit prohibits unsolicited mass engagement and communities set their own promotion rules. These are operating constraints, not reasons to avoid all community outreach. [S6, S7]

## 8. Partner recruitment playbook

### Build the supply of promoters before promising the weekly volume

During preparation, identify 100 relevant organisations/admins across the initial two markets. Record public contact route, audience fit, language, existing connectivity, permission requirements and estimated support burden. This is a research list, not a purchased consumer contact list.

Planning funnel: 100 individually assessed partner approaches → 40 conversations → 24 demonstration sessions → 18 launch partners → at least 12 consistently productive partners. These ratios are targets to validate. If only six become productive, either double their demonstrated capacity or extend preparation; do not assume the missing output.

A coordinator handles roughly 20 tailored approaches per business day for one week, followed by demonstrations and written agreements. Contact organisational/public business channels appropriately; consumer recruitment begins only with permission. No messages have been sent as part of preparing this plan.

Give each partner a one-page brief: eligibility, realistic net examples, exact cost payer, official links, prohibited claims, support route, attribution and compensation. Include a “who should not join” box: costly data, negative measured net outcomes, unsupported phone/country, unwillingness to complete required verification, or expectation of salary-scale income.

### Compensation model

Replace the default fixed acquisition bounty with the **single-level 10% recurring referral allocation in Section 4**. Partners and ambassadors receive only the share for their directly attributed productive licences, after reward reconciliation. Display the full three-way split to the ULO; do not call the referral payment an additional deduction from the ULO’s 50%.

Explain the likely small size honestly. If a referred licence’s monthly pool were $5.60, its referrer would receive $0.56/month; ten such licences would yield $5.60/month and fifty would yield $28/month. These are sensitivity examples, not promised income. A creator may find this insufficient: the partner funnel must be revalidated under recurring-share terms rather than carrying forward assumed acceptance of cash bounties.

No automatic upfront acquisition bounty is layered on top. A separately approved pilot support payment may compensate documented setup work, but must be included in cost-per-retained-operator and funded from the acquisition budget. Pay no commission for KYC submissions, fake installs, self-referrals or duplicate attribution. Referrers do not collect documents, credentials, private keys, credits payments or registration fees.

Use a first-qualified-source rule with an agreed attribution window and explicit dispute process. Preserve a ledger per licence: pool, ULO allocation, UNO allocation, referral allocation, credits paid by UNO, adjustments and settled referral payout. The ULO never pays the promoter.

## 9. Onboarding and retention funnel

### Capacity model

| Stage | Assumed conversion | Weekly count |
|---|---:|---:|
| Opted-in prospects | — | 1,000 |
| Eligible: country/device/connectivity/cost understanding | 70% | 700 |
| Install/account ready | 70% of eligible | 490 |
| Licence bound and first accepted reward | 72% | 353 approximately |
| D7 productive | 85% of activated | 300 approximately |
| D30 retained | 75% of activated, provisional | 265 approximately |

D30 is not 75% of D7: both denominators are the original activated cohort. All conversions are unvalidated assumptions. Do not combine a weak country’s conversion with another country’s retention to claim a profitable average.

**D7 productive definition:** verified unique consenting adult; official active licence/device; accepted rewarded activity on at least four of the last seven days; no unresolved payout/credit blocker; applicable platform rules met. Proposed screening preference: internet available at least 90% of the participant’s intended operating period, with actual task qualification tracked separately. This preference does not override a lease requiring greater uptime.

### Journey and support

1. **Landing page:** plain benefit, small-reward caveat, credit cost/payer, device list, country screen, current vs projected tasks, reward split and official participation link.
2. **Two-minute eligibility form:** adult confirmation, country, Android/iOS/Windows, existing connection and data cap, charging reliability, languages, consent and acknowledgement that credits are UNO-funded and other costs may remain. Do not collect household income or identity-document images.
3. **Economics check:** show only a measured local pilot range when available, the UNO-funded credits, other costs and what happens if tasks are unavailable. Let unsuitable applicants exit without pressure.
4. **Official onboarding:** participant controls account, device permissions and KYC. For APK-only tasks, use current official downloads; never ask users to disable device security globally.
5. **Licence assignment:** bind in the official system, log attribution and inventory status, record UNO credit funding and first credited task.
6. **D1 contact:** resolve installation/connectivity failures, using opted-in reminders only.
7. **D3 check:** compare expected vs actual task availability and data use. Pause unsupported combinations rather than leaving a costly idle licence.
8. **D7 review:** quality check, first referral-ledger reconciliation, publish anonymised cohort metrics.
9. **Withdrawal support:** demonstrate the real process when the participant qualifies. Never promise instant cash-out or local-bank conversion without a supported route.
10. **D30 renewal:** show actual net outcome, confirm whether to continue, reconcile the recurring referral share and offer an optional referral link.

Do not publish a fixed withdrawal timeline without measured per-licence accrual. If the applicable minimum is 5 UP, time to threshold is `5 / measured daily ULO UP`, allowing for variable activity and fees. The previous 39-day example has been removed because it relied on an unverified task-rate estimate. Existing eligible accounts can verify the withdrawal process without pretending that newly enrolled ULOs will reach the minimum during the pilot.

Keep language-specific help sessions twice weekly, one pinned setup video and a searchable FAQ. A typical 353-activation week at two minutes average routine help uses about 12 hours; 15% requiring ten extra minutes adds nine hours. Budget roughly **21 support hours/week** at scale, plus coordination. If effort doubles, the $0.25/device support allowance fails and must be revised.

## 10. Timeline and 2,500-active inventory model

### Preparation and pilot: two weeks before the volume commitment

| Period | Deliverables | Exit test |
|---|---|---|
| Days 1–3 | Inventory reconciliation; platform permission request; confirm UNO credit billing and CSV recipient definition; partner shortlist | No unresolved blocking assumption disguised as fact |
| Days 4–7 | Recruit up to 30 consented pilot participants across two markets; establish device/task measurements | Confirm current tasks and affordability; projected tasks remain excluded |
| Days 8–14 | Evaluate seven-day retention, support time, data consumption and credited rewards; verify eligible withdrawal process; train partners | D7 ≥85%; participant net economics positive; credible channel pipeline |

A two-week pilot does not establish D30 retention. Begin scale conditionally, settle referral shares only against actual accepted rewards and review D30 before releasing the full expansion budget. If a first-week 250 target is required from a cold start, the supplied evidence is insufficient to promise it; pre-launch partner capacity is essential.

### Ten production weeks: net 250 productive licences added weekly

Assume 2% weekly churn of the opening productive base, 85% activation-to-D7 success and legitimate inventory recycling. Early D30 attrition is separately monitored; do not add it again to this recurrence. The 2% aggregate churn assumption is not inferred from the CSV.

| Production week | Opening productive | Expected churn | Required D7 productive additions | Approx. gross onboarding activations | Ending productive |
|---|---:|---:|---:|---:|---:|
| 1 | 0 | 0 | 250 | 295 | 250 |
| 2 | 250 | 5 | 255 | 300 | 500 |
| 3 | 500 | 10 | 260 | 306 | 750 |
| 4 | 750 | 15 | 265 | 312 | 1,000 |
| 5 | 1,000 | 20 | 270 | 318 | 1,250 |
| 6 | 1,250 | 25 | 275 | 324 | 1,500 |
| 7 | 1,500 | 30 | 280 | 330 | 1,750 |
| 8 | 1,750 | 35 | 285 | 336 | 2,000 |
| 9 | 2,000 | 40 | 290 | 342 | 2,250 |
| 10 | 2,250 | 45 | 295 | 348 | 2,500 |
| **Total** | | **225** | **2,725** | **3,211** | **2,500 occupied productive slots** |

This is a cohort planning approximation; onboarding precedes its D7 report by one week. Allow a trailing week to confirm the final cohort. Licence occupancy includes trials: near full capacity, stage admissions as slots are released. If leases cannot be promptly and legitimately recycled, hold applicants on a waiting list and extend the timetable. Never maintain 3,211 simultaneously active licences from a stock of 2,500.

A steady 300 D7/week engine provides limited buffer: at 2,250 opening licences it supports 255 net additions under 2% churn. If churn is 5%, net additions fall to about 188. Achieving net 250 then requires about 363 D7 additions, or roughly 1,210 opted-in prospects at 30% conversion. Retention is therefore part of the acquisition strategy.

Once 2,500 are active, stop promising 250 new placements/week. The goal becomes maintaining capacity; at 2% weekly churn, replacement demand is around 50 productive activations/week. Keep a consented waiting list, not an oversold licence queue.

## 11. Budget, staffing and UNO-funded credit cash flow

All amounts are USD planning assumptions, not vendor quotations. Separate **operating credits**, **recurring referral allocations**, and **incremental acquisition spending**. The 10% referral share is already removed before the UNO’s 40%; do not charge it twice in contribution calculations.

### Weekly acquisition ceiling, released only if margin supports it

| Item | Cash cap/week | Control |
|---|---:|---|
| Optional partner onboarding/service compensation | $150 | Discretionary ceiling, not automatic bounty on top of 10%; require positive retained-cohort economics |
| Paid-media experiment | $30 | One capped test; stop if retained CAC exceeds realised payback |
| Tools/communications | $20 | Existing low-cost tools preferred |
| Translation / content help | $40 | Batch adaptation; narrow scope if actual fair cost is greater |
| QR/print/local materials | $20 | Proven partners only |
| Contingency | $40 | Release for a measured blocker |
| **Cash acquisition ceiling** | **$300** | Not a recommendation to spend before profitability is demonstrated |
| Coordinator time, 25 h × $6 planning value | $150 | Replace with actual economic cost |
| **Fully loaded weekly ceiling** | **$450** | Ongoing device support and credit funding are additional |

If recurring referral shares suffice for partner acquisition and no service compensation is needed, reduce the cash ceiling to $150/week and the fully loaded amount to $300/week. Do not assume ambassadors accept this offer until they agree to it. Cash bounties have been removed as a default growth mechanism.

### Campaign envelope excluding production credits

| Component | Provision |
|---|---:|
| Setup and localisation | $250 |
| Pilot, including up to 30 first-month credits where required | $200 |
| Ten-week maximum acquisition cash | $3,000 |
| Ten-week coordinator labour value | $1,500 |
| Ramped recurring support allowance | $800 |
| **Non-production-credit envelope** | **$5,750** |

The earlier $5,750 figure is **not an all-inclusive budget** under UNO sponsorship. Production credit funding must be added. If the $150 weekly discretionary service line is unused, subtract $1,500 from this envelope. Referral payouts themselves are funded by their 10% share, not by a second acquisition-cost deduction.

### Production credit reserve

At $1.99 per monthly licence block, 2,500 funded licences cost **$4,975/month**. At $3.99 they cost **$9,975/month**. Use actual prices and bills; do not charge all owned but undistributed licences unless the platform actually requires that.

For the ten-week deployment schedule, end-of-week productive counts sum to 13,750. A rough productive-device exposure allowance is `13,750 × 7/30 × $1.99 = $6,384.58`. **This is not a cash invoice forecast**: activation timing, failed trials, full-month billing and non-transferable credits can materially increase expenditure.

A conservative reserve for the scheduled 3,211 onboarding activation attempts is:

- First monthly blocks for all 3,211 attempts, if each needs a fresh block: **$6,389.89**.
- Up to two further 2,500-licence renewal blocks: **$9,950.00**.
- **Production credit reserve: $16,339.89**.
- Add the $5,750 non-production-credit envelope: **$22,089.89 total funding envelope**, including valued labour.

This is a deliberately conservative reserve, not expected consumption and not claimed to be the platform’s billing mechanism. It assumes at most three billing blocks over the ten-week production window and timely cancellation of failed/inactive participation. The final reporting tail and delays can create additional renewals; fund them separately. Confirm whether credit follows a reusable licence, survives reassignment or must be repurchased for each new activation: reusable credits could substantially lower the reserve. Do not count pilot credits twice if pilot licences enter the production cohort.

### Fully deployed portfolio break-even

At $1.99 credits and $0.25 support, monthly UNO costs for 2,500 funded licences are **$5,600**, before recruitment, fixed overhead or tax. Covering that with a 40% share requires **$14,000/month in total distributable rewards** ($5.60 per funded licence).

| Pool per funded licence/month, sensitivity only | Portfolio pool | ULO 50% | Referral 10% | UNO 40% | UNO after $5,600 credits/support |
|---|---:|---:|---:|---:|---:|
| $5.00 | $12,500 | $6,250 | $1,250 | $5,000 | −$600 |
| $5.60 | $14,000 | $7,000 | $1,400 | $5,600 | $0 |
| $7.50 | $18,750 | $9,375 | $1,875 | $7,500 | $1,900 |
| $10.00 | $25,000 | $12,500 | $2,500 | $10,000 | $4,400 |

These pool levels are not projected from the CSV. The previous $12,000 revenue forecast and $1,385 UNO remainder are withdrawn. Until paid-licence-months are supplied, no credible 2,500-licence earnings forecast exists.

At 2% weekly churn near full deployment, about 217 productive replacements/month are required. Incremental acquisition expense for those replacements must fit the post-credit margin; the referral share continues to be accounted for separately. Pre-fund credits without relying on unearned task rewards or future Entropy/GPU contributions.

## 12. Campaign assets and ready-to-adapt messages

These are drafts for the approved programme. Replace placeholders with real pilot evidence and current authorised links before publishing. No message has been sent.

### Community post

“Already have a compatible Android phone and reliable, affordable internet? We are enrolling adults for optional Unetwork device tasks through an approved licence programme. Rewards are small and depend on eligible work; this is not a job or guaranteed income. There is no upfront licence-acquisition charge. We pay the activation credits. You receive 50% of the distributable rewards; the UNO receives 40% and the referral programme receives 10%. You do not pay us a deposit or subscription; your own connection, device and withdrawal costs may still apply. Check the full costs, device requirements and official onboarding steps here: [approved link]. Please do not buy a phone or new data plan just to join.”

### Partner introduction

“We are testing a small device-task programme for adults who already have suitable phones and internet. We would like permission to run an optional demonstration for interested members. We will disclose actual pilot earnings, recurring costs, verification and withdrawal requirements. Participants control their own accounts. Any partner compensation will be disclosed and tied to retained participation, with no participant recruitment fee. Could we share the approved information sheet for your review?”

### 30-second video structure

Show the official app and eligible device; explain only the tasks identified and verified in current records; show a dated, anonymised real reward example alongside credit/data costs; state that earnings vary; finish with eligibility and official onboarding links. Put “credits paid by UNO” and the small-reward warning on screen, not only in a caption.

### Referral invitation

“If this has been useful for you, you may share your referral link with another adult who already has compatible equipment and internet. Please explain the costs and small rewards honestly. You may receive 10% of the accepted distributable rewards for the directly attributed active lease, under the published referral terms. The operator’s share remains 50%, and the UNO pays credits. No purchase from you is required, and there are no downline payments.”

### Retention reminder

“Your device has not recorded eligible activity recently. If you want to continue, check connectivity, app status and task eligibility. If the costs or effort are no longer worthwhile, contact us about the official exit process. Please do not buy extra data merely to keep a licence active.”

## 13. Weekly control dashboard and corrective actions

Track country, source and cohort together. Minimum fields: opted-in prospects, eligibility pass rate, installation, first accepted reward, D7, D30, churn, active inventory, realised UP/device-day, credits payer, support minutes, data costs, acquisition spend, accrued referral-share liability, complaints and payout blockers. Do not treat raw allocations as active devices.

| Indicator | Initial target / trigger | Action |
|---|---|---|
| Opt-in prospects | 1,000/week capacity | Add proven partner capacity, not bulk unsolicited traffic |
| Prospect → first reward | ~35% | Find whether eligibility, installation or activation is failing |
| D7 retention | ≥85% of activated | Pause weak sources; fix economics/task availability |
| D30 retention | ≥75% of activated | Release remaining acquisition spend only after observed results |
| Weekly mature churn | ≤2% planning target | Diagnose cost, connectivity and task loss before buying replacements |
| Net productive additions | ≥250 until inventory full | Count net change, not gross code claims |
| Incremental D30 acquisition cost | At most three months of positive measured UNO contribution, reduced for churn | $2 is affordable only if the measured pool is about $7.27/month or higher under the stated costs |
| ULO net outcome | Positive after measured unavoidable costs | Do not scale loss-making country/device cohorts |
| UNO contribution | Positive after support and incentives | Reject volume that increases losses |
| Paid experiment | Maximum $30 first test | No automatic budget increase from click-through rate alone |
| Source quality | Repeated duplicate/fake outcomes | Suspend disputed referral payouts for review; do not punish legitimate shared-network users automatically |

Monday: reconcile inventory and reward data; Tuesday–Thursday: partner sessions and clinics; Friday: source/cohort review and bounded budget changes; weekend: optional self-service onboarding and a scheduled support window. One coordinator owns revenue and inventory; market helpers own language support; partners own introductions, not KYC or money handling.

At high performance—75% eligible × 75% install × 80% activate × 90% D7—1,000 opt-ins could produce 405 D7 outcomes. At downside—60% × 60% × 60% × 70%—the same volume produces only 151. The plan must react to measured conversion, not assume every channel hits the base case.

## 14. First actions and final decision rule

1. Confirm productive inventory and the meaning/scope of the CSV with the UNO.
2. Resolve platform permission for external recruitment, the `ups` recipient definition, UNO credit pricing/billing and three-way settlement.
3. Prepare the approved offer and transparent eligibility/economics page.
4. Recruit the two-market pilot and measure seven days of actual use; verify an eligible withdrawal path.
5. Sign enough community and ambassador capacity to plausibly produce 1,000 genuine opt-ins weekly.
6. Begin the ten-week production schedule only when participant and UNO economics pass; review D30 before broader commitment.
7. Expand geographically using measured results, keep projections out of promised earnings, and stop net growth when the inventory is full.

**The strongest cost-effective route is a permissioned, partner-led Android campaign for people already connected—not worldwide paid advertising or a GPU campaign.** The 250/week goal is operationally plausible only if the actual offer is worthwhile, the measured paid-licence economics work, partner capacity is established and retention is managed. If credited rewards do not cover UNO-funded credits and support after the 40% allocation, negotiate lower credit costs, improve measured productivity or reduce scope before scaling. Do not transfer that shortfall back to ULOs contrary to the offer.

## Sources and assumptions

- **S1 — User inputs:** `statistics-incentives-All.csv`, supplied task list and stated inventory of 2,500 licences. CSV figures calculated directly; owner confirms historical 50:50 and distributed-cohort scope. `ups` is modelled as UNO credit subject to recipient verification. Prior estimated per-task rates are removed; only measured task-category credits are used. No live account was inspected.
- **S2 — [Unetwork terms](https://unitynodes.io/terms):** participation/credit rules, geographic restrictions, mobile-device wording and §5.6 promotion/leasing constraints, reviewed 28 September 2026. The campaign permission gate comes from these published terms, not from a restriction on preparing this plan.
- **S3 — [Accounts, KYC and attestation](https://unitynodes.io/learn/accounts-kyc-attestation):** device/account eligibility, reviewed 28 September 2026.
- **S4 — [Business coverage](https://unitynodes.io/business):** observed market evidence from this conversation’s source review; not a country approval list.
- **S5 — [World Bank country/lending groups](https://datahelpdesk.worldbank.org/knowledgebase/articles/906519-world-bank-country-and-lending-groups):** income-group context; not evidence of task availability or individual affordability.
- **S6 — [WhatsApp Business policy](https://business.whatsapp.com/policy):** consent and messaging rules, reviewed 28 September 2026.
- **S7 — [Reddit spam policy](https://support.reddithelp.com/hc/en-us/articles/360043504051-Spam):** community distribution constraints, reviewed 28 September 2026.

All suggested conversion rates, weekly output quotas, churn rates, staffing costs and budgets are explicit planning assumptions. They are neither vendor quotations nor performance established by the CSV. This deliverable is a marketing and operating plan; no advertising was purchased, partner contacted, licence assigned or account changed.

**Revision audit:** replaced 80/20 with 50/40/10; moved all activation credits to UNO; replaced estimated daily tasks with recorded CSV categories; reconstructed historical allocations conditionally; removed unsupported device-yield, withdrawal-time and portfolio forecasts; added paid-licence break-even, recurring referral rules and a credit-funded campaign reserve.
