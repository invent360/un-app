# Unetwork / Unity Nodes: consolidated operator, commercial and technical assessment

**Research date: 28 September 2026**  
**Revision:** 2 — consolidated after third-party review and reciprocal fact-checking.  
**Scope:** business model, UNO and ULO roles, geographic availability, participation requirements, commercial dependencies and technical evidence gaps.

## Executive assessment

Unetwork presents a commercial network of participating smartphones that supplies real-world testing and measurement to telecommunications and enterprise customers. Its central proposition is that a physical handset can reveal what actually happens at a destination, beyond what upstream infrastructure reports. [1, 8, 9]

The central distinction is **licence management (UNO) versus device operation (ULO)**. [4]

The country question has an important limitation: **the reviewed public material does not provide a complete, authoritative positive list of countries currently accepting ULOs and supporting every task.** It supplies an observed footprint, named exclusions, and conflicting statements about restricted-country participation. This report preserves those distinctions rather than presenting an inferred list as confirmed availability.

This is a documentary assessment, not an audit of software, corporate records, customer contracts, earnings, or actual account acceptance. Company statements are attributed as such. The duplicated litepaper link supplied for this review was treated as one document.

### Evidence standard for this revision

This revision incorporates the important points from the supplied third-party report, *results.md*, and subsequent feedback. It does not adopt disputed conclusions as facts. The APK examination and private source inspection were reported by that reviewer and have **not been reproduced here**.

| Label | Meaning |
|---|---|
| Documented | Directly observed in a published source; does not establish implementation or legal enforceability. |
| Independently checked | A calculation, document passage or platform rule checked during this review. |
| Third-party observation | Reported in the supplied audit; underlying artefact or code not independently inspected here. |
| Conditional analysis | Conclusion that holds only if its stated assumptions are established. |
| Unresolved | Evidence is insufficient or sources conflict. |

Technical depth and evidential certainty are separate. Static analysis can identify capabilities, dependencies and investigation targets without demonstrating production behaviour. Risk impact is likewise separate from confidence that a condition exists.

## 1. What they do

### Services and customers

| Service | What the company describes | Practical interpretation |
|---|---|---|
| Real-device quality assurance | Customer journeys executed on physical phones, with step results, screenshots, logs and timings. [8] | Check whether registration, authentication or another workflow succeeds under actual market conditions. |
| Network intelligence | Distributed observations of connectivity, carrier conditions, coverage, routing, performance and anomalies. [9] | Give analytics teams visibility from the end-user side of a network. |
| Entropy contribution | Randomness contributions from supported physical-device sensors and system sources, accompanied by provenance information. [10] | Supply an additional randomness input to workloads that need evidence of physical origin. |
| Telecom assurance | Destination-side verification of voice, caller identification, SMS and OTP delivery. [3] | Compare what a provider intended to deliver with what a handset received. |
| Surveys | Additional survey workloads are advertised. [3] | Participation can involve human responses as well as device measurements. |

For example, a QA customer could specify a device model, operating system, carrier and registration flow. The company describes dispatching the test to a matching physical phone and returning evidence for each step. This is a testing service; a sample successful run is not evidence that every requested market/device combination is continuously available. [8]

Network intelligence has a different purpose. Individual timestamped connection observations can be aggregated across geography and time to identify patterns. The advertised output includes carrier context, performance indicators and anomaly signals. Its value depends on sampling quality, device diversity and representativeness—not simply the number of observations collected. [9; analysis]

### Technical and commercial structure

The litepaper describes Switch Nodes routing and timestamping traffic, Validation Nodes checking integrity, and Earth Nodes aggregating and exposing results. It says activity metadata is hashed to WMChain. These are infrastructure functions; a participant buying the marketed node licence should not automatically assume they are personally running a blockchain consensus validator. [2; analysis]

The published fee model assigns **75% of generated service fees to the operator side**, with a negotiated UNO share deducted when a ULO leases its licence. The remaining 25% supports the MNTx/WMTx ecosystem. [2]

**Illustration, not a payout forecast:** if attributable service fees were $100 and the UNO received 30% of the operator allocation, that would mean $25 for the ecosystem, $22.50 for the UNO and $52.50 for the ULO, before costs. This combines the published allocation model with a hypothetical lease split; actual task accounting needs confirmation.

Hashing a measurement can make later alteration detectable. It does not, by itself, establish that the original measurement was truthful, correctly located or commercially valuable. Those properties depend on collection, attestation, validation and fraud controls. Similarly, physical provenance alone does not establish cryptographic randomness quality: entropy estimation, conditioning, health testing and adversarial resistance would require separate technical evidence. [Analysis]

## 2. What UNOs are

**UNO means Unetwork Node Operator.** [4]

The homepage markets a Node Software Licence at **US$10,000**, with **200 Operator Software Licences**, and a node supply limit of **6,000**. **Unetwork describes these licences as NFTs; this review has not independently verified their contract addresses or on-chain implementation.** It also advertises third-party token allocations with a 24-month lock, but different homepage sections display different quantities. A buyer should therefore obtain the actual purchase-specific allocation rather than relying on a marketing example. [1]

The user guide shows a UNO configuring the licence reward split, lease duration, minimum uptime and marketplace visibility, then reviewing those settings. It separately shows activating and binding an operator licence using credits. [7]

Operationally, a UNO must turn licence capacity into useful participation: recruit or arrange operators, agree viable terms, and monitor whether deployed licences remain active. **200 licences are capacity, not 200 guaranteed productive devices or revenue streams.** This follows from the distinction between licence ownership and performed work. [Analysis]

The published terms add a material ownership limitation: withdrawing complementary allocations after the lock period ends causes permanent node cessation and licence forfeiture. They also describe the purchase as non-refundable. These are the company’s contractual statements, not a conclusion here about enforceability in a particular jurisdiction. [5]

The intended complementary allocations are **$1,875 in each of MNTx and WMTx**, subject to third-party discretion. The terms disclaim company liability for non-provision, although operation depends on a valid allocation. [5, §§5.2, 5.7]

**Commercial implication:** payment, allocation delivery and operational access are separate dependencies. Before purchasing, obtain purchase-specific quantities, valuation timing, delivery obligations, custody arrangements and remedies. Token-price-linked quantity differences are not inherently contradictory; unexplained valuation dates and overlapping examples make comparison difficult. [Analysis]

## 3. What ULOs are

**ULO means Unetwork License Operator:** the person running an Operator Software Licence through the smartphone app and contributing verified work. The handbook supports obtaining a lease through the marketplace or redeeming a private code supplied by a UNO. This is a route into participation without purchasing the full node package. [6]

| Question | UNO | ULO |
|---|---|---|
| Core role | Manages node/licence capacity | Operates participating devices |
| Main practical responsibility | Licence allocation and lease terms | Connectivity, task participation and eligible results |
| Main commercial exposure | Capital committed to node capacity | Device, connectivity, time and participation costs |
| Revenue dependency | Productive use of associated licences | Accepted work and applicable lease share |

*The responsibility and exposure rows are analysis of the documented model, not additional contractual promises.*

A single ULO account can cover multiple licences/devices. KYC belongs to the account; attestation belongs to each device. To operate licences owned by their own UNO account, a person needs a second account with a different email and its own verification. The guide currently allows two verified accounts per person. [4]

A useful economic test is:

**Net ULO benefit = accepted incentives after lease sharing − credits paid personally − incremental connectivity/power/device costs − value of time spent.**

Assess this using actual observations from the intended device and location. An attractive percentage split is of limited value if eligible work is scarce. [Analysis]

## 4. Where ULOs operate

### Observed footprint

The business page reports approximately **58,000 30-day active devices across 185 countries**, with an August 2026 context. Its published country chart names the following 20 countries; Ghana also appears in its network sample. These are company-reported observations, not an independently verified admission whitelist. [3]

| Region | Countries explicitly evidenced in those coverage/sample sections |
|---|---|
| Africa | Ghana, Kenya, Nigeria, South Africa |
| Asia | Bangladesh, India, Indonesia, Philippines, Vietnam |
| Europe | Croatia, France, Germany, Greece, Netherlands, Romania, Spain, Switzerland, United Kingdom |
| North America | Canada, United States |
| Oceania | Australia |

Brazil also appears in an illustrative QA run, but it is not added to the observed-country table on that basis alone. [8]

**Coverage-depth correction:** the 20 chart values sum arithmetically to 50,808. However, the page warns against summing country cohorts to infer unique global devices. Subtracting that sum from 58,000 therefore does not establish 7,192 devices in other countries or an average of 44. Device counts also cannot establish a “device every 25 minutes” rate. Ghana is separately supported by the Scancom (GH) sample. [3; independently checked]

Meaningful operational coverage requires concurrent availability, carrier/device diversity, eligible capacity, completion rates and response times by market. Country presence alone cannot answer whether a requested test can run reliably. [Analysis]

### Every explicitly named restricted jurisdiction

The terms exclude users located in these named jurisdictions. This is **Unetwork’s policy list**, not an independently verified description of current sanctions or telecom law. [5]

| Policy category | Explicitly named jurisdictions |
|---|---|
| VoIP restrictions | China, Oman, United Arab Emirates |
| Sanctions-related exclusions | Afghanistan, Belarus, Cuba, Iran, Myanmar, North Korea, Russia, South Sudan, Sudan, Syria, Venezuela, Yemen, Zimbabwe |
| Regional exclusions | Crimea, Donetsk and Luhansk regions of Ukraine |

### Why a complete “available countries” list cannot be certified

The FAQ broadly describes worldwide operation subject to VoIP/sanctions exclusions. Elsewhere it allows node ownership in VoIP-restricted countries and suggests travellers can still earn from some services there. This conflicts with the terms’ broader location-based exclusion. [11, 5]

There are four separate questions:

1. Has a device been observed in that country?
2. Can a resident register and complete verification?
3. May the device perform the intended task there?
4. Can that account redeem the resulting rewards?

An affirmative answer to one does not establish the others. Subtracting the named exclusions from a world-country list would also be unreliable because the exclusion language is non-exhaustive and task availability can differ. [Analysis]

**For a country absent from the table, status remains unconfirmed—not necessarily unavailable.** Obtain written confirmation covering residence, operating location, task category and withdrawal eligibility. For a named restricted jurisdiction, the permissive FAQ wording is insufficient to establish reliable access.

## 5. Fundamental requirements to become a ULO

### Device, account and operating prerequisites

| Requirement | Finding |
|---|---|
| Age | The platform terms require users to be at least 18 with legal capacity. [5] |
| Physical phone | A supported Android or iOS smartphone is the handbook’s baseline. [6] |
| Device/licence mapping | One active operator licence per phone; one ULO account can manage several devices. Managing node licences is a different operation. [11, 4] |
| Internet | Stable connectivity; Wi-Fi works, while some tasks require mobile data. [6] |
| Account access | Email or Web3-wallet sign-in is supported. [6] |
| Licence | Claim an available lease or redeem a private lease code, then activate/bind it. [6, 7] |
| Credits | Each operator licence needs its own plan; either party can fund it. The FAQ discusses $1.99–$3.99 plans. Confirm the current offer. [11] |
| Network origin | The terms prohibit VPNs, proxies, VPS/datacenter access and automated infrastructure for task access. [5] |
| Availability | Meet the selected lease’s duration and uptime conditions. [7] |
| Task eligibility | Follow the actual task’s verification, device and participation conditions. [4] |

There is no substantiated universal minimum RAM, storage, CPU or OS version in the requirements material reviewed. “Most smartphones” is not a compatibility guarantee. Check the current app listing and test the actual handset before buying additional devices.

A SIM is not established as a universal prerequisite: Wi-Fi-only operation is documented. However, it is reasonable to expect destination-carrier calls or SMS tasks to require suitable cellular capability. Confirm this per task rather than treating Wi-Fi eligibility as access to every workload. [6; analysis]

### KYC and attestation

The current account guide says individual KYC uses **Sumsub**, becoming required at **5 UP**, at withdrawal, or earlier for specified tasks. Fully completed earlier ShuftiPro verification remains valid; purchase-only KYC does not automatically establish current UNO verification. [4]

| Device state | Connect | Extended Telemetry / Entropy | CLI / Surveys |
|---|---|---|---|
| Hardware attested | Yes | Meets attestation condition | No attestation restriction |
| Software attested | Yes | Meets attestation condition | No attestation restriction |
| Unattested | Yes | Ineligible | No attestation restriction |

Other task conditions still apply. [4]

### Uptime, activation and withdrawals

The April guide displays a configurable template with **UNO 30% / ULO 70%, six months and 99% uptime**. It also displays a **5 UP withdrawal minimum**, and says claim thresholds and payout schedules depend on network load and governance policies. These are documented screen/guidance values, not independently verified live defaults. [7]

At 99% uptime, a 30-day measurement period allows **7.2 hours offline**. The relevant denominator, grace period and enforcement remain unresolved. A lease target, lost rewards, a monetary penalty and revocation are different outcomes; their coexistence with general “no penalties” wording is an ambiguity to resolve, not proof of an enforced penalty. [Analysis]

Credit exhaustion, lease expiry, revocation and NFT ownership are also distinct states. The reviewed wording does not establish that exhausting credits automatically destroys or revokes ownership. Ask what pauses, what can be restored by topping up and whether pending rewards remain redeemable.

For withdrawals, document the live minimum, conversion basis, chain/network fees, payout timing and identity restrictions. Two guides agreeing makes the FAQ an outlier; it does not replace testing the actual redemption conditions.

### Practical onboarding sequence

1. Confirm your country and intended tasks are accepted.
2. Install the official app on the actual phone you intend to use.
3. Sign in and obtain a licence with an acceptable lease split and uptime commitment.
4. Establish who funds credits, then activate the licence.
5. Complete the account verification prompted by the current system.
6. Check device attestation and opt into eligible tasks.
7. Observe accepted work, incentive accrual and real operating cost before scaling.
8. Verify a withdrawal using the correct supported asset/network and destination.

*This is a recommended sequence synthesized from the guides, not a guarantee of acceptance.*

## 6. Conflicts, limitations and questions to resolve

| Issue | Assessment |
|---|---|
| Country access | FAQ exceptions and contractual exclusions need reconciliation before relying on restricted-country operation. |
| Withdrawal minimum | FAQ says there is no minimum beyond network fees; the handbook states $5 and shows 1 UP = $1. The April guide corroborates 5 UP but reserves variability. [11, 6, 7] |
| KYC provider | The current account guide names Sumsub; terms §2.2 and purchase-oriented FAQ wording still name ShuftiPro. Reconcile the documentation and distinguish purchase verification from current account verification. [4, 5, 11] |
| Privacy | Blanket FAQ claims of no identifying-data collection do not adequately explain how identity verification is handled. Request the account/KYC-versus-telemetry data boundary, retention and processor details. [11; analysis] |
| Future earnings | FAQ language suggesting rewards only grow should not be used as a forecast. Homepage calculator outputs are explicitly illustrative. [11, 1] |
| Decentralization | Geographically distributed phones do not establish decentralized control of admission, scheduling, validation or rewards. Those control boundaries need separate evidence. [Analysis] |
| Commercial validation | A service catalogue and reported device footprint do not establish audited revenue, customer retention, profitability or typical ULO net income. [Analysis] |

For a prospective ULO, the most useful unresolved questions are: Which tasks will this specific phone receive? What disqualifies results? What costs remain after the UNO share? How can a lease be revoked? What happens to pending incentives? Which withdrawal terms actually apply?

For a prospective UNO, additionally obtain the purchase-specific token-allocation terms, deployment statistics, inactive-licence rules and evidence of customer-funded work. The financial difference between unused licence capacity and a productive operator fleet is substantial.

**Overall assessment:** the documented model is understandable and has a plausible operational purpose. The public evidence supports describing its services and onboarding model, but it does not justify guaranteed earnings, universal device suitability, or a certified all-country availability list. A small, measured ULO trial can answer operational questions that marketing material cannot; a full node purchase requires a separate commercial assessment.

## 7. Consolidated economics and draft UNT model

### Two-way and reported three-way allocations

The third-party report cites a CSV fixture with `uno_share=47%`, `agent_share=3%` and `ulo_share=50%`, plus references to these fields in dashboard/reward code. This is **third-party evidence of a model**, not verified evidence of a production deduction. [13]

Assuming these percentages divide the same 75% operator allocation:

| Recipient | $100 fees: hypothetical 30/70 | $100 fees: reported 47/3/50 |
|---|---:|---:|
| Ecosystem | $25.00 | $25.00 |
| UNO | $22.50 | $35.25 |
| Agent | — | $2.25 |
| ULO | $52.50 | $37.50 |
| Total | $100.00 | $100.00 |

Both calculations are correct for their respective assumptions. Comparing $52.50 with $37.50 does not prove a 40% overstatement: they describe different agreements. Nor does the fixture establish whose entitlement funds the agent. A 50/50 arrangement could become 47/3/50 entirely by reallocating part of the UNO share.

Resolve whether the agent is a platform role, an independent UNO’s business arrangement or a test-only field; whether it appears in the accepted lease; and whether the displayed ULO percentage is before or after every deduction. A real production intermediary deserves explicit disclosure, but its existence and funding must first be established.

For a general model, let `F` be attributable fees and `u`, `a`, `l` be fractions of the operator allocation, with `u + a + l = 1`. Then:

- Ecosystem allocation: `0.25 × F`.
- UNO allocation: `0.75 × F × u`.
- Agent allocation: `0.75 × F × a`.
- ULO gross allocation: `0.75 × F × l`.

Actual task valuation and settlement may require further terms; this is an accounting illustration, not a verified implementation.

### Reward-calculator stress test

The homepage displays illustrative outputs equivalent to **$96 per active licence per month**. The arithmetic extrapolation to 1.2 million licences is **$115.2 million monthly / $1.3824 billion annually**. This extrapolation is not demand evidence or a forecast. [1; calculation]

| Meaning assigned to the $96 output | Implied annual gross service fees | Implied annual ecosystem allocation |
|---|---:|---:|
| Gross fees before the 75/25 allocation | $1.3824 billion | $345.6 million |
| Entire 75% operator allocation | $1.8432 billion | $460.8 million |
| UNO-only proceeds after lease sharing | Cannot determine without the applicable share | Cannot determine |

The original third-party calculation mixed possible bases by automatically taking another 25% of the displayed reward total. Establish the output’s accounting meaning first. Then request task prices, paid customer demand, utilisation, rejection rates, allocation rules and a distribution of realised earnings. A disclaimer does not supply these inputs.

At the device level, if an operator personally pays $1.99–$3.99 monthly and retains 52.5% of attributable fees, credit-only break-even requires approximately **$3.79–$7.60** of such fees. At 37.5%, it becomes **$5.31–$10.64**. These conditional calculations exclude labour, connectivity, power, hardware, taxes and withdrawal costs; a UNO-funded plan changes the operator’s cost basis.

### UNT: proposed additional economics, separate from UP

The March 2026 **review draft** describes a billion-token UNT supply on Base with future WMChain expansion. It proposes 38.5% for Round 1 UNOs, 24.5% for Round 2 UNOs, 22.5% for Earth/Switch/Validation operators, 4.5% for MNTx/WMTx holders and 10% treasury. UNO unlock schedules differ: 12 versus 60 months. It proposes collateral-linked weighting and node-holder governance, with team-originated proposals. [12]

The draft keeps UP separate and makes TGE conditional on selling the final node. Its cover’s deployment wording does not independently establish deployment: later text says tokens will not exist until that condition. These are proposed, conditional arrangements, not verified present entitlements. [12]

There is **no dedicated allocation for ULO work in the listed categories**. That does not mean a ULO can never receive UNT: someone may separately qualify through holdings or another role, and the draft permits overlapping eligibility. [12]

Evaluate current task compensation independently of speculative token benefits. Obtain final terms, deployment evidence, eligibility snapshots, governance controls and allocation verification before relying on this proposal.

## 8. Technical findings: preserve observations, limit conclusions

### Reported artefact and source scope

The third-party report identifies `Unetwork_App-1.2.4.apk`, package `io.unetwork.app`, version code 53, and SHA-256:

`06205fcd90f9859717d769c3c552ce38be5224eb028c35a5c3fb47113428e33c`

It reports a React Native/Expo/Hermes application, target SDK 36, multiple ABIs, 41 manifest permissions, OTA support and Crashlytics. These particulars are retained as **unreproduced audit metadata**, not independently certified facts. Target SDK alone would not establish the minimum supported Android version. [13]

The cited private directories are `/Users/admin/Documents/app-stack/u-network/uno-api`, `uno-admin` and `uno-app`. Their names and local locations do not establish ownership by Unity Network Limited or linkage to its production systems. Request repository provenance, commit hashes, complete scope, build/deployment linkage and the reviewer’s commands/results before treating them as authoritative architecture evidence.

### Findings and required proof

| Reported observation | What it could establish | What remains unproven / next check |
|---|---|---|
| No Solidity, ERC-721 references or usual Ethereum Rust clients in those repositories | Absence within the inspected scope, if the search is reproduced | No contracts anywhere. Obtain chain IDs, official contract addresses, verified bytecode/source, token ownership and deployment history. |
| Licence identifiers mapped from hexadecimal strings to UUIDs | An internal identifier mapping | Whether identifiers originate on-chain or map to NFTs. Trace generation and authoritative registry synchronisation. |
| CSV/database licence records | Off-chain operational storage | That the database is the sole ownership authority. Trace minting, transfers, revocation and settlement. |
| Agent share fields and fixture | Support for an extra allocation in that code | Production use, funding source and disclosure. Trace a real agreement and corresponding payout. |
| Alchemy/WMChain endpoint strings | A possible integration | Actual evidence anchoring. Match a task payload to a transaction commitment and independently recompute it. |
| Play Integrity dependencies | Potential device-integrity integration | Server-side verification, nonce/replay protection and mapping to the platform’s attestation tiers. |
| Supabase references and multiple service domains | Potential hosting and service boundaries | Ownership, active production use, administrator powers or full control-plane topology. |

Absence of verified contract evidence is a due-diligence gap. It must not be rewritten as proof that licences are not NFTs. Conversely, marketing terminology alone does not establish blockchain-enforced supply or economic rules.

### Privacy and permissions

The reviewer reports camera, background-location, overlay, system-settings, biometric and battery-optimisation capabilities, plus face-detection/barcode libraries. It reports the absence of `READ_SMS`, `READ_CONTACTS`, `READ_PHONE_STATE`, `CALL_PHONE` and `RECORD_AUDIO`. These observations require reproduction against the identified binary. [13]

Android distinguishes manifest declarations from runtime permission requests and grants. A declaration does not establish continuous access, collection, transmission or retention. A bundled model does not establish that KYC invokes it. Biometric authentication capability does not establish access to raw biometric templates. [14; analysis]

The narrower absence of a contacts permission would not prove that no contact information can ever enter through user input or another mediated flow. Likewise, an unused library may explain a manifest entry without proving the capability is necessary. Review the complete data path:

1. Record the installed build, OS, consent state and granted permissions.
2. Exercise onboarding, KYC and each task separately with authorised test data.
3. Observe permission prompts, actual sensor/API access and network destinations.
4. Identify payload fields, processors, purposes, retention and deletion behaviour.
5. Compare those observations with in-app notices and published privacy statements.

The broad privacy wording deserves clarification, but **undisclosed facial-image or continuous-location collection is not established by the evidence reproduced here**. The third-party report’s absence-of-advertising-SDK observation is also scope-limited; it cannot certify that no tracking occurs.

### Signing, distribution and updates

The reviewer reports sparse certificate subject fields and a signing-certificate SHA-256 of `cbf7602663536620a4cb612cf31ba167dcf1d0738b0ccbe8534c21d2efd69ab5`. It also reports historical APK availability, a skipped version and a smaller latest binary. [13]

These are not, by themselves, security defects. Human-readable certificate fields do not prove corporate identity; naming the certificate would not solve provenance. Android signing and update continuity depend on cryptographic certificates and key management. Check signatures, trusted published fingerprints, release-to-release continuity and legitimate key rotation. [15]

Public download access is distinct from cryptographic authentication. Adding login does not repair compromised signing or update keys. Historical availability and size changes require context, not adverse assumptions. OTA should be assessed for signing, release-authorisation controls, channel configuration and rollback protection; Expo supports client-verified signed updates. [16]

### Service providers, payment flows and control

The reviewer reports domains associated with Unity/Unetwork, `unityedge.io`, `uedge.io`, Scout & Runner, Sumsub, Stripe, Paddle, RevenueCat, Google, WalletConnect/Reown, Alchemy and other services. Treat this as a candidate integration inventory. Embedded domains may be unused, test-only or inherited from dependencies; their presence alone does not prove data disclosure or active commercial relationships. [13]

RevenueCat is not automatically a merchant of record. Its documentation distinguishes RevenueCat Billing using Stripe as gateway from Paddle Billing with Paddle as merchant of record. Multiple SDK names may describe layers of one transaction path rather than four independent payment systems. [17]

Trace the actual checkout, contracting seller, receipt, product type and refund terms. The node-purchase restrictions must not automatically be assumed to describe every recurring-credit transaction. Contractual enforceability remains jurisdiction- and transaction-dependent.

For decentralisation, document who can admit devices, select tasks, reject results, change rates, revoke licences, authorise withdrawals and upgrade contracts. Distributed endpoints can coexist with centrally administered services. Domain counts and an unverified source tree do not establish the whole control model.

### Lower-priority repository observations

The reported `secrets/` directory warrants an authorised secret scan and history review, not an assumption that live credentials are exposed. Boilerplate `CLAUDE.md` files and a missing engineering log are maintainability leads, not evidence of network fraud or broken security. A privacy archive may improve transparency; its existence alone does not prove problematic policy churn. An unauthenticated CMS route serving public content is not inherently an access-control defect. [13; analysis]

## 9. Prioritised evidence and remediation register

Impact describes the consequence if the concern is substantiated, not a confirmed defect severity.

| Priority | Concern | Evidence status | Potential impact | Resolution |
|---|---|---|---|---|
| 1 | Licence backing, supply controls and economic enforcement | Unresolved | High for UNO purchasers | Official contracts, ownership verification, admin powers and implementation of caps/locks |
| 1 | Allocation-dependent operation and forfeiture | Documented terms | High | Purchase-specific obligations, custody and remedies |
| 1 | Actual ULO net compensation and deductions | Published model plus unverified agent fixture | High | Accepted lease, all deductions, validated task rates and payout reconciliation |
| 1 | Geographic admission/task/redemption eligibility | Conflicting public guidance | High for affected operators | Versioned country/task matrix and written resolution of exceptions |
| 1 | Sensitive data use and consent | Documentation concern; unreproduced static observations | Potentially high | Runtime data-flow and privacy review |
| 2 | Uptime enforcement and revocation | Documented template; enforcement unresolved | Medium to high | Measurement window, grace rules and consequences |
| 2 | Signing and OTA trust | Unreproduced observations | Potentially high if controls fail | Signature continuity, update signing and release-authorisation evidence |
| 2 | KYC and withdrawal guidance | Documented inconsistencies | Medium | One maintained operational specification and live-flow verification |
| 2 | Calculator assumptions and realised utilisation | Unresolved | High for economic decisions | Transparent inputs and realised earnings distributions |
| 2 | UNT eligibility and launch | Explicit draft | Medium to high if relied upon | Final terms and verified TGE/deployment status |
| 3 | Coverage depth, entropy quality and chain anchoring | Claims with incomplete independent evidence | Workload-dependent | Market-specific service levels and technical validation |
| 3 | Source provenance and repository hygiene | Third-party observations | Undetermined | Authenticated source scope and targeted review |

## 10. Verification package and decision gates

An upgraded technical audit should retain artefact hashes, commands, tool versions, raw outputs and dates; tie source findings to commits and deployments; separate test fixtures from production records; and use authorised accounts for runtime checks. No secrets or personal identity documents need to appear in the published evidence package.

| Decision | Minimum evidence to resolve first |
|---|---|
| Start as one ULO | Country acceptance, device/task eligibility, full lease split, credit payer, KYC and withdrawal requirements |
| Add more ULO devices | Measured utilisation, incremental earnings, uptime enforcement, geographic/IP constraints and actual net costs |
| Purchase a UNO | Licence provenance, allocation/forfeiture terms, recruitment/deployment economics and customer-funded revenue evidence |
| Buy enterprise services | Concurrent carrier/device coverage, reproducible outputs, data handling and contractual service levels |
| Rely on UNT benefits | Final eligibility, allocation, governance and deployment documentation |

### Reconciliation of the two reports

**Retained and expanded:** operator roles, country distinctions, economic costs, contract dependencies, uptime examples, inconsistent documentation, calculator assumptions, draft UNT model and the need for technical verification.

**Retained as conditional leads:** APK metadata, permission inventory, service integrations, signing observations, source identifier mappings and agent-share fields.

**Corrected or rejected:** inferred country allowlists; country-cohort subtraction; device-count-to-time conversion; universal 99%/30–70 defaults inferred from screenshots; permissions as proof of collection; missing local Solidity as proof of no NFT; UUID mapping as proof against on-chain backing; a fixture as proof of concealed deductions; RevenueCat automatically acting as merchant of record; and categorical ULO exclusion from every possible UNT eligibility route.

The result is a broader assessment with explicit evidence boundaries. It does not certify the platform’s implementation, profitability, privacy compliance or global availability.

## Sources and method

All sources below were accessed on 28 September 2026. PDF page references use document pages. FAQ category answers were also read from the page’s embedded content because the text-only web extraction exposed only the initial category. No account was created, no purchase made, and no message sent.

1. [Unetwork homepage](https://unitynodes.io/): node offer, credits, rewards and marketing qualifications.
2. [Unetwork Litepaper 2026 V1](https://storage.googleapis.com/unetwork-io/UNETWORK_LITEPAPER_2026_V1.pdf): especially pages 7–12, fee flow and infrastructure.
3. [Business overview and use cases](https://unitynodes.io/business#use-cases): reported footprint, country cohorts and service categories.
4. [Accounts, KYC and device attestation](https://unitynodes.io/learn/accounts-kyc-attestation): account rules and current task eligibility.
5. [Terms of Service](https://unitynodes.io/terms): §§2.1–2.5, 5.1–5.8 and 8.3/8.6 are relevant. Displayed effective date 1 December 2025; earlier purchases have separate terms. Section 5.6 covers rewards/leasing, 8.3 post-purchase refunds and 8.6 chargebacks.
6. [ULO Handbook 2026](https://storage.googleapis.com/unetwork-io/ULO%20UNETWORK%20USER%20GUIDE%202026.pdf): especially pages 4–12 and 16–17. Downloaded and text-extracted; requirements page visually checked.
7. [Unetwork User Guide, April 2026](https://storage.googleapis.com/unetwork-io/UNETWORK%20USER%20GUIDE%20APR%202026.pdf): UNO lease settings and ULO licence activation.
8. [Real-Device QA](https://unitynodes.io/business/real-device-qa).
9. [Network Intelligence](https://unitynodes.io/business/network-intelligence).
10. [Entropy Contribution](https://unitynodes.io/business/entropy-contribution).
11. [FAQ](https://unitynodes.io/faq): including dynamically displayed categories.
12. [UNT Whitepaper V1-1, March 2026 review draft](https://storage.googleapis.com/unetwork-io/Unetwork%20Token%20Whitepaper%20V1-1.pdf): independently read; conditional allocation and TGE proposals.
13. User-supplied *results.md*, “Unetwork — Comprehensive Analysis & Technical Audit,” dated 28 September 2026, and subsequent reviewer feedback supplied in this conversation. Private-source and APK findings are attributed, not independently reproduced. Reported code references include `uno-admin/src/logic/marketplace_service.rs:196–200`, `storage/local_store.rs:174–176`, `handler/dashboard_handler.rs`, `handler/rewards_handler.rs`, and `uno-api/src/models/license.rs`.
14. [Android: request runtime permissions](https://developer.android.com/training/permissions/requesting) and [declare app permissions](https://developer.android.com/training/permissions/declaring).
15. [Android: sign your app](https://developer.android.com/studio/publish/app-signing).
16. [Expo: end-to-end code signing with EAS Update](https://docs.expo.dev/eas-update/code-signing/).
17. [RevenueCat: web payment integrations](https://www.revenuecat.com/docs/web/payment-integrations).

**Version note:** Revision 2 expands the original report in place. It preserves the five requested subject areas and adds evidence classification, technical verification scope, corrected economics and a prioritised resolution plan. The third-party attachment itself has not been modified.
