# Unetwork: a cost-controlled plan to activate 2,500 ULO licences

**Prepared:** 28 September 2026  
**Revision 4:** adds a task-driven weekly revenue, expense, profit and cash-flow forecast; editable calculator; 52-week continuation and payback; explicit scenario assumptions. Retains all Revision 3 review improvements and ULO 50% / UNO 40% / referral 10%.  
**Objective:** build a repeatable acquisition operation capable of at least 250 net additional productive licences per week, until 2,500 are productively deployed.  
**Basis:** the supplied incentive CSV, the owner’s clarifications and public participation terms. Section 3 uses CSV evidence only. Section 16 introduces explicitly illustrative daily reward assumptions for scenario forecasting, not measured rates or promises. Budgets, conversion targets and market allocations remain planning assumptions.

**Revenue forecast:** Section 16 provides week-by-week results after configured expenses and a pluggable algorithm. Open the companion `UNETWORK_REVENUE_CALCULATOR.html` in a browser to change tasks and regenerate up to 260 weeks.

## 1. Recommended strategy and conditions for success

**Decision first:** do not commit to the 250-per-week production schedule until measured rewards cover UNO-funded credits, support and acquisition with room for overhead; participants choose to continue after full disclosure; and the next cohort has credible task and recruitment capacity. If those conditions fail, improve the economics or reduce scope before adding funded licences.

The proposed route to test first uses **trusted local community partners, adult digital-skills groups, small creators and one-level referrals**, supported by short onboarding clinics. Recruit people who already own suitable phones and already have affordable, stable internet. Begin in two markets; expand into six only after local operation and withdrawals are demonstrated. Retain international reach through an eligibility-screened waiting list.

Position the opportunity as **small supplementary rewards for optional device tasks**, with clearly disclosed costs. It is not employment, a salary, guaranteed daily income or a reason to buy a phone, broadband connection or GPU.

Design for **approximately 300 day-seven productive activations per week**, providing room for churn while pursuing **250 net additions**. A nominal distribution count is inadequate: codes handed out, registrations and inactive devices are not successful deployment.

The revised offer is **50% ULO / 40% UNO / 10% referral of the distributable licence reward pool**, with **the UNO paying all activation credits**. The ULO pays no licence-acquisition fee or platform credit charge under this offer. Internet, electricity, device wear, withdrawal costs and time may still matter. The referral percentage comes from the UNO’s previous 50% allocation, preserving the ULO’s historical 50% share.

The historical split is confirmed by the owner as 50:50, and the export covers only licences distributed within its timeframe. The calculations below **interpret `ups` as the UNO’s credited share**, because this is presented as a UNO incentive export. Confirm that column’s recipient definition against the ledger before treating reconstructed totals as verified pool amounts. If `ups` is already the combined pool, do not double it. There is no basis for treating the export as 2,500 fully active licences.

### Launch and expansion gates

The UNO owns the decision log; the coordinator gathers evidence and reports failures. A gate is passed with dated evidence, not a checklist assertion. All pilot participation also requires an authorised recruitment route.

| Gate | Required evidence | If it fails |
|---|---|---|
| Permission and settlement | Applicable lease terms; platform-approved recruitment route and copy; supported 50/40/10 accounting and UNO credit funding | Use only authorised discovery; resolve settlement before promising referral payouts |
| Product and eligibility | Official Android build installs on intended devices; applicable attestation/KYC works; an accepted task reward appears; eligible withdrawal route checked | Pause the affected device/market; do not infer support from a download link |
| Data and billing | Export recipient and task labels defined; funded licence-months reconciled; actual credit invoice, exemptions, renewal and reassignment rules recorded | No per-device earnings claims or large credit commitment |
| Task availability at increasing scale | Compare incumbent and new-cohort accepted activity, reward per eligible device-day and zero-reward days; obtain platform guidance on task/country quotas and onboarding capacity | Hold the next cohort; diagnose workload, eligibility or connectivity rather than assume marketing can fix it |
| UNO commercial margin | Realised contribution after credits/support is positive and expected retained contribution covers acquisition plus allocated overhead | Reduce acquisition cost, improve productivity or negotiate credit price; do not pass costs back to ULOs |
| Participant value | Positive measured net outcomes plus informed uptake and voluntary continued participation in each tested market | Change the offer or stop that market; positive cash alone is insufficient |
| Promoter and funnel capacity | Partners accept disclosed compensation; two completed weekly cohorts demonstrate sufficient unique opt-ins, conversion and retention | Replace unproven channel quotas or extend preparation |
| Funding and inventory | Credits, earned referral liabilities and support are funded; active trials plus productive licences stay within 2,500 | Limit admissions to funded, legally reusable slots |

The CSV does not establish total network capacity. This is a conditional operating plan, not a guarantee of 250 weekly activations. The production clock starts only after validation; a two-week diagnostic pilot is not a deadline for declaring success.

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

### Scope and capacity: do not divide a partial cohort by the full inventory

Under the conditional `P = 2H` reconstruction, the latest completed 28-day window normalises to about **69.397369 UP of pool per 30 days**. Dividing that by all 2,500 owned licences gives 0.027759 UP, but that is not an observed per-licence yield: the export does not show 2,500 funded licences contributing for the entire period. Comparing it with the $14,000 portfolio break-even is a **fixed-total-income stress case**, not proof of a 202-fold network shortfall. If the cohort's total reward really stayed fixed while costs grew to 2,500 funded licences, scaling would fail; that fixed-budget premise must be tested, not assumed.

Similarly, allocation counts do not establish a 23-device network ceiling. Establish the accounting unit and the export population first. Capacity for scarce calls can differ from capacity for continuous telemetry: request task-, country- and eligibility-specific demand information rather than one universal device-day quota.

The `Other` category records **2.5 UP per allocation in every row**. For example, 25 UP represents ten allocations. It does not prove a 25× premium tier, nor recurring premium work. Similar average rewards across other task labels do not prove equal units of work. Retain the categories without inventing mappings or multipliers.

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

### Credit-cost sensitivity and full commercial margin

Licence ownership and recurring operating credits are separate questions. Published terms describe credits starting at $1.99 monthly and the FAQ permits the licence holder to fund the operator's plan. Confirm the actual charge with the platform; a zero-cost scenario requires an exemption, included credits or another documented basis. Prepaid credits may remove current cash expenditure but still have an economic cost or expiry. [S2, S8]

| Credit cost per funded licence-month | Support assumption | Operating break-even pool | Pool to recover $2 acquisition over three retained months, before overhead/churn |
|---|---:|---:|---:|
| $0.00 | $0.25 | $0.625 | $2.292 |
| $1.99 | $0.25 | $5.60 | $7.267 |
| $3.99 | $0.25 | $10.60 | $12.267 |

The $5.60 figure is a threshold, **not a predicted or base-case yield**. The 50/40/10 split is not inherently insolvent, but it has less UNO margin than a 50% UNO share on the same pool. Credit cost does not change the allocation identity `new UNO reward = 0.8 × historical UNO reward`; it changes net contribution afterwards.

For planning, let `A` be acquisition expense per retained operator, `T` expected paid retained months and `F` allocated fixed overhead per funded licence-month. Required pool before tax and capital recovery is `(C + S + A/T + F) / 0.40`. In practice, use survival-weighted monthly contribution: `CAC ceiling = sum(probability active in month t × contribution in month t)`, less overhead and a cash buffer. Do not count the same partner fee or support time in both acquisition and recurring costs.

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

### Participant value test for each pilot market

National GNI is background context, not a participation threshold or median household income. Do not assert that rewards below 5% of national income are worthless. Test whether the specific offer is worthwhile for people who already have suitable equipment and connectivity.

| Measure | Collection method | Decision use |
|---|---|---|
| Informed uptake | Explain measured rewards, 50/40/10, permissions, KYC, withdrawal and UNO-funded credits before acceptance; record invited/eligible/accepted counts | Low uptake after disclosure means the value proposition needs revision |
| Participant net cash | Actual ULO rewards less incremental data, electricity and withdrawal costs; include zero-reward users | Pause cohorts with negative median net cash; investigate lower-quartile losses |
| Time and inconvenience | Setup minutes, help requests, battery/connection issues and voluntary time valuation | Report separately from cash; do not declare time free |
| Voluntary continuation | Ask whether the participant wants to continue at the observed reward, without recruitment bonus; check D7 and D30 behaviour | Initial management target: at least 70% of respondents willing to continue, plus D7/D30 targets in Section 13; report non-response separately |
| Exit reasons | Optional short interview: earnings, trust, permissions, data, battery, verification, payment or technical problems | Fix the actual cause rather than increasing recruitment pressure |

These are pilot decision targets, not market benchmarks. Start with up to 30 total participants across two markets; small samples are directional, not population estimates. Review results by market and device; expand the sample when it is too small to distinguish a repeatable outcome. Do not use a pooled positive average to hide a loss-making market. No purchase of hardware or new data service is required to qualify for the test.

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
| Small local tech creators | Conditional compensation test | Demonstrate real installation and verified pilot results. Test the recurring referral offer first; separately price any production or onboarding service and include it in CAC. Disclose compensation. |
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

### Telcos, NGOs and other potentially subsidised channels

Keep these as additional partnership experiments, not guaranteed sources in the weekly quota.

| Channel | Commercial or mission hypothesis | Evidence needed before counting weekly output |
|---|---|---|
| ISP/telco customer placement | An optional service may support customer engagement or an approved network-quality programme | Named counterpart, agreed permitted audience, privacy boundaries, delivery dates and measured productive activations |
| Sponsored data / zero-rating | Sponsor funds participating traffic because documented outcomes justify the expense | Written eligible-traffic and cost terms, technical feasibility, applicable policy review, setup/support costs and task eligibility; zero-rated data does not guarantee connectivity or power |
| NGO/digital-literacy programme | Optional participation may complement digital-skills training if independently measured participant benefits outweigh burden | Explicit programme approval, informed consent process, staff budget, benefit measurement and a funded agreement; no assumption a grant will be awarded |
| Crypto/airdrop communities | Members may be familiar with wallet-based rewards | Test honest small-reward messaging, unique productive operators and D30 retention; no speculative token promises or bulk-account incentives |

Revenue share is an economic cost, and organic acquisition consumes labour. A federation configuration or directory listing is not a signed partner. If any strategic partner needs another share, identify its funding source without silently increasing the 10% referral allocation or reducing the ULO's 50%.

## 8. Partner recruitment playbook

### Build the supply of promoters before promising the weekly volume

During preparation, identify 100 relevant organisations/admins across the initial two markets. Record public contact route, audience fit, language, existing connectivity, permission requirements and estimated support burden. This is a research list, not a purchased consumer contact list.

Planning funnel: 100 individually assessed partner approaches → 40 conversations → 24 demonstration sessions → 18 launch partners → at least 12 consistently productive partners. These ratios are targets to validate. If only six become productive, either double their demonstrated capacity or extend preparation; do not assume the missing output.

A coordinator handles roughly 20 tailored approaches per business day for one week, followed by demonstrations and written agreements. Contact organisational/public business channels appropriately; consumer recruitment begins only with permission. No messages have been sent as part of preparing this plan.

Give each partner a one-page brief: eligibility, realistic net examples, exact cost payer, official links, prohibited claims, support route, attribution and compensation. Include a “who should not join” box: costly data, negative measured net outcomes, unsupported phone/country, unwillingness to complete required verification, or expectation of salary-scale income.

### Compensation model

**Separate occasional referrers from active service providers.** A participant making an occasional introduction receives the disclosed 10% recurring share. An ambassador expected to prospect, host sessions or resolve installations must first agree to a realistic scope and compensation; do not assume recurring shares fund a job.

Run a two-week promoter test with a small set of willing partners. Record unique opt-ins, D7/D30 operators, hours spent, actual referral accrual, paid service fees and willingness to continue. At the illustrative $5.60 pool, fifty retained licences yield $28 monthly referral income; this is not a salary claim. At ten hours of work it is $2.80 per hour for that month's accrual, excluding future income and other costs—not an agreed fair rate. Use actual local terms and include continuing support effort.

Count a promoter's weekly quota only after demonstrated output and acceptance of the actual agreement. The $150/week service ceiling is a budget constraint, not evidence it buys ten ambassadors. If it cannot buy sufficient agreed capacity, lower the quota, replace the channel or extend the schedule. Do not require unpaid labour to preserve the model.

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

A two-week pilot does not establish D30 retention. Continue with bounded validation cohorts, settle referral shares only against actual accepted rewards and observe at least one D30 cohort before committing the full production budget. If a first-week 250 target is required from a cold start, the supplied evidence is insufficient to promise it; pre-launch partner capacity is essential.

### Capacity-validation ladder before the production clock

Use up to 30 pilot participants, then bounded cohorts toward 100 and 250 total productive licences, only while Section 1 gates pass. These are test ceilings, not mandatory additions or proof of network limits. Reconcile pre-production participants into opening inventory; do not recruit or bill them twice. Existing verified cohorts can provide equivalent evidence.

For each addition, compare at least seven complete days of incumbent and new-cohort observations: funded licence-days, eligible device-days, accepted task activity, zero-reward proportion, realised pool, support and participant net outcomes. Normalise task/country/device mix. A greater than 20% fall in incumbent reward per comparable eligible device-day, or a greater than 10 percentage-point rise in zero-reward days, triggers a hold and investigation; these are management alerts, not statistically proven saturation thresholds. Hold immediately if either party's economics fail. Short-term seasonality, connectivity and eligibility must be separated from demand saturation.

Ask the platform whether rewards come from a fixed pool or qualifying incremental work, whether task/country quotas apply, and what onboarding capacity is supported. Do not claim a maximum network capacity without this evidence. Confirm Android installation and credited activity using the official published build; APK availability alone is not an execution test. [S9]

After validation, require two completed weekly recruitment cohorts to demonstrate enough funnel capacity for the chosen schedule, at measured CAC. A successful 30-user test cannot prove 250-per-week acquisition. Validation can exceed two weeks; never compress it to preserve an advertised date.

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

### Downside schedule and automatic response

The illustrative downside conversion is `0.60 × 0.60 × 0.60 × 0.70 = 15.12%` from opt-in to D7 productive. Required unique opt-ins are rounded up:

| Situation | D7 additions required | Opt-ins at 15.12% |
|---|---:|---:|
| Initial 250 net additions, no opening churn | 250 | 1,654/week |
| Final scheduled week: 250 net plus 45 replacements | 295 | 1,952/week |
| Maintain 2,500 at 2% weekly churn | 50 | 331/week |

At 1,000 opt-ins weekly and 15.12% conversion, expected D7 additions are 151.2. Under the simplified recurrence `N_next = 0.98 × N + 151.2`, starting at zero and with unrestricted legitimate slot recycling, reaching 2,500 takes approximately **20 production weeks**, plus preparation and reporting time. This is a downside planning illustration, not a promised alternative timetable. At 5% churn the same calculation takes about 35 weeks; rounding, eligibility and slot-release delays can extend both.

After two completed weekly cohorts below the conversion needed for the current target, choose one documented response: expand already-proven sources within the CAC ceiling; fix the failing funnel stage and retest; or reduce admissions and extend the schedule. Do not silently increase ad spend. Rebudget credits, support and coordination for every extension; the ten-week reserve below no longer applies unchanged. Recompute using current productive inventory, not a repeated zero-start assumption.

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

### Credit-reserve sensitivity

Under the same conservative 8,211 monthly-block allowance (3,211 initial plus 5,000 renewal blocks), with the other campaign provisions held fixed:

| Actual credit price per block | Production credit reserve | Campaign funding envelope including $5,750 other provisions |
|---|---:|---:|
| $0.00 | $0.00 | $5,750.00 |
| $1.99 | $16,339.89 | $22,089.89 |
| $3.99 | $32,761.89 | $38,511.89 |

The zero-credit case is conditional on documented coverage or exemption; it is not inferred from ownership. These are funding allowances, not expenditure forecasts. The pilot's $200 provision remains unchanged in this comparison; replace it with actual cost when known. Validation beyond the diagnostic pilot, delayed deployment and additional billing blocks require an updated cash calendar before commitment. At each weekly review show opening cash, credit purchases/renewals, service costs, earned referral liabilities, settled receipts and closing cash; unsettled rewards are not available cash.

### Fully deployed portfolio break-even

At $1.99 credits and $0.25 support, monthly UNO costs for 2,500 funded licences are **$5,600**, before recruitment, fixed overhead or tax. Covering that with a 40% share requires **$14,000/month in total distributable rewards** ($5.60 per funded licence).

| Pool per funded licence/month, sensitivity only | Portfolio pool | ULO 50% | Referral 10% | UNO 40% | UNO after $5,600 credits/support |
|---|---:|---:|---:|---:|---:|
| $5.00 | $12,500 | $6,250 | $1,250 | $5,000 | −$600 |
| $5.60 | $14,000 | $7,000 | $1,400 | $5,600 | $0 |
| $7.50 | $18,750 | $9,375 | $1,875 | $7,500 | $1,900 |
| $10.00 | $25,000 | $12,500 | $2,500 | $10,000 | $4,400 |

These pool levels are not projected from the CSV. The previous $12,000 revenue forecast and $1,385 UNO remainder are withdrawn. Until paid-licence-months are supplied, no empirically calibrated 2,500-licence earnings forecast exists. Section 16 supplies conditional scenarios with explicit reward assumptions instead.

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
| UNO contribution | Positive after UNO-funded credits and support; sufficient retained margin for CAC and overhead | Reject volume that increases losses; reconcile referral share without deducting it twice |
| Task capacity | Compare incumbent/new cohort yields and zero-reward days against Section 10 alerts | Hold expansion and diagnose deterioration |
| Participant willingness | Informed uptake and voluntary continuation measured separately from cash outcome | Stop weak-value cohorts even if UNO economics pass |
| Promoter viability | Agreed compensation plus demonstrated retained output and hours | Remove unproven ambassador capacity from the weekly quota |
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
6. Complete bounded capacity validation, observe D30 and demonstrate the recruitment funnel; start the production clock only when all launch gates pass. Reconcile opening inventory and adjust the ten-week illustration accordingly.
7. Expand geographically using measured results, keep projections out of promised earnings, and stop net growth when the inventory is full.

**The proposed route to test first is a permissioned, partner-led Android campaign for people already connected. Its relative cost-effectiveness remains to be demonstrated.** The 250/week goal is operationally plausible only if the actual offer is worthwhile, the measured paid-licence economics work, partner capacity is established and retention is managed. If credited rewards do not cover UNO-funded credits and support after the 40% allocation, negotiate lower credit costs, improve measured productivity or reduce scope before scaling. Do not transfer that shortfall back to ULOs contrary to the offer.

## 15. Peer-review disposition and evidence boundaries

| Review contribution | Decision in this revision |
|---|---|
| Arithmetic verification of splits, reserve and deployment recurrence | Retained; clearly separated thresholds, funding allowances and forecasts |
| Put economics and capacity before acquisition | Accepted; explicit launch gates, bounded cohorts and hold triggers now precede production |
| Add zero-credit sensitivity | Accepted alongside $1.99/$3.99; actual billing still controls |
| Test participant value and ambassador compensation | Accepted; willingness, costs, hours and retained outcomes are separate tests |
| Explicit downside response | Accepted; 1,654–1,952 weekly opt-ins at 15.12% or an extended schedule with revised funding |
| Explore telcos, sponsored data and NGO partners | Retained as hypotheses requiring actual agreements and funded delivery |
| Network-wide 202× shortfall or 23-device ceiling | Rejected: cohort export is not network capacity; retained only a clearly conditional fixed-income stress interpretation |
| Premium 25× tier | Rejected: 25 UP / 10 allocations = 2.5 UP per allocation |
| GNI-based universal participation threshold | Rejected: measure individual costs and willingness locally |
| EMBER source code as Unetwork evidence | Excluded: no demonstrated production relationship; ECR, emctl and federation configuration cannot establish Unetwork features or partners |
| 50/40/10 mathematically impossible | Rejected: viability depends on realised pool, costs, retention and CAC; allocation identities do not depend on credit price |
| CSV unrecoverable | Corrected: supplied CSV remains available; preserve the source export and capture future export metadata |

Keep a dated evidence register for source URL/build, export period and hash, billing evidence, pilot cohort, permission scope and decision owner. Public documentation is evidence of published rules; it is not proof of live payment, production security or all-country availability. This revision incorporates useful review findings without importing unsupported project, market or legal conclusions.

## 16. Revenue growth forecast and pluggable task model

### 16.1 What this forecast means

The forecast now connects weekly productive licence growth to **reward revenue, the 50/40/10 distribution, all configured operating expenses, net profit, settlement timing and cumulative funding**. It runs from production week 1; preparation and validation happen before that clock. The companion calculator defaults to 52 weeks and supports up to 260.

**Reference expectation, conditional on the inputs below:** reward revenue grows as devices join, but credit purchases, failed activations and acquisition spending make weeks 1–10 loss-making. The reference scenario first produces a positive weekly result in week 11, recovers its cumulative managerial loss in week 37 and recovers cumulative cash spending in week 50. This is a planning expectation under chosen inputs, not an earnings estimate derived from the CSV. The scenario should be recalibrated with observed per-device task rates before funds are committed.

No probability-weighted expected value is claimed. “Downside”, “reference” and “upside” are sensitivity labels only. The forecast explicitly assumes that qualifying incremental device activity can earn the entered rates. If a task has a fixed reward budget, enter its daily pool cap; do not assume unlimited demand.

### 16.2 Reference inputs and expense coverage

| Input | Reference setting | Treatment |
|---|---|---|
| Opening productive licences | 0 | Illustrative production start; replace with verified opening inventory |
| Net growth | 250/week, capped at 2,500 | Replace 2% weekly churn in addition to net growth |
| Activation success | 85% | Round gross attempts up; unsuccessful attempts earn nothing |
| Deployment timing | Successes spread evenly across each week | Half-day earnings on arrival; incumbents earn full days |
| Device mix | 100% Android | Mutually exclusive Android/iOS/Windows fractions must sum to 1 |
| Combined distributable reward | 0.25 UP per productive device-day | Synthetic reference pool = 7.50 UP per 30 full days; not a rate attributed to a named task |
| UP conversion | $1 realised per UP | Editable cash-realisation assumption, not proof of withdrawal |
| Shares | UNO 40%; ULO 50%; referral 10% | No additional upstream deduction; shares sum to 100% |
| Credits | $1.99 per 30-day block | Each gross attempt buys a new block; surviving cohorts renew every 30 days |
| Support | $0.25 per 30 device-days | Successful device exposure plus half a day per failed attempt |
| Growth acquisition | $300/week | Existing marketing cash ceiling; referrals are already allocated from the pool |
| Growth coordination | $150/week | Treated as actual cash pay here; earlier budget valued labour without establishing payment |
| After capacity is reached | $60 acquisition + $60 coordination weekly | New maintenance planning inputs, not measured staffing costs; replacements continue |
| Setup and diagnostic pilot | $450 before week 1 | $250 setup + $200 pilot; included in cumulative profit and funding, not charged again weekly |
| UNO settlement lag | 2 weeks | Illustrative fixed lag; no receipt before maturity; no pre-existing receivables |
| Opening funding | $25,000 | Demonstration cash balance, not required funding or an additional expense |
| Additional overhead, task costs, fees, tax | $0 by default | Unknown amounts must be entered; zero is not a claim of exemption |
| Historic licence cost allocation | $0 by default | Existing ownership treated as sunk cash; optional noncash charge over editable weeks |

**“Net profit” here means after every configured expense, not after costs that have never been supplied.** The calculator includes fields for additional overhead (including financing/insurance/administration if applicable), per-task costs, UNO reward fees, a tax provision and optional original licence-cost allocation. Enter actual values before treating results as after-tax or full-investment profitability. ULO internet/electricity costs belong in participant economics unless the UNO subsidises them; subsidies must be entered as extra costs.

The pre-production $450 covers only the stated diagnostic pilot/setup provision. Any further validation cohorts, travel or extended preparation must be added to setup or opening inventory/cash as appropriate; do not silently assume all pre-launch validation is funded by $450.

**Credit accounting convention:** the model expenses each complete non-refundable credit block when purchased, including failed trials, with no residual prepaid asset. This is conservative managerial profit with lumpy renewal costs, not a formal accrual-accounting income statement. It avoids prorating away charges paid for unsuccessful devices. Cash differs from managerial profit mainly because reward receipts lag earnings and any optional historic capital allocation is noncash.

Churn is applied proportionally to all incumbent cohorts at the start of each week. Failed trials consume a half-day of support and are assumed released before same-day successful placements; verify that the platform permits the necessary inventory turnover. Productive cohorts are fractional expected counts, while weekly gross activation attempts are rounded up. If trials last longer, reserve capacity and extend admissions; the nominal ten-week schedule is then not an executable promise.

### 16.3 Weeks 1–10: reward revenue and distribution

Each week's revenue reflects actual modelled device-days, not the closing licence count multiplied by seven. In week 1, 250 ending productive licences produce 875 device-days, equivalent to 125 operating for seven days. ULO and referral revenue are allocations from the pool, not additional costs deducted from the UNO's 40%.

| Week | Ending productive | Productive device-days | Total pool | ULO 50% | Referral 10% | UNO 40% |
|---|---|---|---|---|---|---|
| 1 | 250 | 875.0 | $218.75 | $109.38 | $21.88 | $87.50 |
| 2 | 500 | 2,607.5 | $651.88 | $325.94 | $65.19 | $260.75 |
| 3 | 750 | 4,340.0 | $1,085.00 | $542.50 | $108.50 | $434.00 |
| 4 | 1000 | 6,072.5 | $1,518.12 | $759.06 | $151.81 | $607.25 |
| 5 | 1250 | 7,805.0 | $1,951.25 | $975.63 | $195.13 | $780.50 |
| 6 | 1500 | 9,537.5 | $2,384.37 | $1,192.19 | $238.44 | $953.75 |
| 7 | 1750 | 11,270.0 | $2,817.50 | $1,408.75 | $281.75 | $1,127.00 |
| 8 | 2000 | 13,002.5 | $3,250.63 | $1,625.31 | $325.06 | $1,300.25 |
| 9 | 2250 | 14,735.0 | $3,683.75 | $1,841.88 | $368.38 | $1,473.50 |
| 10 | 2500 | 16,467.5 | $4,116.87 | $2,058.44 | $411.69 | $1,646.75 |

### 16.4 Weeks 1–10: expenses and net result

The “marketing + coordinator” column is $300 + $150 weekly. Other overhead, task costs, fees, tax and capital allocation are zero in this reference run; the calculator shows their separate columns when supplied. Cumulative profit includes the $450 pre-production expense.

| Week | UNO earned | Credit purchases/renewals | Support | Marketing + coordinator | All weekly expenses | Net weekly profit | Cumulative profit |
|---|---|---|---|---|---|---|---|
| 1 | $87.50 | $587.05 | $7.48 | $450.00 | $1,044.53 | −$957.03 | −$1,407.03 |
| 2 | $260.75 | $597.00 | $21.92 | $450.00 | $1,068.92 | −$808.17 | −$2,215.20 |
| 3 | $434.00 | $608.94 | $36.36 | $450.00 | $1,095.30 | −$661.30 | −$2,876.49 |
| 4 | $607.25 | $620.88 | $50.80 | $450.00 | $1,121.68 | −$514.43 | −$3,390.92 |
| 5 | $780.50 | $960.59 | $65.24 | $450.00 | $1,475.83 | −$695.33 | −$4,086.26 |
| 6 | $953.75 | $1,107.57 | $79.68 | $450.00 | $1,637.25 | −$683.50 | −$4,769.76 |
| 7 | $1,127.00 | $1,128.64 | $94.13 | $450.00 | $1,672.76 | −$545.76 | −$5,315.52 |
| 8 | $1,300.25 | $1,149.70 | $108.57 | $450.00 | $1,708.27 | −$408.02 | −$5,723.54 |
| 9 | $1,473.50 | $1,352.16 | $123.01 | $450.00 | $1,925.17 | −$451.67 | −$6,175.21 |
| 10 | $1,646.75 | $1,613.88 | $137.45 | $450.00 | $2,201.33 | −$554.58 | −$6,729.79 |

Ten-week totals: total reward pool **$21,678.13**; UNO earned **$8,671.25**; credit expenditure **$9,726.41**; net result including setup **−$6,729.79**.

This result is not improved by counting the 10% referral allocation twice or ignoring failed activation charges. Credit expenditure here is generated from daily cohorts; the conservative $16,339.89 reserve in Section 11 is a funding allowance, not another expense to add to this forecast. Likewise, do not add the whole $5,750 campaign envelope on top: its marketing, coordination and support components are already modelled, and setup/pilot is already $450.

### 16.5 Weeks 1–10: cash flow and funding

Only the UNO's 40% entitlement enters this treasury model. ULO/referral shares are assumed settled directly by the platform. If the UNO actually handles their payouts, add a matched pass-through cash ledger and liabilities; their gross receipts are not UNO revenue. Net UNO receipts arrive two weeks after accrual in this example.

| Week | UNO cash received | Cash expenses | Net cash flow | Cumulative cash flow incl. setup | Closing cash from $25,000 |
|---|---|---|---|---|---|
| 1 | $0.00 | $1,044.53 | −$1,044.53 | −$1,494.53 | $23,505.47 |
| 2 | $0.00 | $1,068.92 | −$1,068.92 | −$2,563.45 | $22,436.55 |
| 3 | $87.50 | $1,095.30 | −$1,007.80 | −$3,571.24 | $21,428.76 |
| 4 | $260.75 | $1,121.68 | −$860.93 | −$4,432.17 | $20,567.83 |
| 5 | $434.00 | $1,475.83 | −$1,041.83 | −$5,474.01 | $19,525.99 |
| 6 | $607.25 | $1,637.25 | −$1,030.00 | −$6,504.01 | $18,495.99 |
| 7 | $780.50 | $1,672.76 | −$892.26 | −$7,396.27 | $17,603.73 |
| 8 | $953.75 | $1,708.27 | −$754.52 | −$8,150.79 | $16,849.21 |
| 9 | $1,127.00 | $1,925.17 | −$798.17 | −$8,948.96 | $16,051.04 |
| 10 | $1,300.25 | $2,201.33 | −$901.08 | −$9,850.04 | $15,149.96 |

The reference model's maximum cumulative cash deficit is **$9,850.04**, reached by week 10; this is the modelled minimum external funding before any contingency. It is not the same as the more conservative credit reserve or a recommended cash buffer. Unsettled rewards are receivables, not spendable funds. Longer settlement, verification holds or lower realised conversion can increase funding needs substantially.

### 16.6 Continued growth in retained earnings after deployment

From week 11, the portfolio stops net growth and replaces approximately 50 productive licences weekly. At 85% activation success that requires 59 gross attempts each week. The reference model reduces acquisition/coordination to the explicitly assumed maintenance amounts; if those reductions are infeasible, enter higher values and payback moves later. Productive exposure is about 17,325 device-days weekly because churned devices take time to replace.

| Week | Ending productive | UNO earned | Total weekly expenses | Net weekly profit | Cumulative profit | Cumulative cash flow |
|---|---|---|---|---|---|---|
| 11 | 2500 | $1,732.50 | $1,320.67 | $411.83 | −$6,317.96 | −$9,697.21 |
| 13 | 2500 | $1,732.50 | $1,411.43 | $321.07 | −$5,602.56 | −$9,067.56 |
| 16 | 2500 | $1,732.50 | $1,345.50 | $387.00 | −$4,963.90 | −$8,428.90 |
| 20 | 2500 | $1,732.50 | $1,352.59 | $379.91 | −$3,998.24 | −$7,463.24 |
| 26 | 2500 | $1,732.50 | $1,472.25 | $260.25 | −$2,410.65 | −$5,875.65 |
| 37 | 2500 | $1,732.50 | $1,388.98 | $343.52 | $246.46 | −$3,218.54 |
| 50 | 2500 | $1,732.50 | $1,411.41 | $321.09 | $3,513.94 | $48.94 |
| 52 | 2500 | $1,732.50 | $1,528.92 | $203.58 | $4,030.57 | $565.57 |

Weekly profit is not perfectly smooth: monthly-block renewals follow cohort activation anniversaries. Distinguish **first positive week**, **cumulative profit payback**, and **cumulative cash payback**. One profitable week does not prove sustained profitability.

### 16.7 Downside, reference and upside with identical costs

Only the illustrative daily reward pool changes across these scenarios; device growth, support, credits, marketing and settlement assumptions remain identical. These are not the user-supplied $0.10 task estimates and are not fitted to the aggregate CSV.

| Measure | Downside | Reference | Upside |
|---|---|---|---|
| Daily pool per productive device | 0.166667 UP | 0.250000 UP | 0.333333 UP |
| 30-day equivalent pool | $5.00 | $7.50 | $10.00 |
| Week 10 UNO earned | $1,097.83 | $1,646.75 | $2,195.67 |
| Week 10 net profit | −$1,103.49 | −$554.58 | −$5.66 |
| Cumulative profit through week 10 | −$9,620.20 | −$6,729.79 | −$3,839.37 |
| Cumulative profit through week 26 | −$14,541.06 | −$2,410.65 | $9,719.77 |
| Cumulative profit through week 52 | −$23,114.84 | $4,030.57 | $31,175.99 |
| First positive week | None within 52 weeks | 11 | 8 |
| Cumulative profit payback | Not within 52 weeks | 37 | 15 |
| Cumulative cash payback | Not within 52 weeks | 50 | 20 |
| Peak funding deficit through week 52 | $25,424.84 | $9,850.04 | $7,999.70 |

The downside case continues losing money and eventually exceeds the illustrative $25,000 funding balance; it is not a viable rollout simply because licences are deployed. The upside case first turns positive in week 8 but can turn negative again at a renewal-heavy week. The reference case's long recovery period makes measured reward yield and credit cost central to the decision.

### 16.8 Pluggable task schema

Open `UNETWORK_REVENUE_CALCULATOR.html` locally in a modern browser. It contains the complete algorithm, editable controls, task table, revenue/profit chart, all weekly results, input JSON save/load and CSV export. No installation, external service or network connection is required. Edited inputs are not automatically saved; use **Save inputs JSON** before closing.

| Task input | Meaning and unit | Example for an additional task, illustrative only |
|---|---|---|
| Name | Unique human-readable task label | New telemetry service |
| Enabled | Include only when intentionally modelled | true |
| Supported devices | Android/iOS/Windows boolean fields | Android=true, others=false |
| Daily reward | UP per eligible device-day on the selected basis | 0.05 |
| Rate basis | `pool`, `historical_uno` (50%), or current `ulo` | pool |
| Eligible fraction | Eligible country/device subset within supported mix, 0–1 | 0.60 |
| Activity factor | Expected rewarded activity on eligible device-days, 0–1 | 0.80 |
| Start / end week | Production weeks in which task contributes | 5 / 52 |
| Daily pool cap | Maximum UP across the portfolio per day; 0 means no entered cap | 0 |
| Extra cost | Additional UNO cost in USD per qualifying device-day | 0 |

The default active task is **Illustrative combined reward pool — REPLACE**. All real task names are disabled with rate zero until measured values are provided. Disable the placeholder before enabling real task rows. The calculator rejects an active illustrative pool combined with active named tasks, preventing accidental double counting. Multiple real tasks are additive only if they can operate concurrently; adjust activity/eligibility or use scenario runs for mutually exclusive workloads.

For a per-call task: `daily_reward = reward_per_verified_call × expected_accepted_calls_per_eligible_device_day`. The CSV allocation column cannot supply that call denominator without definition. If a daily average already includes zero-activity days, use activity=1 rather than applying the same downtime reduction again.

Rates entered as the historical UNO 50% share are divided by 0.50 to reconstruct the pool. Rates entered as current ULO receipts are divided by the selected ULO share. Pool rates require no reconstruction. A reward must not be grossed up twice. The historical 50% basis is fixed to the owner's supplied export convention; it is not the proposed 40% UNO share.

### 16.9 Formula and regeneration algorithm

Let `N_w` be opening productive licences, `K` capacity, `g` target net additions, `c` weekly churn and `a` activation success:

```text
churn_w       = c × N_w
net_growth_w  = min(g, max(0, K − N_w))
successes_w   = net_growth_w + churn_w
attempts_w    = ceil(successes_w / a)
failures_w    = attempts_w − successes_w
closing_w     = N_w − churn_w + successes_w
```

The implementation subtracts a tiny floating-point tolerance before `ceil` to avoid spurious rounding from values such as 300.00000000000006. Expected productive counts can be fractional; actual operational counts are integers reconciled each week.

Distribute successes equally across seven days. On day `d`, let `B_d` be surviving previously active devices and `A_d` new successful devices:

```text
productive_device_days_d = B_d + 0.5 × A_d
supported_share_j        = sum(device_mix_k for supported device types k)
qualifying_device_days_jd = productive_device_days_d
                           × supported_share_j × eligible_j × activity_j
pool_rate_j = daily_reward_j × USD_per_UP / basis_share_j
  where basis_share = 1 for pool, 0.50 for historical UNO,
                      current ULO share for ULO-denominated rewards
raw_reward_jd = qualifying_device_days_jd × pool_rate_j
reward_jd = min(raw_reward_jd, daily_cap_j × USD_per_UP) if cap > 0
            otherwise raw_reward_jd
weekly_pool_w = sum(reward_jd for enabled tasks and days in week w)
UNO_earned_w   = UNO_share × weekly_pool_w
ULO_earned_w   = ULO_share × weekly_pool_w
referral_w     = referral_share × weekly_pool_w
```

Only tasks within their start/end weeks contribute. Fixed mix fractions sum to one, so a task supporting Android and Windows does not count the same device twice. Capacity caps apply to each task independently; a shared cross-task budget must be represented through adjusted task caps or a future shared-budget extension.

```text
credit_cost_w = credit_price × (gross attempts + surviving 30-day renewals)
support_w = support_per_month / 30
            × (productive_device_days_w + 0.5 × failed_attempts_w)
task_cost_w = sum(qualifying_device_days_jd × extra_cost_j)
fees_w = fee_fraction × UNO_earned_w
capital_allocation_w = historic_licence_cost / allocation_weeks
                       during the selected allocation period; otherwise 0
pretax_w = UNO_earned_w − credits_w − support_w − task_cost_w
           − acquisition_w − coordinator_w − overhead_w − fees_w
           − capital_allocation_w
tax_w = max(0, pretax_w) × tax_fraction
net_profit_w = pretax_w − tax_w
cumulative_profit_w = −setup_cost + sum(net_profit_1 ... net_profit_w)

UNO_receipts_w = (UNO_earned − fees) from week (w − settlement_lag)
                or zero if that week precedes the forecast
cash_expenses_w = credits + support + task_cost + acquisition
                  + coordinator + overhead + tax
net_cash_flow_w = UNO_receipts_w − cash_expenses_w
cumulative_cash_flow_w = −setup_cost + sum(net_cash_flow_1 ... net_cash_flow_w)
closing_cash_w = opening_funding + cumulative_cash_flow_w
required_funding = max(0, −minimum cumulative cash flow including setup)
```

Fee expense is recognised on earned UNO rewards and deducted from the eventual receipt, avoiding a second cash fee deduction. Tax is a simplified positive-week provision paid immediately with no loss offsets; it is not jurisdiction-specific tax advice. Historic licence-cost allocation reduces profit but not future cash because the model assumes the licences are already purchased. Future purchases or debt repayments must be budgeted as cash costs separately.

Cohorts retain their activation day. On each day 30, 60, 90, etc. after activation, surviving licences buy a new credit block. Churn reduces each cohort before that week's renewals. Opening productive licences, if entered, are assumed to need fresh blocks immediately for surviving inventory; prepaid balances or differing renewal ages require an adjusted initial cohort schedule. This limitation matters when reusing the model for an existing portfolio.

### 16.10 Adding a task: worked incremental example

After replacing the illustrative pool with your verified existing tasks, add an Android task paying **0.05 UP/day**, eligible fraction **60%**, activity **80%**, starting week 5, with no entered cap or extra cost. Keep every other setting unchanged. At the reference week-10 device exposure:

```text
Incremental effective pool rate = 0.05 × 0.60 × 0.80 = 0.024 UP/device-day
Incremental week-10 pool        = 16,467.5 × 0.024 = 395.22 UP
Incremental ULO revenue         = 395.22 × 50% = 197.61 UP
Incremental referral revenue    = 395.22 × 10% = 39.522 UP
Incremental UNO revenue/profit  = 395.22 × 40% = 158.088 UP
```

At $1/UP and zero additional fees/tax/cost, week-10 profit improves by **$158.09**. Credits, support, acquisition and coordinator costs do not change. With a two-week settlement lag, that week's additional UNO cash arrives in week 12. If the task introduces resource contention, extra support, transaction fees or tax, adjust those inputs rather than treating incremental revenue as cost-free.

### 16.11 Refresh and decision workflow

1. Reconcile active inventory, task naming and paid licence-days from actual records.
2. Replace the placeholder with task-specific rates and correct reward basis; record measurement dates and sample size outside the calculator.
3. Enter supported devices, eligible proportions, expected rewarded activity and known demand caps.
4. Confirm credit billing, renewal/reassignment behaviour, support, recruitment, labour, additional overhead, fees, tax and settlement lag.
5. Recalculate, inspect weekly net profit and lowest cash balance, then export inputs JSON and weekly CSV with a dated scenario name.
6. Compare downside and reference outcomes. Release the next recruitment cohort only if Section 1 gates still pass and the cash requirement is funded.
7. At each weekly review replace expectations with actuals in the operating ledger and regenerate the forward scenario. The calculator is a scenario engine, not an accounting ledger or live connection to Unetwork.

**Validation performed:** the reference schedule reproduces 3,211 gross attempts over the first ten weeks; weekly reward shares reconcile to the pool; added tasks increase revenue by the formula above without changing other costs; device exclusions, caps, launch dates, zero credits, historical-share normalisation and settlement lag were checked. These checks validate model mechanics, not the chosen revenue assumptions.

## Sources and assumptions

- **S1 — User inputs:** `statistics-incentives-All.csv`, supplied task list and stated inventory of 2,500 licences. CSV figures calculated directly; owner confirms historical 50:50 and distributed-cohort scope. `ups` is modelled as UNO credit subject to recipient verification. Prior estimated per-task rates are removed; only measured task-category credits are used. No live account was inspected.
- **S2 — [Unetwork terms](https://unitynodes.io/terms):** participation/credit rules, geographic restrictions, mobile-device wording and §5.6 promotion/leasing constraints, reviewed 28 September 2026. The campaign permission gate comes from these published terms, not from a restriction on preparing this plan.
- **S3 — [Accounts, KYC and attestation](https://unitynodes.io/learn/accounts-kyc-attestation):** device/account eligibility, reviewed 28 September 2026.
- **S4 — [Business coverage](https://unitynodes.io/business):** observed market evidence from this conversation’s source review; not a country approval list.
- **S5 — [World Bank country/lending groups](https://datahelpdesk.worldbank.org/knowledgebase/articles/906519-world-bank-country-and-lending-groups):** income-group context; not evidence of task availability or individual affordability.
- **S6 — [WhatsApp Business policy](https://business.whatsapp.com/policy):** consent and messaging rules, reviewed 28 September 2026.
- **S7 — [Reddit spam policy](https://support.reddithelp.com/hc/en-us/articles/360043504051-Spam):** community distribution constraints, reviewed 28 September 2026.

- **S8 — [Unetwork FAQ](https://unitynodes.io/faq):** per-operator-licence plan and licence-holder payment option, checked 28 September 2026.
- **S9 — [Official Android release index](https://releases.unetwork.io/android/):** published APK artifacts, including 1.2.4 dated 23 September 2026; checked 28 September 2026. Publication does not certify compatibility or security.
- **S10 — Peer review inputs:** supplied `LICENSE-DISTRIBUTION-MARKETING-PLAN.md` and the subsequent pasted review; evaluated against user clarifications, CSV and official sources. Disposition recorded in Section 15.

All suggested conversion rates, weekly output quotas, churn rates, staffing costs and budgets are explicit planning assumptions. They are neither vendor quotations nor performance established by the CSV. This deliverable is a marketing and operating plan; no advertising was purchased, partner contacted, licence assigned or account changed.

**Revision audit:** Revision 2 established 50/40/10, UNO-funded credits and CSV-only task evidence. Revision 3 retains those foundations and the channel inventory, message drafts, churn model and reserve; adds launch/capacity gates, zero-credit sensitivity, participant/promoter tests, strategic partnership hypotheses and a downside schedule; corrects overconfident channel positioning and distinguishes validation time from production time. No per-device earnings or network capacity is inferred from the aggregate export. Revision 4 adds transparent conditional revenue projections and a self-contained task-driven calculator; reference reward assumptions are illustrative and replaceable.
