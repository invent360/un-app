# HighLevel and Plai: vendor due-diligence supplement

**Status:** supplement to `HIGHLEVEL_PLAI_UNO_DISTRIBUTION_TEAM_GUIDE.md` and `UNETWORK_AI_MARKETING_AND_COUNTRY_ACQUISITION_PLAN.md`. **Does not replace either.** Those documents remain the programme design of record; this one supplies harder primary-source verification, contract-level findings, and commercial-channel policy research they do not contain.
**Audience:** business owners, marketing, country agents, operations, engineers
**Research date:** 29 September 2026
**Activation status:** no subscription, campaign, API trial or integration was activated. Everything below is desk research.

---

## 0. Correction to earlier analysis, and what changed

An earlier pass produced a generic per-seat SaaS licence-distribution analysis. **That framing was wrong for this programme and has been withdrawn.** Three substantive corrections:

| Earlier assumption | Actual programme position | Consequence |
|---|---|---|
| UNO sells a per-seat/per-use software licence to consumers at a list price | UNO **subleases licences to Unetwork Local Operators (ULOs)**; a distributable reward pool is split **ULO 50% / UNO 40% / referral 10%**; the UNO funds all activation credits | The entire "consumer checkout → merchant of record → commercial licence API" analysis does not apply. The participants are not card buyers. |
| "Lease code" is a typo for "license code" | **"Lease code" is the programme's own term** for a sublease credential. It is not a typo. | Do not rebrand it as a "licence key" in copy, code or vendor conversations. |
| Participants pay ~$29–$499 | Pool thresholds analysed around **$5.60/month per paid licence**, of which ULO ~$2.80 and referral ~$0.56 | Payout economics, not checkout economics, are the binding constraint. |

**Withdrawn as out of scope:** commercial licence-key services (Keygen, LicenseSpring, Cryptlex, Gumroad, Lemon Squeezy licence APIs), the 5%+50¢ merchant-of-record cost model for consumer licence sales, and the per-seat named-user/device binding question. Entitlement here derives from **the authoritative Unetwork services and the UNO's own ledger**, exactly as the team guide states: *"None of the marketing software establishes the upstream revenue entitlement."* No marketing or CRM vendor can or should issue the lease credential.

**Retained and materially strengthened below:** HighLevel contract terms, the API surface census, Plai commercial terms, and — most importantly — **advertising-policy and affiliate-programme exposure created by a recruitment-linked referral share.**

---

## 1. The three findings that most change the buying decision

### 1.1 HighLevel is never a merchant of record — the UNO bears all tax and dispute liability `[V]`

Not present in the team guide. From the live GoHighLevel Terms of Service (Last Updated **June 2026**), §2:

> *"You acknowledge and agree that the Platform and/or Services **are not a marketplace**, and any contract of sale made through the Platform and/or Services is **directly between You and the customer**. **You are the seller of record for all items you sell through the Platform and/or Services.** You are responsible for … authorizing the charge to the customer in respect of the customer's purchase, refunds, returns, fulfilling any sales or customer service, **fraudulent transactions**, required legal disclosures…"*

> *"**You are solely responsible for determining, calculating, collecting, reporting, and remitting all taxes applicable to your business operations and customer transactions.** HighLevel provides technology tools only and is not engaged in providing tax, accounting, or professional services advice."*

> *"HighLevel is not responsible for your compliance with laws and … **tax laws, sales tax regulations, VAT/GST requirements**…"*

> *"**You are exclusively responsible for Taxes associated with your use of the Platform, including all Taxes associated with transactions you conduct with your customers.**"*

Corroborated architecturally: HighLevel's **Connect** model requires the client to authorise **their own** Stripe account; supported processors are Stripe, PayPal, NMI, Authorize.net, Square, Adyen, Razorpay. Because the account holder is you, the merchant is you.

**Programme relevance.** This matters less for consumer licence sales than it first appears — the ULO is not a card buyer. It matters for **any** HighLevel Payments use in the programme: agent subscriptions, credits top-ups, pilot budget tooling, or payments collected from a participant. In each case the UNO is the seller of record and carries sales tax, chargebacks and fraudulent-transaction exposure. Note also the ToS reserves the right to add VAT/GST to **HighLevel's own invoice or wallet** — HighLevel taxing you for the platform fee — while saying nothing about HighLevel taxing your customers for their purchase, because it does not.

### 1.2 The two HighLevel subsystems that define the product are write-closed `[V]`

Not present in the team guide. From censusing **all 44 v3 OpenAPI specs — 443 paths, 627 operations**:

| Deepest surfaces | Operations | | Thin surfaces | Operations |
|---|---:|---|---|---:|
| ad-publishing (FB/IG) | 95 | | **workflows** | **1** |
| calendars | 59 | | **courses** | **1** |
| social-planner | 45 | | companies | 1 |
| invoices | 42 | | campaigns | 1 |
| contacts | 31 | | email-isv | 1 |
| locations | 32 | | | |
| products | 27 | | | |
| payments | 23 | | | |

The **entire public Workflows API is one operation:** `GET /workflows/` — "Get Workflow". No create, no update, no trigger CRUD, no action CRUD, no bulk enrollment. The only external workflow write path is `POST /contacts/{contactId}/workflow/{workflowId}` and its DELETE. The Courses/Memberships API is **one operation**: `POST /courses/courses-exporter/public/import`. There is no API to create courses, create offers, enroll, unenroll, or query memberships.

**Programme relevance — this is a material constraint on the team guide's design.** The team guide's §7 lifecycle (12 states, automation per state) and §8 integration blueprint assume workflows and custom objects are the automation substrate. Custom objects *are* properly API-exposed. **Workflows are not.** Consequences:

- Workflows are a **UI-built asset**. They cannot be provisioned by a deployment script, version-controlled, or migrated to another vendor.
- If the UNO ever changes CRM, or if an account is lost or terminated, the automation layer does not export. **Every workflow is a bespoke rebuild.**
- Practical mitigation: treat workflows as configuration to be **documented and snapshotted** alongside the deployment, and put anything durable in **custom objects plus your own service**, not only in workflow logic. The team guide's §8.5 `referral_beneficiary_id` pattern is the right instinct — push it further.

Also verified and worth adding to the team guide's §8: **627 operations and 127 OAuth scopes**; rate limits **100 requests / 10 seconds burst, 200,000 / day**, scoped **per app per resource (Location or Company)** — installing on more sub-accounts does **not** split the budget. Webhook signature is **`x-wh-signature`, RSA/SHA256/base64**, payload carries `timestamp` and `webhookId` for replay defense, return `200` immediately and process asynchronously. Every request returns a `traceId`. **API v3 shipped 11 June 2026** — a short production track record, and the versioning table still says legacy versions are "Supported Until: TBD" while every legacy page is banner-marked "no longer actively maintained" — a live documentation contradiction.

Two operational warnings the team guide should carry:

> ⚠️ **Webhook public-key rotation is announced by email and Developer Council Slack, with no key-id in the payload.** A missed rotation silently breaks verification. Monitor that channel. `[V]`
> ⚠️ **No AI event webhooks exist.** Conversation AI and Voice AI are workflow-triggered instead — so AI interactions cannot be captured by webhook-only ingestion. `[G]`

### 1.3 A recruitment-linked referral share is the programme's largest external-policy exposure `[V]`

The offer includes a **10% recurring referral share of accepted rewards**. That single design choice interacts badly with three separate regimes, none of which the team guide currently documents in text.

**Meta — Prohibited Commercial Practices** bans, verbatim:
> *"Offers of investment opportunities where returns on investment or compensation is **partly or fully based on recruitment of others** to participate in the scheme."*

**Google Ads — Unacceptable business practices.** This is an **egregious** violation: *"Your Google Ads accounts will be suspended upon detection and without prior warning, and you will not be allowed to advertise with Google Ads again."* It prohibits offering *"products or services that you don't have or can't deliver"* and names as an example:
> *"Tricking people into believing you're the official website for a new real estate development **so you can earn commission**."*

That is the closest published analogue to **promoting a sublease in order to earn a commission**, which is exactly this programme. Google also requires: *"Be clear about your partnerships… If you reference another brand but you're not an official or authorized partner, consider a disclaimer."*

**FTC business opportunity advertising.** If an ad claims buyers can make a certain amount, **the law requires disclosing the number and percentage of previous purchasers who earned that income**; without it, *"the business opportunity seller may be violating the law."* Named deception patterns include "WORK PART-TIME FROM HOME", "EARN $2,000 A MONTH" without the disclosure, and "NO RISK! GUARANTEED!". `[LEGAL]`

**The 2,500-plan is already right about the structure.** It commits to a *"single-level 10% recurring share of accepted rewards from directly attributed licence participation… **No downstream commissions and no payment merely for recruiting another referrer.**"* That constraint is the correct mitigation and must be enforced technically, not just documented. **Keep it.**

**⚠️ Do not use HighLevel's own affiliate programme to recruit programme agents.** HighLevel pays **40% on tier 1 plus 5% on tier 2** — *"You will also earn an additional 5% for Your Tier 2 referrals, which your original client brings in."* `[V]` A two-level commission is *"partly or fully based on recruitment"* in the plain sense Meta's policy uses. The HighLevel affiliate policy also imposes: a **live engagement call required after your second referred customer** before further payouts; a **ban on business-in-a-box and income-guarantee representations**; **no sub-affiliate networks** without written agreement; no self-referrals and no 30-day re-affiliation games; and **auditable** — HighLevel may audit your sites, landing pages, ads, social posts and emails, with complaints reportable within 24 hours.

`[I]` Use the UNO's own single-level ledger (already specified in the 2500 plan) as the promoter compensation mechanism. Do not stack HighLevel's affiliate programme on top of it.

**FTC affiliate disclosure is mandatory and prescriptive** `[V]`: "I earn a commission if you buy through my link" is good; "affiliate link" or "commissionable link" is explicitly bad. Must be conspicuous, not in a footer, at the **top** of posts, at the **beginning** of social captions before truncation, and **audibly and as superimposed text** in video — description-only is insufficient. Every post needs its own; platform branded-content labels do **not** substitute.

**Enforcement reality — a live ad is not clearance `[V]`.** Reuters (2025-12-15) reported ads offering a **10% weekly return** (~14,000% annualised), created with help from agencies in Meta's own **Partner Directory** of "trusted experts", **running** and reaching 20,000+ users across the US, Europe, India and Brazil. The Verge (2026-04-30) reported Meta-owned Manus running "Easy side hustle" ads promising *"potential $5k a month"*, amplified by a paid creator network, with UK legal experts saying undisclosed commercial relationships "probably break the law".

`[I]` **Build to the policy text and the statute, not to observed enforcement.** These platforms catch low-quality spam and leave professionally-produced scheme advertising live. Using an agency to disguise a recruitment-linked earning claim does not reduce risk — it increases it and moves the programme into the impersonation and brand-bait prohibitions.

**Safe vs prohibited copy `[I]`:**

| Safe | Prohibited or high-risk |
|---|---|
| "50% of the distributable pool goes to the operator. 40% to the UNO. 10% to the referrer." — **capability/allocation fact** | Any "make money", "passive income", "financial freedom", "work from home", "side hustle", "get rich quick", "no risk", "guaranteed" |
| "No payment for recruiting another referrer." | Any income figure without FTC earnings-claim substantiation and typical-results disclaimer |
| Concrete allocation percentages with a defined pool and its denominator | "Unlimited" earnings, or earnings stated without the observation window |
| Attribution of Unetwork as the platform operator, plus a non-affiliation disclaimer if not an authorised partner | Implying endorsement by Unetwork using logos, "as seen in", or official-site framing |
| Disclosed AI-generated presenters, clearly labelled as not real operators | AI UGC avatars presented as, or resembling, a real earning ULO (see §4.3) |

---

## 2. HighLevel: contract terms the team guide does not yet carry

All `[V]`, from the ToS §2 and §3 and the pricing materials.

### 2.1 MAP policy — read before quoting any price to an agent

| Clause | Effect |
|---|---|
| **Core Platform** | 2 or more of: Funnel/Website Builder, Forms, Surveys, CRM, Email Builder, Calendars, Automation workflows. **Any single feature sold standalone is not subject to MAP.** |
| **Advertised vs Final Price** | MAP applies **only to the Advertised Price**, computed **after** deducting coupons, rebates, giveaway value, gift cards and other promos. *"The final price at which you resell access … is not subject to the MAP Policy."* |
| **Cart exclusion** | Excluded where Final Prices are first disclosed **in the shopping cart** and not retrievable by search engines. |
| **EU/UK** | Discounts may be offered, or Final Price divergence communicated, for sales into the EU and UK. |
| **Anti-circumvention** | Violation to transmit a below-MAP Advertised Price **from** a MAP-free jurisdiction **to** customers where MAP applies. |
| **Pricing can change anytime** | *"HighLevel reserves the right to change its standard pricing at any time, for any reason."* |
| **Special Pricing gives no relief** | HighLevel's own promos create no exception. |
| **Exceptions** | Sole discretion, **must be in writing**, revocable at any time. |

`[I]` **Programme relevance:** if the UNO resells HighLevel to country agents, or advertises an agent-facing plan, MAP governs the advertised price. Public "first month free" pages are exactly what the Advertised Price rule targets — coupons are deducted *before* the comparison. Cart- or funnel-disclosed pricing is the compliant pattern.

**Lifetime licences are prohibited** without prior written approval: *"Subscription fees … must be charged on a recurring basis … It is a violation of these Terms to resell lifetime access to the Core Platform or any standalone features of the Platform for a one-time fee unless you have a prior written approval from HighLevel."*

### 2.2 White label — revocable at will, and support liability stays with the UNO

- *"HighLevel **may remove any of your modifications at any time without advance notice and without liability to you**."* `[V]`
- License is **limited, non-exclusive, non-transferable and revocable**. `[V]`
- You **may not** direct customers to contact HighLevel *"for any reason, including Platform support."* `[V]`
- You **may not** solicit existing HighLevel customers to cancel in order to buy your white-label version. `[V]`
- You **may not** publish marketing that directly compares HighLevel to your version, or claims yours is superior or has more features. `[V]`
- **The reseller is fully liable for customer access, support, disputes and inquiries** — and for third-party pass-through fees including chargebacks. Repeated failure is grounds for termination. `[V]`
- **Termination at HighLevel's sole discretion, with or without notice**; 90-day inactivity auto-deletion; 90-day post-termination retention then permanent deletion. `[V]`
- **All fees non-refundable**, including unused or partially used subscriptions; **no refunds for failure to cancel**. `[V]`

`[I]` **Programme relevance:** if agents receive branded portals, the branding is revocable without notice and the UNO remains fully liable for ULO support it cannot redirect to HighLevel. This is a direct conflict with the team guide's multi-market support model. Treat branded agent portals as reversible at any time and keep the authoritative record in UNO systems.

### 2.3 Telephony is US-centric — directly relevant to NG, PH, KE, BD, GH `[V]`

The programme is SMS-heavy in six emerging markets. Constraints:

- **Regulatory Bundles are required outside the US/Canada** and vary by country *and number type* — Address Bundle (identity + physical address) plus Regulatory Bundle (compliance documents). Created at Sub-Account → Phone Numbers → **Compliance** tab. `[V]`
- **A2P registration is priced on US-centric tiers only:** Sole Proprietor 3,000 segments/day, up to $2.10/mo campaign fee, up to $23.475 registration · Low Volume 600,000/day, $1.50–$10.50/mo · High Volume 600,000/day, $10.50/mo + $68.625. **Other jurisdictions' A2P/10DLC/sender-ID regimes are not covered by vendor pricing.** `[V]`
- ⚠️ **The $3 fast-track fee is non-refundable, and submitting starts both fees regardless of review outcome.** Delete the campaign to stop recurring charges. `[V]`
- **LC Phone is a Twilio front**; number portability between LC↔Twilio and LC↔LC requires a **HighLevel support ticket**. `[V]`
- **UK international long codes blocked for A2P since 1 June 2023**; from **30 Sep 2024 all UK long codes** require an approved Regulatory Bundle plus KYC. **Turkey blocks all international A2P SMS containing URLs, hyperlinks or shortened links from 1 April 2026** (error 30007). `[V]`
- **No documentation found on hosting regions, EU data residency, or in-region storage.** No platform-level DPA located (the *affiliate* programme binds affiliates to a DPA — different scope). `[G]`
- **ToS International Use:** *"HighLevel makes no representation that materials on the Platform are appropriate or available for use in locations outside the United States."* `[V]`

`[I]` **Programme relevance:** budget Regulatory Bundle work and per-country A2P registration as separate, country-by-country engineering tasks for all six markets. Treat participant PII handling as unverified on data residency and DPA. **This is a genuine gap, not a solved item.**

### 2.4 AI unit economics — published, and worth planning against

| Item | Rate `[V]` |
|---|---|
| Conversation AI token (ChatGPT-5) | $1.25 / $10.00 per 1M in/out |
| GPT-5 Mini | $0.25 / $2.00 |
| GPT-4.1 / Mini | $2.00 / $8.00 · $0.40 / $1.60 |
| Voice speech-to-speech | Google `gemini-3.1-flash-live` $0.10/min · OpenAI `gpt-realtime-2` $0.20/min |
| Voice classic stack | Voice Engine $0.045/min + TTS (OpenAI $0.015, Cartesia $0.015, ElevenLabs V2.5 $0.035, **V3 $0.170**) |
| **Phone System on top of every Voice AI call** | **Applies even on AI Employee Unlimited** |
| Agent Studio | Web search $0.01 · video Veo3 Fast **$0.15/second** · Veo3 $0.40/second · image $0.04–$0.12 · TTS **$12.00 per 1M audio output tokens** — **never bundled in any plan** |
| Workflow Pro | Free 100 lifetime · $10/mo 10k · $25/mo 30k · $50/mo 65k; overage $0.010→$0.004. **Excludes ChatGPT/Workflow AI executions** |
| Premium Workflow AI | $0.01/execution (Decision Maker, Intent Detection, Summarize, Translate) |

Agent prompts capped at **15K tokens** `[V]`. Cost control is per-location **AI Usage Limits** with four enforcement modes including *Keep AI running (just notify)* vs *Block AI at the limit*, plus an agency-level **AI Suite** console. **"Unlimited" = 3× Growth inside 5-hour rolling windows.** `[V]`

Two clauses that constrain every "unlimited" assumption `[V]`:
> *"If usage is excessive, abusive, or negatively affects platform performance, HighLevel may throttle, limit, require service upgrades, or **terminate access with or without notice**."*
> *"AI features are software functionality only, require setup, configuration, and ongoing human oversight. Results, performance, cost savings, and efficiencies vary."*

`[I]` The team guide's arithmetic that Growth's 1,000 responses is inadequate at 1,000 leads × 6 responses is **correct and should be extended to token cost per conversation**, which varies with context size. For six languages and low per-participant value, **Conversation AI cost per participant interaction is a first-order budget line**, not an afterthought.

### 2.5 The pricing contradiction the team guide flagged is now resolved `[V]`

The team guide's §11 asks to *"reconcile the conflicting pricing blocks."* Resolution: **`/white-label-crm` lists SaaS Mode under Starter $97, while the SaaS API and the pricing page both state Agency Pro $497 only.** The API doc is explicit: `POST /saas/enable-saas/:locationId` — *"This feature is only available on Agency Pro ($497) plan."* **Trust $497.** A separate, lower-value contradiction: Dedicated IP is "$59/month per IP" on the pricing page but "$59/mo per dedicated domain" in the Pricing Guide — almost certainly billed per domain. Do not budget per-IP.

Rebilling with markup is **$497 only**, confirmed independently by the ToS: *"You may not mark-up or increase any HighLevel Fees that you pass through to Your customers or third parties unless you are enrolled in our $497 tier plan."* AI Employee rebilling also requires $497. **No lifetime licensing** without written approval.

---

## 3. Plai: commercial terms, with the pricing defect identified

### 3.1 Verified pricing `[V]`

| Plan | Monthly | Workspaces | Included "managed" ad spend | Then |
|---|---|---|---|---|
| Brand | **$97** | 1 | $20k/mo | 2.5% |
| Agency | **$297** | 8 | $20k/mo | 2.5% |
| Pro | **$497** | Unlimited | $20k/mo | 2.5% |
| Done For You | from $3,000 | — | not stated | sales |
| Free | $0/yr | 1 brand, 1 user | 2 campaigns, 10 creatives/mo | — |

**Add-ons:** extra brand **$17/mo** · extra user **$7/mo** · white-label **$49/mo**. 7-day trial.
**Annual:** Brand Builder **$790/yr** ($65.83/mo) · Agency White-Label **$2,490/yr** ($207.50/mo) — 10 brands, own-domain white label, **GHL integration**, SaaS configurator.

`[V]` The country plan is right that "$20k managed ad spend included" is a **management allowance, not $20,000 of free advertising** — `help.plai.io/how-billing-works` confirms platforms charge media separately. Good catch, and it is not repeated here.

### 3.2 ⚠️ A pricing-page arithmetic defect the team guide should now know about `[V]`

The savings badges **do not reconcile with the same page's monthly prices**:

| Plan | Badge says | Badge implies | Actual saving vs monthly |
|---|---|---|---|
| Brand Builder | "Save $158 (2 months)" | $97 × 2 = $194 | $1,164 − $790 = **$374** |
| Agency White-Label | "Save $498 (2 months)" | $297 × 2 = $594 | $3,564 − $2,490 = **$1,074** |

`[I]` **Do not reproduce "$158 / $498" in any advert or agent-facing collateral.** Under Google's *Unacceptable business practices* policy, inaccurate pricing and offering what you cannot deliver are grounds for **immediate suspension without warning** — the same policy that already threatens this programme on sublease framing. If an advert ever quotes a Plai saving, use the reconciled figure, or omit the saving entirely.

### 3.3 Plai unit economics for this programme `[I]`

```
Monthly fee = Plan base + 0.025 × max(0, Plai-managed ad spend − 20,000)
```

> ⚠️ **Two unresolved ambiguities `[G]`.** "Then 2.5%" is read as 2.5% of the amount **above** $20k; if Plai charges 2.5% of the whole month past the threshold, every figure changes. And "managed" is undefined — the SaaS Configurator page describes clients running **their own ad accounts** against a prepaid credit balance, which implies a broader meaning than "spend in a Plai-created account". **Confirm both in writing before publishing any unit-economics claim.** This matters here because six-market spend will be small, so the base fee dominates and the ambiguity matters less — but the second ambiguity determines whether Plai can touch client-owned ad accounts at all.

**Fee as % of monthly ad spend:**

| Ad spend | Brand $97 | Brand Builder annual $65.83 |
|---|---|---|
| $2,000 | 4.85% | 3.29% |
| $5,000 | 1.94% | 1.32% |
| $10,000 | 0.97% | 0.66% |
| $20,000 | 0.485% | 0.329% |
| $50,000 | 1.694% | 1.632% |

The curve is **not monotone** — it falls to a $20k cliff then rises back to a 2.5% asymptote. No plan arbitrage below $20k: buy the cheapest plan covering the workspace count.

`[I]` **Crossover against a percentage-of-spend agency** is around **$485–$970/mo of spend** (10–20% of spend bands), with no re-crossover above $20k because Plai's 2.5% marginal rate sits below every agency band. But this compares **fee only**. An agency's fee buys strategy, creative, bid management and reporting; Plai buys software. For a six-market pilot where local partners supply the market knowledge, **the crossover is not the relevant comparison** — the team guide's recommendation to reject the $3,000 Done For You tier and start with native tools remains sound.

### 3.4 The affiliate programme — 40% recurring is the expensive one `[V]`

Plai's Ambassador Program (Referson): **40% on every purchase and renewal, indefinitely** — *"We pay 40% commission on every purchase and renewal for as long as the merchant is a Plai client!"* 30-day attribution window, net of discounts, **PayPal only**, commission forfeited on refund, self-referral prohibited with immediate termination, rates changeable at Plai's sole discretion, every conversion manually inspected.

`[I]` Compare: HighLevel is 40% **recurring** plus a forbidden 5% tier-2; Plai is 40% **recurring, lifetime, uncapped** and PayPal-only. Neither is appropriate as the UNO's promoter compensation mechanism — both are third-party programmes that would sit *on top of* the UNO's own single-level 10% ledger and confuse attribution. **Use the UNO's ledger only.**

### 3.5 What Plai is missing — matters for a pilot with no analytics staff `[G]`

From direct reading of the Features and About pages:

- **No dedicated analytics or reporting module.** No named metrics, no cadence, no scheduled reports, no white-label report branding. "Review results" appears once as a Client Portal bullet. All the impressive figures on the site (CPC −98%, CTR +1500%, CPL $27→$9) appear **only inside customer testimonials**, not as product claims.
- **No post-approval workflow.** Approvals appear only in the Client Portal, for creative.
- **No named user roles** — "Team Members" supports per-workspace assignment and an admin toggle, with no Admin/Editor/Viewer model and no caps stated.
- **No named AI model or vendor** anywhere (no OpenAI, HeyGen, Synthesia, ElevenLabs).
- **No carousel format**; no monthly creative quota published; **no automated bidding strategy feature** named.
- **No reporting integration** — Zapier, HubSpot, Salesforce, GA, Looker Studio all absent from the pages. Named integrations are only: **GoHighLevel**, Pipedrive, Slack, FlowTrack, Stripe.
- The About page carries **contradictory campaign counts** ("100,000+" in one place, "200K+" in another), **dual copyright years (2025 and 2026)**, and a duplicated block of ~14 unrelated "Founder, TechMatter" testimonials containing a Patreon/analytics testimonial that is not a Plai claim. **Do not cite Plai scale numbers.**

**Genuinely useful for this programme `[V]`:** **International Localization — "create ads and posts in 40+ languages" and localize the platform UI**, plus **Brand Context** (auto-generated brand voice and visuals per market) and **Audience Builder** (reusable audience templates). These are the features that map onto six-market localisation. Also **LinkBridge** (one secure link to connect client ad accounts, Pages and Pixels) and the **GoHighLevel integration** — though whether the GHL integration **synchronises leads or only embeds the interface** remains unproven, and the team guide's §11 question on this should stay on the list.

**⚠️ Synthetic presenter risk `[G]`.** Plai markets **AI UGC Avatars** — presenter-style synthetic video. The pages carry **no disclosure requirement** for synthetic creators. If used in any market, undisclosed synthetic presenters that resemble a real earning ULO create FTC endorsement and ad-policy exposure. The team guide's rule — *"use disclosed explainers; never impersonate a real earning ULO"* — is correct and should be treated as a hard constraint, not guidance.

---

## 4. Findings that strengthen the team guide's existing design

These confirm positions the team guide already takes, with the underlying evidence attached.

### 4.1 Two-level, not hierarchical `[V]`

```
Agency / Company (companyId)
  └── Sub-Account / Location (locationId)   ← "Sub-Account (Formerly Location)"
        Contacts · Business · Followers · Opportunity+Pipeline · Conversations → Messages
        Appointments/Calendars · Custom Objects → Records → Associations
        Products → Prices → Orders → Transactions/Subscriptions · Invoices/Estimates/Schedules
        Courses → Offers → Products · Tags · Tasks · Notes · Users
```

`[I]` **The hierarchy is two-level only** — no nested sub-accounts. The only cross-tenant mechanism is a workflow action, **"Copy Contact: Duplicates a contact into another sub-account"**, which is a manual copy, not a foreign key. For a six-market, multi-agent programme this is a real design constraint: **do not model markets as nested agents, and do not rely on contact copies to carry referral attribution** — the team guide's `referral_beneficiary_id` pattern in the authoritative ledger is correct precisely because it avoids this.

### 4.2 The lifecycle automation surface — which is native `[V]`

Confirming the team guide's §7 design has real native support:

| Lifecycle moment | Native HighLevel mechanism |
|---|---|
| Enquiry, consent capture | Form Submitted, Survey Submitted, inbound conversation, Facebook/TikTok/LinkedIn/Google Lead Form Submitted |
| Eligibility pending | Contact Created/Changed, Custom Date Reminder, Goal Event |
| Registered | Contact Created, User Login, New Signup |
| Reserved / Activated | Product Access Granted/Removed, Offer Access Granted/Removed, **Course Grant Offer / Course Revoke Offer**, Custom Trigger |
| First rewarded task | Product Started/Completed, Lesson Started/Completed, **Transcript Generated** |
| D7 / D30 cohort | Custom Date Reminder on a custom-object date field, Smart Lists |
| Inactive | Database Reactivation Templates, Smart Lists, automated birthday/seasonal campaigns |
| Withdrawn / ineligible | Contact DND, Remove from Workflow, Update Custom Value |
| Referral events | Affiliate Created, New Affiliate Sales, Lead Created, Add to Affiliate Manager |
| Escalation to a human | **Manual Action** (human-in-the-loop inside automation), Send Internal Notification |
| Agent comms | **Send Slack Message**, Send Internal Notification, Copy Contact to another sub-account |

**The three primitives that make this competitive `[V]`:** **Custom Code** (arbitrary script in a workflow — the most powerful automation primitive in the product, and **not API-exposed**), **Arrays** (built-in find/filter/transform/iterate — effectively an ETL step), and **Drip Mode** (batch rate-limiting, explicitly framed as *"maintaining reputation and delivery rates"*). **Drip Mode is directly relevant** to the team guide's proposed reminder cadence and to SMS deliverability in six markets.

`[G]` No documented multi-step branch or parallel-join semantics beyond If/Else, Split, Go To and Goal Event. **Given workflows are UI-only, verify complex branching behaviour in a live trial — it cannot be inspected from the spec.**

### 4.3 Enterprise facts `[V]`

**HighLevel LLC, a subsidiary of GoHighLevel Inc.**, 1801 N. Lamar St., Suite 600, Dallas, Texas 75202. Founded 2018 by **Shaun Clark, Varun Vairavan, Robin Alex**. **Texas law; AAA commercial arbitration in Dallas; class-action waiver.** ⚠️ 404 pages render *"© 2026 HighLevel, Inc."* — **two entity names in circulation; have counsel confirm which entity the contract is with.** `[LEGAL]`

`[M]` Scale claims are marketing and **internally contradictory** — the homepage says 7.3B leads / $5.2B sales in 2025; the About timeline says 6.9B / $4.2B. Treat neither as audited.

**Plai `[V]`:** founded 2020 as a marketing community (500K+ followers), **Y Combinator 2021**, backing from Live Nation, Hawke Media and "the creators of Google AdSense". *"We built the first-ever text-to-ad AI model."* Named individual: **Logan Welbaum**. **No funding amount, no team size and no office locations are disclosed.**

---

## 5. Consolidated risk register

Programme-specific; the general vendor risk register lives in the team guide.

| # | Risk | Severity | Basis |
|---|---|---|---|
| N1 | **Recruitment-linked 10% referral share collides with Meta's ban on compensation "partly or fully based on recruitment"** and Google's egregious-violation bar on "official site … so you can earn commission" framing | **Critical** | Policy text `[V]`; mitigated by the 2500-plan's single-level rule — **enforce technically** |
| N2 | **Unnetwork authorisation for promotion and subleasing is a launch gate** ("platform-approved recruitment route and copy"). No marketing tool resolves it | **Critical** | 2500-plan launch gate; country plan `[LEGAL]` |
| N3 | **FTC earnings-claim substantiation** required if any ad states achievable income | **Critical** | FTC BOTS guidance `[V]` `[LEGAL]` |
| N4 | **HighLevel's own affiliate programme is two-level (40% + 5% tier-2)** — stacking it on UNO promoter compensation would create the prohibited structure | **Critical** | HighLevel affiliate policy `[V]` |
| N5 | **Workflows are a 1-operation API** — the automation layer cannot be provisioned, version-controlled or migrated; an account loss means a full rebuild | **High** | Census `[V]` |
| N6 | **White-label branding is revocable at will** while the UNO retains full support liability — conflicts with the multi-market support model | **High** | ToS §2 `[V]` |
| N7 | **MAP policy** governs any agent-facing advertised price; coupons deducted before comparison; price can change anytime | **High** | ToS §2 `[V]` |
| N8 | **Regulatory Bundles and per-country A2P registration required outside US/Canada**, for all six markets, with **non-refundable fast-track fees that bill regardless of review outcome** | **High** | Help centre `[V]` |
| N9 | **No HighLevel data-residency or platform-DPA documentation** for participant PII in six jurisdictions | **High** | `[G]` |
| N10 | **AI UGC avatars with no visible synthetic-media disclosure**, usable to resemble a real earning ULO | **High** | Plai pages `[G]`; FTC/policy `[V]` |
| N11 | **Two-level-only sub-account hierarchy**; contact copies break cross-tenant referral attribution | Medium | `[V]` |
| N12 | **Plai savings badges do not reconcile** ($158 vs $374; $498 vs $1,074) — reproducing them risks the same suspension bar as N1 | Medium | `[V]` |
| N13 | **Plai take-rate base and "managed spend" definition ambiguous** — determines whether Plai can touch client-owned ad accounts | Medium | `[G]` |
| N14 | **No Plai reporting module, no white-label reporting** — the programme's measurement burden stays with the UNO | Medium | `[G]` |
| N15 | **Plai 40% recurring lifetime affiliate** — not usable as promoter compensation | Medium | `[V]` |
| N16 | **Webhook public key rotates out-of-band with no key-id**; a missed rotation silently breaks verification | Medium | `[V]` |
| N17 | **API v3 shipped 11 June 2026**; docs contradict the versioning table on legacy support | Medium | `[V]` |
| N18 | **SaaS Mode plan-tier contradiction** ($97 page vs $497 API/pricing) — resolved as $497, but verify in trial | Medium | `[V]` |
| N19 | **No AI event webhooks** — AI interactions cannot be captured by webhook-only ingestion | Medium | `[G]` |
| N20 | **Agent prompts capped at 15K tokens**; voice carries Phone System charges even on Unlimited; Agent Studio never bundled | Medium | `[V]` |

---

## 6. What this changes in the plan, in priority order

1. **Enforce the single-level referral rule technically, not just in policy.** The 2,500-plan's *"no downstream commissions and no payment merely for recruiting another referrer"* is the correct mitigation for the programme's largest external risk. Make it a **ledger constraint** — a schema or service invariant — so no configuration can create a second level. Do not use HighLevel's affiliate programme for promoter compensation.
2. **Resolve Unetwork authorisation before any campaign spend.** It is already a launch gate. It is not a software question and no subscription answers it.
3. **Snapshot and document every HighLevel workflow as a deployable artefact.** The automation layer is not API-manageable. Put anything durable in custom objects plus the UNO's own service, not only in workflow logic.
4. **Budget per-country Regulatory Bundle and A2P registration as separate engineering tasks** for all six markets, and treat the non-refundable fast-track fee as committed spend.
5. **Resolve the data-residency and DPA gap before processing participant PII in any of the six markets.** Currently unverified.
6. **Re-verify token cost per conversation** before committing to Conversation AI at 1,000 leads × 6 responses. The team guide's allowance arithmetic is right; the token cost is unmodelled.
7. **Do not publish Plai savings figures**, do not use AI UGC avatars as anything resembling an operator, and do not treat Plai's reported CPC/CTR/CPL results as product capabilities.
8. **Keep the commercial recommendation unchanged:** reject Plai Done For You at $3,000/month; at pilot scale, native tools plus repaired application and local agents first; add one subscription only against a measured bottleneck.

---

## 7. Method, and what was not verified

**Sources are primary vendor material** — ToS, pricing pages, help centre articles, and the `GoHighLevel/highlevel-api-docs` OpenAPI specs (44 v3 specs, basis of the 627-operation census). Policy text is quoted from Google Ads, Meta and FTC primary pages. No authenticated product trial was conducted; no subscription, campaign or integration was activated.

**Four research passes were unreliable and this is recorded deliberately.** Three targeted agents returned no usable report — one asserted a report it never produced, one asked which of three options to execute despite instructions not to ask, and one failed twice on a large scope before being split. The merchant-of-record finding, the decisive item in §1.1, was obtained by reading the saved ToS fetches directly rather than delegating it. Plai's inventory was rebuilt from saved primary-source page dumps. **Treat any figure in a prior pass that is not reproduced here as unverified.**

**Explicit gaps carried forward:**

| Gap | Why it matters |
|---|---|
| Plai 2.5% take-rate base; "managed ad spend" definition | Determines whether Plai can act on client-owned ad accounts |
| Whether the Plai→GHL integration syncs leads or only embeds the UI | Determines whether lead data reaches the CRM at all |
| HighLevel platform DPA; hosting regions; EU data residency | Participant PII in six jurisdictions |
| HighLevel inbound-webhook facility vs. the `Inbound Webhook` **trigger** | The team guide's integration blueprint assumes an inbound path |
| Plai reporting metrics, cadence, white-label report branding | Programme measurement |
| Named AI model/vendor behind Plai's creative stack | Synthetic-content policy posture |
| Keygen Std/Ent pricing; Plai managed-service written scope | Not blocking; recorded for completeness |
| Multi-step workflow branch/parallel-join semantics | Must be tested live; not readable from the spec |
| Whether `landing.site` AI landing pages are included in Brand | Team guide §11 question, still open |

**All tax, VAT, employment, payout-licensing, data-protection and advertising-compliance conclusions are mechanical descriptions of who owes what, not advice.** `[LEGAL]` Items require qualified counsel in each market before campaign launch, and none of them is resolved by buying either platform.

---

## 8. Sources

**HighLevel — contract and pricing**
- `gohighlevel.com/terms-of-service` — §§2, 3, 13. Last Updated **June 2026**. Seller-of-record, tax, MAP, lifetime-licence, white-label, refund, termination and international-use clauses. **The decisive source in §1.1.**
- `gohighlevel.com` — `/`, `/pricing`, `/about-us`, `/white-label-crm`, `/affiliate-policy`, `/affiliate-policy/program-rules` (last updated June 2026)
- `github.com/GoHighLevel/highlevel-api-docs` — `apps/v3/*.json` (44 specs, basis of the census), `toc.json`, `docs/oauth/{WebhookAuthentication,Scopes,Authorization,Billing,Faqs}.md`, `docs/country list/Country.md`
- Help centre — Pricing Guide · AI Product Pricing (modified 28 Sep 2026) · A List of Workflow Triggers (19 Jun 2026) · What Are Workflow Actions (3 Jun 2026) · Getting Started: Connect Stripe · Supported Payment Providers · Rebilling, Reselling and Wallets Explained · Regulatory Bundle and Address Creation for Sub-Accounts · International SMS Compliance (UK & Turkey)
- API — `developers.gohighlevel.com` · `/docs/other/rate-limits` · `/Versioning` · `/ghl/saas-api/enable-saas-location` · `/ghl/objects/custom-objects-api` · `/docs/webhook/WebhookIntegrationGuide/`

**Plai**
- `plai.io/features` · `plai.io/pricing` · `plai.io/features/saas-configurator` · About page · `help.plai.io/how-billing-works` · `plai.referson.com` (Ambassador Program terms)
- The team guide's P1–P22 per-feature index remains the canonical feature list; this supplement adds what is **absent** and corrects the pricing.

**Policy and regulatory — primary**
- Google Ads — Unreliable claims `support.google.com/adspolicy/answer/15936857` · Unacceptable business practices `…/15938071` · Enabling dishonest behavior `…/6016086` · Misrepresentation `…/6020955` · Strike system `…/10922738`
- Meta — Prohibited Commercial Practices `transparency.meta.com/policies/community-standards/prohibited-commercial-practices/` · Advertising Standards `…/policies/ad-standards/`
- FTC — Business Opportunity Advertising `ftc.gov/system/files/documents/plain-language/bus68-ads-business-opportunities-how-detect-deception.pdf`
- **Reuters (2025-12-15)** `reuters.com/investigations/metas-trusted-experts-…` — **The Verge (2026-04-30)** `theverge.com/…/meta-manus-ai-ads-website-slop`
- WhatsApp Business Messaging Policy, as updated 23 Sep 2026 — carried in the team guide as X1

**Programme documents**
- `docs/marketing/UNETWORK_2500_LICENSE_MARKETING_PLAN.md` — allocation rules, referral agreement, launch gates, market quotas
- `HIGHLEVEL_PLAI_UNO_DISTRIBUTION_TEAM_GUIDE.md` — design of record
- `UNETWORK_AI_MARKETING_AND_COUNTRY_ACQUISITION_PLAN.md` — country sequencing and pilot economics
