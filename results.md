# Unetwork — Comprehensive Analysis & Technical Audit

**Subject:** Unetwork / Unity Network Limited (Hong Kong, CRN 77515624)
**Analysis date:** 28 September 2026
**Coverage figures dated:** August 2026 (company self-reported)
**Version:** 2 — revised after adversarial review. Supersedes v1 of the same date.

**Change notice.** v1 was reviewed by an independent party and did not survive it. This revision withdraws several v1 conclusions, downgrades the source-code section from *findings* to *unverified observations*, and replaces a single severity scale with a two-axis confidence model. Material withdrawals are listed in **Appendix B**. Readers who relied on v1 should read Appendix B first.

---

## 0. How to read this document

### 0.1 Confidence model

Every finding carries two independent labels. The separation matters: v1 collapsed them, which is how inferences came to be presented as defects.

| Evidence status | Meaning |
|---|---|
| **Verified** | Directly observed and independently re-checkable from a cited source |
| **Documented claim** | Asserted by the company in a published document; not independently confirmed |
| **Static observation** | Present in a shipped artefact or source file; says nothing about runtime behaviour |
| **Inference** | A conclusion drawn from evidence; may be wrong |
| **Unresolved** | The question was not settled and should not be treated as answered |

Impact is rated **Critical / High / Medium / Low** *independently* of evidence status. A Critical-impact item resting on Unresolved evidence is a prompt to investigate, not a finding.

### 0.2 Evidence classes

| Class | Source | Yields |
|---|---|---|
| **A** | Public marketing site | Positioning, catalogue, self-reported metrics |
| **B** | Legal + documentation | Intended policy and economics |
| **C** | Shipping Android artefact (v1.2.4) | Declared capabilities, bundled code |
| **D** | Local codebase under `app-stack/u-network` | **Provenance unverified — see Part 7** |

**Nothing in this document is Critical on the strength of an inference.** The sole Critical item is a term the company published itself.

### 0.3 What this document is not

It is a documentary and static review. No account was created, no purchase made, no KYC or task flow exercised, no transaction submitted, no on-chain deployment queried. It is not a security assessment, a legal opinion, a financial audit, or a penetration test.

---

# PART 1 — WHAT THEY DO

## 1.1 The business

Unetwork describes itself as a DePIN (decentralised physical infrastructure network) built from consumer smartphones. Real handsets on live carrier networks act as distributed edge endpoints running carrier verification and measurement tasks.

The claimed differentiator against a conventional device farm is that endpoints sit **inside real mobile markets** — real SIM, real routing, real congestion, real destination behaviour. The commercial proposition is *proof at the edge*: a verifiable, timestamped record that a telecom event occurred as reported, produced by a third party with no stake in the outcome.

## 1.2 The product catalogue

| Product | Question it answers | Buyer |
|---|---|---|
| **Real-Device QA** | Does this journey work on this device, carrier and market — and how long did each step take? | QA / release teams |
| **Network Intelligence** | What do real handsets on real networks see where you cannot instrument? | Network analytics, fraud, identity teams |
| **Telecom Assurance** | Did the call / SMS / OTP actually arrive as sent, at the destination? | Carriers, aggregators, CPaaS |
| **Entropy Contribution** | Randomness contributions from device sensors and system sources, with provenance | Security / randomness consumers |
| **Survey Tasks** | Structured research in a targeted market | Research / market intelligence |

A QA customer could specify device model, OS, carrier and registration flow; the company describes dispatching the test to a matching physical phone and returning evidence per step. Note that a successful sample run is not evidence that every requested market/device combination is continuously available.

## 1.3 Network architecture — and two claims that do not survive scrutiny

The Litepaper (p.12) describes Switch Nodes routing and timestamping traffic, Validation Nodes checking integrity, and Earth Nodes aggregating and exposing results, with activity metadata hashed to WMChain.

Three qualifications belong here, two of them supplied by the independent review:

**Geographically distributed phones do not establish distributed control.** Admission, scheduling, task assignment, validation, reward calculation and withdrawal are all functions of a single operator. A network whose *measurement points* are distributed while its *control plane* is centralised is a distributed execution network, not a decentralised one. Class: **Inference** (well supported; see Part 6.3 for the control-plane surface).

**Hashing a measurement proves non-alteration after collection, not truth at collection.** A hash commits to a value; it does not establish that the value was correctly measured, correctly located, or commercially meaningful. Those properties depend on collection integrity, attestation, validation and fraud controls, none of which are specified in the reviewed material. Class: **Inference.**

**Physical provenance does not establish randomness quality.** Contributing entropy from a physical device is not the same as supplying cryptographically strong randomness. Entropy estimation, conditioning, statistical health testing and adversarial resistance would each require separate technical evidence, and none is presented. Class: **Inference.**

**WMChain activity is not established.** The shipped client contains Alchemy endpoints for WMChain mainnet and testnet (`worldmobilechain-mainnet.g.alchemy.com`, `testnet-explorer.worldmobile.net`). *v1 claimed this confirmed "real and operational" — that was wrong.* A bundled URL proves neither that the endpoint is called nor that any write succeeds. Establishing the relationship between task evidence and an on-chain commitment requires transaction hashes. Class: **Static observation → Unresolved.**

## 1.4 Self-reported scale (August 2026)

| Metric | Value | Status |
|---|---|---|
| 30-day active devices | ~58,000 | Documented claim |
| High-resolution devices | ~17,000 | Documented claim |
| Countries | 185 | Documented claim |
| ASNs observed | 3,745 | Documented claim |
| Observations / day | ~11,000,000 | Documented claim |
| Unique public IPs (30d) | ~366,000 | Documented claim |

None independently confirmed. See **R13** for an arithmetic check that materially changes how the country figure should be read.

## 1.5 Commercial structure

The published model assigns **75% of generated service fees to the operator side**, with a negotiated UNO share deducted when a ULO leases its licence. The remaining 25% supports the MNTx/WMTx ecosystem. Source: Litepaper pp.7–12.

Worked illustration on a hypothetical $100 of attributable service fees, using the app's default template split (see §5.2) — **illustration, not a forecast:**

| Recipient | Calculation | Amount |
|---|---|---|
| MNTx/WMTx ecosystem | $100 × 25% | $25.00 |
| UNO | $100 × 75% × 30% | $22.50 |
| ULO | $100 × 75% × 70% | $52.50 |

*Before credits, connectivity, power, device depreciation or time. Whether actual task accounting follows this path is unconfirmed — see R8.*

---

# PART 2 — WHAT UNOs ARE

**UNO = Unetwork Node Operator** — licence owner / capital provider. Not a device operator.

## 2.1 The marketed product

| Attribute | Stated value | Status |
|---|---|---|
| Price | US$10,000 (Round 2) | Documented claim |
| Bundle | 200 Unetwork Operator Software Licences | Documented claim |
| Node supply limit | 6,000, stated as never increasing | Documented claim |
| Implied total operator licences | 1,200,000 | Documented claim (arithmetic, not stated) |
| Personal purchase cap | 100 | Documented claim |
| Payment | MNTx, WMTx, ADA, BTC, DOT, ZEBC, SHIB, ETH, BNB, USDT, USDC; cards (max 2 nodes / $25,000 per transaction) | Documented claim |
| Validity | Indefinite, no expiry | Documented claim |
| Representation | NFT, Ledger-compatible | **Unresolved — see Part 7 and R1** |

The homepage markets these as NFTs. **No contract implementation was identified in the inspected paths, and no contract address appears in any public document.** Whether the licences are tokenised is therefore an open question, not a settled fact. See Part 7 for the full position and its limits.

## 2.2 What a UNO does

Manages node capacity, self-operates licences, or leases them to ULOs and collects a negotiated share. ToS §5.6 prohibits publicly advertising or marketing operator licences for money outside the Unity ecosystem.

**200 licences are capacity, not 200 productive devices or revenue streams.** This is the single most important economic point in this document. A UNO buying capacity must recruit operators, agree viable lease terms, and maintain activity; idle licences generate nothing and, at the default uptime template, may breach their lease conditions. The gap between licensed capacity and productive fleet is where the entire financial outcome of a $10,000 purchase sits. Class: **Inference**, from the documented ownership/operation split.

## 2.3 What a UNO receives

- **75%** of network service fees from licences they operate, or their negotiated share from leased licences
- **Complementary allocations** of MNTx and WMTx from third-party treasuries. **The stated amount appears in four different forms across four sources** — see R11. FAQ splits the 25% as WMTx 12.5% (buyback pool) and MNTx 12.5% (DePIN service fee pool, per epoch)
- **UNT allocations** per the draft whitepaper: Round 1 = 38.5% (12-month linear), Round 2 = 24.5% (60-month linear), weight multiplier 1.0×–5.0× based on staked MNTx + WMTx (up to 100,000 MNTx + 200,000 WMTx for 5.0×). **Proposed arrangements, not established benefits**
- **Governance** restricted to node holders, weight ∝ UNT staked on the node

## 2.4 The 24-month binary election (ToS §5.4) — Critical

After the lock period, the holder faces one irreversible choice:

- **Keep** the complementary allocations → node operates indefinitely, **or**
- **Withdraw** them → **the node ceases immediately and permanently, and the Licence NFT irrevocably reverts to the Company**, returning to the open market.

No partial withdrawal, substitution, top-up or reuse. ToS p.8 pre-frames this as *"not a penalty but the necessary operational consequence."*

This is a **documented claim, not a discovered defect** — the company publishes it. It is rated Critical because of the asymmetry: $10,000 non-refundable, no chargebacks accepted (ToS §8.6), no company liability if the third-party treasuries withhold the allocation (ToS §5.2), 24 months during which withdrawal is impossible under any circumstances, then permanent total forfeiture on withdrawal.

*Assessment of enforceability in any given jurisdiction is a legal question outside the scope of this document.*

---

# PART 3 — WHAT ULOs ARE

**ULO = Unetwork License Operator** — the device operator. Economically distinct in kind from a UNO.

> *"anyone who operates a Unetwork Operator Software License on a smartphone using the Unetwork app... performing verification tasks from your smartphone and receiving network incentives for verified participation."*
> — ULO Handbook p.4

## 3.1 The distinctions that matter

| Question | Answer |
|---|---|
| How many licences per **device**? | **One.** One licence per phone; a phone runs one Operator Software Licence |
| How many devices per **account**? | **Many.** A single ULO account can manage many licences across many devices. No separate account, email or KYC per phone |
| How many verified accounts per person? | **Two** |
| Can an account operate licences it owns? | **No** — a second account with a different email is required |

*Correction: v1 stated "a ULO holds exactly one licence" and simultaneously stated that one account manages many licences. Those contradict. The restriction is per **device**, not per account.*

## 3.2 Token entitlement — precisely stated

**The draft whitepaper defines no allocation for ULO work.** v1 said ULOs receive "no UNT token allocation whatsoever," which overstated it: the draft expressly permits overlapping eligibility categories, so a ULO could separately qualify as a token holder, node operator, or MNTx/WMTx holder by acquiring that status.

The accurate statement: **ULO participation generates no token entitlement by itself; any token exposure comes from separately acquiring a qualifying status.** The party bearing device, power and connectivity cost has no automatic claim on the token, while governance is restricted to node holders and the draft states proposals originate with the internal team. Class: **Documented claim.**

## 3.3 Obtaining a licence

Three distinct routes — **v1 conflated these and produced a self-contradictory description; corrected here:**

1. **Secondary-market purchase** from a UNO or a self-operating ULO. The FAQ qualifies that purchaseable licences must be self-operated and *not* currently leased out. That qualifier attaches to this route only.
2. **Gift or in-app transfer**, from a ULO giving up a licence.
3. **Private lease code** issued by a UNO, or lease claimed from the in-app marketplace.

The April guide additionally shows a **"Skip"** option on the lease-code step, confirming a licence can be operated with no lease in place.

## 3.4 The role model

Roles are **chosen at sign-in** and apply to that session; an account is not permanently one role. Email aliases (plus- and dot-addressing) are treated as the *same* account, not separate users.

## 3.5 Task types

| Task | Attestation requirement |
|---|---|
| CLI / Caller ID Verification | None |
| SMS Sender ID & Content Testing | None |
| Connectivity Verification | None |
| OTP / authentication flows | Per task |
| Extended Telemetry | **≥ software attested** |
| Entropy Contribution | **≥ software attested** |
| Surveys | None |

Tasks are opt-in / opt-out individually. Device states are hardware attested, software attested, or unattested; all three may connect to the network, but unattested devices are ineligible for Extended Telemetry and Entropy. Hardware and software attestation are currently treated identically for incentive purposes.

## 3.6 Rewards and cost

**Rewards:** UP points, 1 UP = $1.00 USD, converted to crypto. Paid in Bitcoin, Ethereum, ADA, select partner assets and USDC. Crypto only — the FAQ answers directly: *"Rewards are paid in cryptocurrency."* Rewards do not expire — the FAQ answers directly: *"Rewards do not expire."*

**Cost:** each Operator Software Licence requires its own activation-credit plan, stated as $1.99–$3.99/month. Credits are not a currency, are unusable outside the platform, and pricing may change. If leased, the UNO may fund the plan.

**The ULO's take is negotiable, not fixed.** See §5.2.

**Net-benefit test.** A ULO's position is:

> Net benefit = accepted incentives after lease sharing − credits paid personally − incremental connectivity, power and device costs − **value of time spent**

*That last term is the one most often omitted, and for a task-queue-driven device it is likely the largest.* Assess against observed work from the intended device and location. An attractive percentage split is worth little if eligible work is scarce — the split applies only to work that is actually accepted. Class: **Inference.**

---

# PART 4 — WHERE ULOs OPERATE

**This section deliberately does not produce a country allowlist.** An earlier draft of this report did, and that was a mistake: a non-exhaustive exclusion list cannot be inverted into a positive permission list. The reasoning is set out below.

## 4.1 The observed footprint — and what it actually means

The business page reports ~58,000 30-day active devices across 185 countries (August 2026) and publishes a chart naming 20 countries:

| Country | Active devices | Country | Active devices |
|---|---|---|---|
| India | 13,316 | Romania | 788 |
| Australia | 10,093 | South Africa | 744 |
| United States | 8,842 | Kenya | 743 |
| United Kingdom | 2,909 | Croatia | 728 |
| Netherlands | 2,118 | France | 619 |
| Philippines | 2,032 | Indonesia | 493 |
| Nigeria | 1,722 | Switzerland | 410 |
| Canada | 1,307 | Bangladesh | 388 |
| Viet Nam | 1,103 | Greece | 386 |
| Germany | 1,092 | Spain | 975 |

Ghana additionally appears in a network sample on that page. Brazil appears in an illustrative QA run but is not added to the coverage chart on that basis.

**The arithmetic that reframes the 185-country claim:**

| Quantity | Value |
|---|---|
| Sum of the 20 published cohorts | **50,808** |
| Headline active devices | 58,000 |
| Share of fleet in the top 20 | **87.6%** |
| Remainder across the other 165 countries | **7,192** |
| Mean devices per remaining country | **≈ 44** |

Roughly 44 devices per country, for the remaining 165 countries. For a network whose QA proposition is *device-, carrier- and market-specific* verification, that is not operational coverage. "185 countries" is true of the header number and misleading as a statement of service availability. **Verified** (arithmetic on published figures). See R13.

## 4.2 Every explicitly named restricted jurisdiction

ToS §2.4, effective 1 December 2025:

| Policy category | Named jurisdictions |
|---|---|
| VoIP restrictions | China, Oman, United Arab Emirates |
| Sanctions-related exclusions | Afghanistan, Belarus, Cuba, Iran, Myanmar, North Korea, Russia, South Sudan, Sudan, Syria, Venezuela, Yemen, Zimbabwe |
| Regional exclusions | Crimea, Donetsk and Luhansk regions of Ukraine |

This is **Unetwork's policy list**, not a verified statement of current sanctions or telecom law. It may be updated "from time to time," and no changelog is published.

## 4.3 Why no allowlist can be certified

Four questions must be separated, and an affirmative answer to any one does not establish the others:

1. **Has a device been observed** in that country?
2. **Can a resident register** and complete verification?
3. **May the device perform** the intended task category there?
4. **Can that account redeem** the resulting rewards?

There is also an unresolved conflict. The FAQ describes operation in "all countries" subject to VoIP/sanctions exclusions; elsewhere it permits **node ownership** in VoIP-restricted countries and suggests travellers can still earn from some services there. The ToS exclusion is written more broadly, as a location-based bar on use. These are not reconcilable from the text alone.

**Position taken:** for a country absent from the published table, status is **unconfirmed — not necessarily unavailable**. For a named restricted jurisdiction, the permissive FAQ wording is insufficient to establish reliable access. Anyone depending on a specific country should obtain written confirmation covering residence, operating location, task category, and withdrawal eligibility.

**Position withdrawn from v1:** v1 concluded "everywhere else is nominally permitted." That inferred a permission from a non-exhaustive prohibition and is withdrawn.

**Additional observation:** the published country table is a *device-coverage* chart and does not itself exclude the restricted jurisdictions. A reader scanning it could reasonably conclude the UAE or China is covered. That is a presentation issue worth correcting on its own terms.

## 4.4 No per-country cap

The FAQ states there is no ULO device limit per country, so unlimited devices per location is permitted, though "not encouraged."

---

# PART 5 — FUNDAMENTAL ULO REQUIREMENTS

## 5.1 Legal and eligibility (binding — ToS)

| Requirement | Source | Status |
|---|---|---|
| 18+ with full legal capacity | §2.1 | Documented claim |
| Not located in a ToS §2.4 jurisdiction | §2.4 | Documented claim; current list Unresolved |
| Tasks originate **solely from a personal mobile device**; datacenter IPs, VPS, VPN, proxy and automated infrastructure prohibited; violation triggers immediate IP blocking or banning | §2.5 | Documented claim |
| Lawful use; no prohibited use (fraud, prohibited goods, malware, sanctions circumvention, unauthorised access) | §3 | Documented claim |
| Accept the Terms; material changes notified by email | §1, §17 | Documented claim |
| Participation is opt-in and revocable | Business page | Documented claim |

## 5.2 Licensing, revenue share and recurring cost

| Requirement | Detail | Status |
|---|---|---|
| Hold one Operator Software Licence per **device** | Purchased, gifted, or leased | Documented claim |
| Purchase an activation-credit plan, per licence | $1.99–$3.99/month | Documented claim (ToS §5.6, FAQ) |
| Credits are not a currency; pricing may change | ToS §5.6 | Documented claim |
| Credits must be renewed for the licence to continue operating | April guide | Documented claim |

**The default licence template**, from the April guide's setup-wizard screen:

| Setting | Displayed default |
|---|---|
| Share | **UNO 30% / ULO 70%** |
| Lease duration | 6 months |
| **Minimum uptime requirement** | **99%** |
| Marketplace visibility | Disabled |

Three precise qualifications, correcting v1 overreach:

- These are **template defaults, not universal production defaults.** The guide states defaults are editable under Settings → License Templates, so a UNO may set different values per licence. A screenshot establishes what the wizard displays, not what every lease contains. Class: **Static observation → Unresolved** as to production values.
- At 99%, a device is permitted roughly **7.2 hours of downtime per month** — derived from the template value, and illustrative of the burden, not a compliance guarantee.
- **The relationship between the uptime condition and the FAQ's "no penalties for devices going offline" is undetermined.** A lease condition, lost earning opportunity, a monetary penalty, and licence revocation are four different mechanisms. v1 asserted a contradiction; the honest position is that the enforcement mechanism is not specified. Class: **Unresolved.**

**Revenue share is negotiable.** The 30/70 template is one axis; the 75/25 network allocation in §1.5 is another, and they compose as shown there. A ULO's actual share is whatever the specific lease states. Class: **Documented claim** for the default; **Unresolved** for any given lease.

## 5.3 Hardware

| Requirement | Detail |
|---|---|
| One supported Android or iOS smartphone per licence | Handbook p.5 |
| Most modern smartphones; described as "incredibly lightweight, not CPU or RAM intensive" | FAQ |
| Older and second-hand devices acceptable | FAQ |
| A separate phone is not required | FAQ |
| **No substantiated universal minimum RAM, storage, CPU or OS version exists in the requirements material** | — |

*"Most smartphones" is not a compatibility guarantee.* Check the current app listing and test the actual handset before acquiring additional devices. Class: **Documented claim.**

## 5.4 Connectivity

| Requirement | Detail |
|---|---|
| Stable connectivity; Wi-Fi is sufficient | Handbook p.5 |
| Some tasks require mobile data | Handbook p.5 |
| Works on Wi-Fi with no cell service at all | FAQ |
| No minutes consumed — data only; nothing appears on the bill | Home, FAQ |
| Network-agnostic: any carrier, any plan, prepaid or contract | FAQ |
| SIM/eSIM optional; a SIM widens the available task set | FAQ |
| Changing SIM or phone does not invalidate a licence — reassign in-app | FAQ |

A SIM is **not** established as a universal prerequisite. It is reasonable to expect destination-carrier call or SMS tasks to require suitable cellular capability; confirm per task rather than treating Wi-Fi eligibility as access to every workload.

## 5.5 Account and identity

| Requirement | Detail | Status |
|---|---|---|
| Account with ULO role selected at sign-in | Learn guide | Documented claim |
| Sign-in by email (recommended), email OTP, or Web3 wallet (MetaMask, WalletConnect, Coinbase; 40+ wallets) | Handbook p.9, April guide | Documented claim |
| One account may manage many licences and devices | Learn guide | Documented claim |
| KYC via identity document (passport, national ID, driving licence) | Learn guide, FAQ | Documented claim |
| KYC required at 5 UP, at withdrawal, or earlier for specified tasks | Learn guide | Documented claim |
| Maximum 2 KYC-verified accounts per person | Learn guide | Documented claim |
| An account may not operate the licences it owns | Learn guide | Documented claim |
| A UNO's email and Telegram become visible to ULOs leasing their licences (opt-in) | April guide | Documented claim |
| Merge is ULO→ULO only; Scout & Runner registrations must share a country | Learn guide | Documented claim |

## 5.6 Operational conduct

| Requirement | Detail | Status |
|---|---|---|
| Keep the app online and powered on; a minimum uptime condition is a configurable lease parameter | April guide (99% template), Handbook p.8 (95% worked example) | Static observation; production values Unresolved |
| Green "active" indicator with all statuses "Active" confirms correct operation | FAQ | Documented claim |
| Attestation gates Extended Telemetry and Entropy; all states may connect | Learn guide | Documented claim |
| Google Play Integrity referenced in the shipped client | APK | Static observation |
| The app declares **no** contacts, SMS, call-history, phone-state or microphone permission | APK | Static observation (see Part 6.1 for scope limits) |

## 5.7 Withdrawal

| Requirement | Detail | Status |
|---|---|---|
| Via the Unetwork Web Rewards Centre (`manage.unetwork.io`), not the app | Handbook p.16, April guide | Documented claim |
| Amount, destination address, chain, asset | Handbook p.17 | Documented claim |
| **Crypto only, never to a bank account** — FAQ: *"Rewards are paid in cryptocurrency."* | FAQ | **Verified** |
| **Rewards do not expire** — FAQ: *"Rewards do not expire."* | FAQ | **Verified** |
| Single transaction on withdrawal | FAQ | Documented claim |
| **Minimum: $5.00 (Handbook p.16) / 5 UPs (April guide) / "no minimum except fees" (FAQ)** | three sources | **Verified conflict** — see R7 |
| *"Minimum claim amount and payout schedules depend on network load and governance policies"* | April guide | Documented claim — see R7 |
| Crypto sent to a wrong address cannot be recovered; irreversible | April guide | Documented claim |

---

# PART 6 — ARTEFACT AUDIT: THE SHIPPING APPLICATION

| Attribute | Value |
|---|---|
| Artefact | `Unetwork_App-1.2.4.apk` (23-Sep-2026 21:56 UTC) |
| Size | 99,853,230 bytes |
| SHA-256 | `06205fcd90f9859717d769c3c552ce38be5224eb028c35a5c3fb47113428e33c` |
| Package | `io.unetwork.app` |
| versionCode / versionName | 53 / 1.2.4 |
| targetSdk | 36 (Android 16) |
| ABIs | arm64-v8a, armeabi-v7a, x86, x86_64 |
| Stack | React Native + Expo (Hermes VM), expo-updates, Firebase Crashlytics |
| Declared permissions | **41** |

## 6.1 Permissions — what a manifest can and cannot establish

Android distinguishes **declaring** a permission from **requesting** it at runtime, from the user **granting** it, from the code path **executing**, from data being **transmitted**, and from **retention**. A manifest dump establishes only the first. *v1 collapsed all six into "the app collects" — that was the report's largest methodological error.*

**What the manifest shows (Static observation):**

| Permission | Why it is notable |
|---|---|
| `CAMERA` | On-device ML Kit face detection (`face_detection_short_range.tflite`) and barcode scanning (`libbarhopper_v3.so`) are bundled. Plausibly KYC liveness and QR lease/wallet codes — **not established** |
| `ACCESS_BACKGROUND_LOCATION` + `FOREGROUND_SERVICE_LOCATION` | Would permit continuous background location **if granted and exercised** |
| `SYSTEM_ALERT_WINDOW` | Overlay capability |
| `WRITE_SETTINGS` | System settings modification |
| `USE_BIOMETRIC`, `USE_FINGERPRINT` | Biometric authentication. **Does not imply access to raw biometric templates** |
| `REQUEST_IGNORE_BATTERY_OPTIMIZATIONS` | Consistent with a sustained-uptime design |
| `com.google.android.finsky.permission.BIND_GET_INSTALL_REFERRER_SERVICE` | Install-referrer attribution |
| `com.android.vending.BILLING` | Google Play Billing |
| 15 launcher-badge permissions (HTC, Huawei, Oppo, Samsung, Sony, Majeur, Badger) | Via `me.everything.badger`; benign UI polish |

**Not declared:** `READ_SMS`, `READ_CONTACTS`, `READ_PHONE_STATE`, `CALL_PHONE`, `RECORD_AUDIO`.

**Correctly bounded findings:**

- The **declared** permission set warrants a data-flow and consent review. **Actual collection, transmission and retention were not established by this analysis** — no runtime flow was exercised.
- The **absence** of `READ_CONTACTS` is evidence about that permission only. It does not prove no contact information can enter through another user-mediated flow.
- A bundled model or SDK does not establish that it is exercised.
- Biometric permissions do not imply access to raw fingerprint or facial templates.

## 6.2 The disclosure question — which does survive, on separate grounds

The FAQ answer is stronger than the homepage phrasing:

> *"Unetwork does not access your personal data, **or request access to your personal data**. Unetwork is GDPR compliant."*

Two distinct claims, which must not be merged:

| Claim | Status |
|---|---|
| "The app collects background location and facial imagery" | **Withdrawn from v1.** Not established |
| "The published privacy claim is unqualified and unexplained against the app's declared capabilities" | **Stands — on disclosure grounds, not collection grounds** |

The second survives because it does not depend on runtime behaviour. A user presented with a camera or background-location prompt has no in-app explanation for it, and the site affirmatively states such access is not requested. Whether the prompts appear at all is Unresolved; that the *stated* basis is narrower than the *declared* surface is Verified. A defensible formulation:

> *The declared permissions warrant a data-flow and consent review. The published claim that personal data is not accessed **or requested** is unqualified and is not reconciled with the app's declared capability set. Actual collection and disclosure have not been established by this analysis.*

A blanket GDPR-compliance assertion alongside an unreconciled declared surface is a **disclosure** exposure requiring legal review — not a finding that data is collected.

## 6.3 Control-plane surface

| Domain | Role |
|---|---|
| `unitynodes.io` | Marketing, legal, Terms |
| `unetwork.io` | Per the Learn guide, the "wider platform FAQ" |
| `unetwork.app` | App-facing; exposes a `/privacy-archive` route — repeated privacy-policy churn |
| `api.unityedge.io` | Primary API (Supabase object storage) |
| `auth.unityedge.io` | Authentication |
| `u-edge.io` · `validation.` · `trust.` · `switch.uedge.io` | Edge / attestation services |
| `scoutandrunner.com` | Corresponds to the in-app Scout & Runner merge feature |
| `manage.unetwork.io` | Web Rewards Centre |
| `releases.unetwork.io` | APK distribution |

*Class: Static observation.* Nine first-party domains, none of the relationships documented. Combined with the KYC, attestation and reward paths below, this is the concrete basis for the control-plane point in §1.3.

## 6.4 Bundled third parties

| Third party | Apparent role | Note |
|---|---|---|
| Stripe | Payments | Bundled; active flows not established |
| Paddle | Merchant of record | Bundled; active flows not established |
| RevenueCat | Subscription management | Bundled; active flows not established |
| Google Play Billing | In-app purchases | Declared permission |
| **Sumsub** (`test-api.sumsub.com`) | KYC | **Test endpoint.** See correction below |
| Cloudflare Turnstile | Bot detection | Bundled |
| WalletConnect / Reown / web3modal, Coinbase Wallet | Wallet connectivity | Bundled |
| Firebase Crashlytics | Crash / device telemetry → Google | Bundled |
| Alchemy | WMChain mainnet + testnet | Endpoints configured; no call evidence |
| Arweave | Decentralised storage | Referenced |
| OpenChain / Solscan | Address validation / Solana | Referenced |
| Google Play Integrity | Device attestation | Referenced |

**Correction on Sumsub.** v1 quoted `test-api.sumsub.com` — a *test* endpoint — to support a production claim. That was bad reasoning and is withdrawn as evidence. The conclusion that **Sumsub is the current KYC vendor** rests instead on the Learn guide (September 2026), which states individual KYC is handled through Sumsub, with completed ShuftiPro verifications remaining valid. Production configuration remains **Unresolved**; the documentation contradiction in R10 is unaffected.

**Four bundled payment SDKs for $1.99–$3.99/month of credits.** ToS §8.6 states fees are non-refundable and chargebacks are declined. Where a merchant-of-record processor (Paddle, RevenueCat) or Google Play Billing sits between the user and the platform, its own refund regime and the user's statutory rights in a given jurisdiction may sit in tension with that clause. **v1 asserted the Terms were "not enforceable as written" — that is a legal conclusion beyond both the evidence and this document's competence, and is withdrawn.** The defensible statement: the clause is in tension with the bundled processors' regimes; resolution requires transaction-specific legal analysis. See R8.

**Point in the application's favour:** no advertising SDK and no Adjust, AppsFlyer, Amplitude, Mixpanel or Segment were found. For an application of this sensitivity, that is unusual and worth preserving.

## 6.5 Signing and distribution — v1's criticism was wrong, and is withdrawn

```
Signer #1 certificate DN: CN=, OU=, O=, L=, ST=, C=US
SHA-256: cbf7602663536620a4cb612cf31ba167dcf1d0738b0ccbe8534c21d2efd69ab5
```

**The empty subject is not a defect.** Android's update model relies on signing-key provenance, continuity, protection and verification against a trusted release — not on human-readable organisation fields. Adding `CN=Unity Network Limited` would make the certificate look better and prove nothing; **v1's recommendation to do so was security theatre and is withdrawn.**

**v1's distribution criticism was also wrong in kind:**

- Public availability does not mean an APK lacks cryptographic authentication. It is signature-verified like any other build.
- Retaining historical releases is good practice for audit and reproducibility, not a risk.
- A skipped version number (1.2.3 is absent) and a size reduction (148.9 MB → 99.9 MB) are ordinary release-engineering artefacts. **v1 implied concealment without basis; that implication is withdrawn.**
- Requiring login for download would not address signing-key or update-integrity questions.
- Removing OTA is **not** the correct recommendation. Expo supports client-verified, cryptographically signed updates. **Whether Unetwork uses that protection is Unresolved and should be checked before any distribution recommendation is made.**

The legitimate residual concern is narrower and real: the install guide presents sideloading as a co-equal channel, so a user may install an app signed by a key whose provenance they cannot independently confirm, over a public index, with an update channel whose signing status is unknown. Class: **Static observation → Unresolved.**

---

# PART 7 — LOCAL CODEBASE: UNVERIFIED

> **This entire section is demoted from v1.** It is retained because the leads are worth pursuing, not because the conclusions are established. **Nothing here is a finding about the platform.**

## 7.1 Provenance — the finding that governs this section

An independent reviewer asked for repository ownership, commit hashes, scope, deployment linkage, and an explanation of how these services relate to the shipping application. Those were not gathered before v1 was written. They have now been checked, and the result is negative:

| Repository | VCS | History | Remote | Author | Uncommitted | Licence |
|---|---|---|---|---|---|---|
| `uno-api` | **none** | — | — | — | — | Unlicense |
| `uno-app` | **none** | — | — | — | — | Unlicense |
| `uno-admin` | git | **7 commits**, 2026-06-08 → 2026-06-11 | **none** | `Nissi T. <nissi.tafie@gmail.com>` (an individual) | **111 paths** | Unlicense |

Additionally, `uno-admin/Cargo.toml` declares its dependencies as **local filesystem paths into a separate personal workspace** — `../../../../Dev-x/polkanight/ember/ember-multichain`, `ember-fx/crates/components`, `ember-fx/crates/core`.

**Conclusion:** these are local working copies in a personal development workspace. They are *not* a version-controlled corporate repository, carry no ownership evidence, and cannot be tied to the shipping application or the live site. A directory named `uno-admin` with an agent/ULO dashboard is equally consistent with a node operator's back-office tool, a prototype, or an unrelated implementation.

**Therefore: no Part 7 observation may be cited as a property of the Unetwork platform.**

## 7.2 Deployment linkage — none found

`uno-app/infra` contains genuine infrastructure (Terraform for k3s clusters across apac/europe/americas, Cloudflare, R2 object storage; Ansible; Kubernetes charts). However, **no production domain** — `unityedge.io`, `unetwork.io`, `u-edge.io`, `scoutandrunner.com` — appears anywhere in the infrastructure definitions, migrations, or CMS documentation. Status: **Unresolved.**

## 7.3 The tokenisation question — Unresolved, with a hypothesis tested and rejected

**What was checked:** a whole-tree search (excluding vendored dependencies and build output) found **zero `.sol` files** and **zero Ethereum client crates** in any `Cargo.toml`. No contract address appears in any public document. The shipped app derives an internal UUID from the first 32 hex characters of a licence identifier for keying (`marketplace_service.rs:196-200`), and a test fixture in `uno-admin/src/storage/local_store.rs:174-176` shows licence identifiers being generated locally alongside lease codes and share percentages.

**Hypothesis tested and rejected.** v1 inferred that identifiers were locally generated rather than on-chain. A follow-up test examined 182 unique 32-byte identifiers found in the tree: they are distributed across **99.0% of the 32-bit prefix space** (min `0x0111e175`, max `0xfe837374`, against an expected uniform minimum of `~0x01681681`). That distribution is consistent with **random or hash-derived values, not a sequential counter**, and a null `0x000…0` sentinel is present as the standard unset EVM address. **The sequential-counter theory is withdrawn as wrong.**

**Position:** the identifier scheme is *consistent with* cryptographic hashes and equally consistent with locally generated random keys. A UUID mapping is normal internal keying and does not disprove token backing. There is no positive evidence of a contract, and no proof of its absence.

> **Status: Unresolved.** Whether the licences are tokenised is an open question. Establishing it requires either published contract addresses or direct inspection of a deployment — neither of which was available. This is a lead, not a finding.

## 7.4 The three-way share field — a lead with a specific unknown

`uno_share` / `agent_share` / `ulo_share` appear in production structs (`dashboard_handler.rs:18-19`, `rewards_handler.rs:97-98,111-112`, `dashboard_handler.rs:147-153` builds a `license_id → (agent_name, uno_share, agent_share)` mapping).

**What this does not establish.** The values `47% / 3% / 50%` come from a **unit-test fixture** with placeholder strings (`NGA Lagos1`, `Staffman`). They are test data, not production policy. Three things remain unknown:

1. What the production default shares are.
2. Whether the agent share is carved from the UNO's allocation or is separate.
3. Whether any agent arrangement is disclosed in the actual agreement.

The **data model supports a third role** that does not appear in the ToS, FAQ, Litepaper or either User Guide. That is worth asking about. It is not evidence that an undisclosed party takes revenue. Class: **Static observation → Unresolved.** See R5.

## 7.5 Reward figures are seeded, not derived

`uno-admin/scripts/seed_rewards.py` imports rewards from JSON chunk files into a ScyllaDB keyspace (`unity_dashboard`) via `data/rewards`. **No `reward_rate`, `per_license` or equivalent constant exists anywhere in the inspected tree.** The homepage calculator's $96/licence/month figure therefore has **no counterpart in this code** — either it is computed in code not present here, or this code does not serve that page. Both possibilities are open. Relevant to R9.

## 7.6 A stray credential

`uno-admin/secrets/cisco-test-335812-123bc202bd24.json` is a **GCP service-account private key** (`private_key_id`, `private_key`, `client_email`, `project_id`). The filename indicates a test project, and the directory is in an unpublished, public-domain-licensed working copy with no remote — so exposure risk is low.

It is still a **private key in a `secrets/` path**, and v1's warning is retained in corrected form: not "this may be live material" (it is a test key), but "remove it before any publication, and confirm the test project is decommissioned." Recorded for hygiene, not as a security incident. See R16.

---

# PART 8 — RISK REGISTER

Two independent labels. **No item is Critical on the strength of an inference.**

| ID | Finding | Evidence status | Impact |
|---|---|---|---|
| **R1** | No contract implementation identified; tokenisation of licences unproven and unverifiable from public material | Static obs. + **Unresolved** (provenance, deployment) | **High** |
| **R2** | 24-month binary election: irreversible, total forfeiture of a non-refundable $10,000 | **Documented claim** (ToS §5.4, §8.6) | **Critical** |
| **R3** | Declared camera / background-location / biometric / overlay permissions are not reconciled with the published claim that personal data is neither accessed **nor requested** | Static obs. + Documented claim | **High** |
| **R4** | `Test-api.sumsub.com` in v1 cited as production evidence — reasoning withdrawn; Sumsub conclusion now rests on the Learn guide only | **Withdrawn** → Unresolved | — |
| **R5** | Data model supports a three-way `agent_share` role absent from all public documentation | Static obs. + **Unresolved** (production values, disclosure) | Medium |
| **R6** | 99% minimum-uptime template implies ~7.2 hrs/month permitted downtime; enforcement mechanism undocumented and unreconciled with the FAQ's "no penalties" | Static obs. + **Unresolved** | Medium |
| **R7** | Withdrawal minimum stated three ways ($5 / 5 UP / "no minimum") and described as dependent on "network load and governance policies" | **Verified** (all three confirmed in source) | Medium |
| **R8** | No-refund clause (ToS §8.6) sits in tension with four bundled payment SDKs, two merchant-of-record | Static obs. + Documented claim + **Unresolved** (active flows, contracting merchant) | Medium |
| **R9** | Per-licence reward basis never established; the homepage calculator's $96/licence/month cannot be interpreted without knowing whether it is gross fees or the 75% operator allocation | **Unresolved** | Medium |
| **R10** | ToS §2.2 and the FAQ both still name ShuftiPro; the Learn guide names Sumsub | **Verified** (documents) + Unresolved (production config) | Medium |
| **R11** | Complementary allocation quoted at four different values across four sources | **Verified** | Medium |
| **R12** | Exclusion list is non-exhaustive, mutable, unversioned; no positive allowlist can be derived; FAQ permits node ownership in VoIP-restricted countries in tension with the ToS location bar | Verified + **Unresolved** | Medium |
| **R13** | Top-20 cohorts sum to 50,808 of 58,000 — leaving ~7,192 devices across 165 countries (≈44 each). "185 countries" is not a service-availability statement | **Verified** (arithmetic) | Medium |
| **R14** | UNT whitepaper is an unsigned review draft; minting conditional on completing the node sale; ULO work carries no dedicated allocation | **Verified** (draft stamp); Documented claim (terms) | Medium |
| **R15** | Nine first-party domains, relationships undocumented; control plane centralised despite distributed measurement | Static obs. | Low–Med |
| **R16** | GCP service-account private key in a `secrets/` path; public-domain licence on unpublished working copies; identical boilerplate `CLAUDE.md` in all three repos with no project content | **Verified** | Low |

**Withdrawn from v1:** the "Critical" rating on R1 (provenance-dependent), the "Critical" rating on R4 (v1's own reasoning was invalid), the "High" rating on the signing-certificate finding (§6.5), and the legal conclusion that the Terms were unenforceable (§6.4).

---

# PART 9 — RECOMMENDATIONS

Ordered by value. Each states what evidence would close it.

## 9.1 Before any external sale of a UNO licence

1. **Publish the contract addresses, or stop describing the licence as an NFT.** (R1) *Closes with: one published address, or a statement that licences are off-chain records.* This remains the highest-value item — not because tokenisation is absent, but because the question is currently unanswerable by any counterparty.
2. **Reconcile the four complementary-allocation figures to one number, and state who bears risk if the third-party treasuries withhold.** (R11, R2)
3. **Obtain legal review of the 24-month binary election** against the "not a security" position asserted in the Litepaper (pp.2, 11). (R2)
4. **Establish the per-licence reward basis** — gross or operator allocation — and publish per-task rates, volumes and utilisation assumptions. (R9) An honest low number serves the business better than an uninterpretable high one.

## 9.2 Privacy and disclosure

5. **Reconcile the published privacy claim with the declared capability set.** Either narrow the claim to what is demonstrably true, or explain the camera, location and attestation surfaces. (R3) *Closes with: an in-app disclosure at the point of each permission request, plus an updated FAQ answer.*
6. **Commission a data-flow review of the four sensitive permissions** — camera, background location, biometrics, overlay. (R3) This document establishes only what is *declared*; the review must establish what is requested, granted, executed, transmitted and retained.
7. **Reconcile ToS §2.2 and the FAQ with the Learn guide on the KYC vendor.** (R10)

## 9.3 Distribution

8. **Do not change the certificate subject for cosmetic reasons.** (R6.5) Determine and document signing-key provenance, continuity and protection, and publish the SHA-256 fingerprint in the install guide so users can verify the build they receive.
9. **Establish whether expo-updates is using client-verified cryptographically signed updates before changing the update model.** (§6.5) Remove OTA only if that protection is absent — the opposite of v1's recommendation.
10. **Make Google Play the default install path**, demoting sideloading to a documented fallback.

## 9.4 ULO economics and operations

11. **State the enforcement mechanism for the uptime condition**, and replace the FAQ's "no penalties" line with whatever is actually true. (R6) Also confirm whether the 99% and 95% figures are template examples or operational requirements.
12. **Publish the actual lease split and the withdrawal threshold as fixed, or state plainly that both are governance-controlled.** (R7) The April guide already makes the threshold discretionary; the FAQ implies it is not.
13. **Clarify whether credits exhaustion pauses operation or expires the licence.** The April guide's wording conflates the two and additionally mislabels a ULO licence as a "Node license."
14. **Document the agent role**, if it exists in production, in the Terms. (R5) *First: determine whether it does.*

## 9.5 Geography

15. **Version the ToS §2.4 list with a public changelog**, and remove the UAE / China / Oman from the coverage chart or mark them as observation-only. (R12)
16. **Reconcile the FAQ's node-ownership-in-VoIP-restricted-countries allowance with the ToS location bar.** (R12)

## 9.6 Housekeeping

17. **Remove the GCP test key from `secrets/`** and confirm the test project is decommissioned. (R16)
18. **Populate the three `CLAUDE.md` files** with project content and create the engineering log they reference. All three are currently identical boilerplate titled "ZK Proof System Research."
19. **If this codebase is the platform's, establish that**: initialise version control with a corporate remote, add deployment linkage documentation, and record how these services relate to the shipping application. *Until then, no statement about the platform should cite it.*

---

# APPENDIX A — EVIDENCE BASE

**Public documents**

| Document | Location |
|---|---|
| Home / Business / FAQ / Terms / Learn | `unitynodes.io` |
| Unetwork Litepaper 2026 V1 | `storage.googleapis.com/unetwork-io/UNETWORK_LITEPAPER_2026_V1.pdf` |
| ULO Handbook 2026 | 25 pp |
| Unetwork Application User Guide, Apr 2026 | 34 pp |
| Unetwork Token Whitepaper V1-1 | 30 pp — marked **REVIEW DRAFT** |

**FAQ retrieval.** The accordion content is not exposed by text extraction. The full dataset was retrieved from the site's CMS endpoint: `https://unetwork-io-969417913506.europe-west4.run.app/api/pages?limit=100&depth=0`, page ID 4. The served HTML does contain the Q&A payload inline; the CMS route was used for reliable field-level extraction. *This unauthenticated internal API is itself worth noting.*

**Artefact**

```
https://releases.unetwork.io/android/Unetwork_App-1.2.4.apk
SHA-256  06205fcd90f9859717d769c3c552ce38be5224eb028c35a5c3fb47113428e33c
Tools: aapt2 (build-tools 36.1.0), apksigner, unzip, strings
```

**Local codebase — provenance unverified (Part 7)**

```
/Users/admin/Documents/app-stack/u-network/uno-api      no VCS
/Users/admin/Documents/app-stack/u-network/uno-admin   7 commits, no remote, 111 uncommitted
/Users/admin/Documents/app-stack/u-network/uno-app      no VCS
```

Searches performed: whole-tree `.sol` (0 files), whole-tree Ethereum crates in `Cargo.toml` (0), 32-byte identifier distribution across 182 samples, `infra/` `migrations/` `scripts/` `docs/` inspection, `secrets/` inspection.

## Appendix B — What changed from v1

**Withdrawn conclusions**

| v1 claim | Why withdrawn |
|---|---|
| "There is no on-chain contract" | Provenance unverified; searched paths only. Now Unresolved (R1) |
| UUID mapping proves licences are not on-chain | Normal internal keying; does not disprove token backing |
| Sequential-counter theory (added during review, then tested) | 182 IDs span 99.0% of the 32-bit space — consistent with randomness, not a counter. Rejected |
| Agent fixture proves an undisclosed party takes ULO revenue | Test data, not production policy |
| "The app collects background location / facial imagery" | Manifest declares; it does not establish request, grant, execution or transmission |
| "Alchemy endpoints confirm operational WMChain recording" | Bundled URL proves neither call nor write |
| `test-api.sumsub.com` as production KYC evidence | Test endpoint. Conclusion now rests on the Learn guide |
| Four **active** payment systems | Four SDKs bundled; active flows not established |
| Signing certificate DN empty = High risk; set `CN=Unity Network Limited` | DN fields are cosmetic. Security-theatre recommendation withdrawn |
| Public APK index, retained builds, skipped version, size drop framed as risk | Ordinary practice; no basis for concern |
| Remove OTA updates | Expo supports signed client-verified updates; status unverified |
| "A ULO holds exactly one licence" | Contradicted v1's own §5.5. Restriction is per **device** |
| All three acquisition routes require a "non-leased licence" | Self-contradictory; qualifier attaches to secondary-market purchase only |
| ULOs receive "no UNT whatsoever" | Draft permits overlapping eligibility categories |
| 30/70 and 99% are universal defaults | Template values, editable per licence |
| 99% uptime contradicts "no penalties" | Different mechanisms; enforcement unspecified |
| Credits exhaustion expires the licence | Conflates pausing operation with licence expiry |
| "Everywhere else is nominally permitted" | Non-exhaustive prohibition cannot yield an allowlist |
| FAQ answers absent from served HTML | False — 97 Q&A pairs inline. My extraction error |
| FAQ has no answers on expiry / bank withdrawal | **False.** Answers are "Rewards do not expire" and "Rewards are paid in cryptocurrency" |
| $1.38B / $346M extrapolation | Accounting base unestablished. Gross vs operator allocation changes the figures |
| Terms "not enforceable" against bundled processors | Legal conclusion beyond evidence and competence |
| "The operator's own repository" | Provenance checked and negative |

**Corrections to v1's own text:** one licence per *device* (not per account); acquisition routes separated; ULO token entitlement restated; uptime defaults qualified; credit-expiry claim withdrawn; country allowlist withdrawn; signing/distribution recommendations replaced; `uno-admin/secrets/` characterised accurately as a GCP test key.

## Appendix C — Not verified

- All business metrics are self-reported; none independently confirmed.
- **No on-chain verification was possible** (R1).
- No runtime flow exercised: no account, purchase, KYC, task, reward or withdrawal. **Actual data collection is therefore unknown** (R3).
- Payment SDK active flows and contracting merchants not established (R8).
- Whether expo-updates uses client-verified signed updates not established (§6.5).
- WMChain usage not established — no transaction hashes (§1.3).
- Production licence share defaults and agent role not established (R5).
- Local codebase provenance and deployment linkage not established (Part 7).
- The three Apple App Store IDs referenced in the app bundle were not confirmed.
- Whether the five published APKs share a single signing key was not tested.

---

*Prepared from public materials, the shipping Android artefact, and an unverified local codebase. Every finding carries an explicit evidence status; where status is Unresolved, treat the item as a question rather than a conclusion. Of the sixteen items in the register, one is Critical, two are High, and the remaining thirteen are Medium or below — a materially different picture from v1, and the more useful one.*
