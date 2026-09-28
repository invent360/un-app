# Technical Infrastructure & Operations
## UNO DevOps & Business Systems Guide

---

## Table of Contents

1. [Website Development](#1-website-development)
2. [Logo & Brand Identity](#2-logo--brand-identity)
3. [AI Agent Marketing Automation](#3-ai-agent-marketing-automation)
4. [Risk Management & Platform Dependency](#4-risk-management--platform-dependency)
5. [Fiat Off-Ramp Partnerships](#5-fiat-off-ramp-partnerships)
6. [AI Flier Creation Tools](#6-ai-flier-creation-tools)
7. [Business Automation & Infrastructure](#7-business-automation--infrastructure)

---

## 1. Website Development

### Site Architecture

```
+---------------------------------------------------------------------+
|                         WEBSITE STRUCTURE                           |
+---------------------------------------------------------------------+
|                                                                     |
|   HOME (Landing Page)                                               |
|   +-- Hero Section (Above the fold)                                 |
|   +-- How It Works (3 steps)                                        |
|   +-- Earnings Calculator                                           |
|   +-- Testimonials / Social Proof                                   |
|   +-- FAQ Accordion                                                 |
|   +-- Final CTA                                                     |
|   +-- Footer (Legal, Contact, Social)                               |
|                                                                     |
|   /how-it-works                                                     |
|   +-- Detailed explanation                                          |
|   +-- Video tutorial embed                                          |
|   +-- Technical FAQ                                                 |
|                                                                     |
|   /earnings                                                         |
|   +-- Real earnings screenshots                                     |
|   +-- Monthly averages                                              |
|   +-- Payout history examples                                       |
|                                                                     |
|   /about                                                            |
|   +-- About the operator (you)                                      |
|   +-- About Unetwork (official link)                                |
|   +-- Transparency: revenue split explained                         |
|                                                                     |
|   /faq                                                              |
|   +-- Getting started                                               |
|   +-- Earnings questions                                            |
|   +-- Technical troubleshooting                                     |
|   +-- Withdrawal process                                            |
|                                                                     |
|   /join (or /start)                                                 |
|   +-- Email capture form                                            |
|   +-- Referral code input                                           |
|   +-- Download links (App Store / Play Store)                       |
|                                                                     |
+---------------------------------------------------------------------+
```

---

### Landing Page Design Specifications

#### Above the Fold (Hero Section)

```
+---------------------------------------------------------------------+
|                                                                     |
|   [LOGO]                              [Discord] [Start Earning]     |
|                                                                     |
+---------------------------------------------------------------------+
|                                                                     |
|                   YOUR PHONE CAN EARN MONEY                         |
|                    JUST $1.99/MONTH                                 |
|                                                                     |
|   Turn your smartphone into a passive income machine.               |
|   No investment. No skills needed. Just install and earn.           |
|                                                                     |
|   [  START EARNING NOW  ]  <-- Primary CTA (Red/Orange)             |
|                                                                     |
|   * Only $1.99/mo    * Earn $4-30/month  * Works in background      |
|                                                                     |
|   [Phone mockup showing earnings dashboard]                         |
|                                                                     |
+---------------------------------------------------------------------+
```

**Hero Requirements:**
- Headline: 8 words or less
- Subheadline: 1-2 sentences max
- Single primary CTA (contrasting color)
- 3 benefit bullets with checkmarks
- Device mockup showing app/earnings

---

#### How It Works Section

```
+---------------------------------------------------------------------+
|                                                                     |
|                     HOW IT WORKS                                    |
|                                                                     |
|   +---------+      +---------+      +---------+                     |
|   |    1    |      |    2    |      |    3    |                     |
|   |  [Phone]|  ->  |  [Bolt] |  ->  |  [Cash] |                     |
|   |Download |      | Let It  |      |  Get    |                     |
|   |   App   |      |   Run   |      |  Paid   |                     |
|   +---------+      +---------+      +---------+                     |
|                                                                     |
|   Install the      App runs in      Withdraw to                     |
|   the app in       background       BTC, ETH, USDC                  |
|   2 minutes        automatically    anytime                         |
|                                                                     |
+---------------------------------------------------------------------+
```

---

#### Social Proof Section

```
+---------------------------------------------------------------------+
|                                                                     |
|                    REAL PEOPLE. REAL EARNINGS.                      |
|                                                                     |
|   +-------------------------------------------------------------+   |
|   |  [Photo]  "I've earned $47 this month with 3 phones.        |   |
|   |  Maria    Just let them run and check my balance            |   |
|   |  PH       occasionally. Easiest money ever."                |   |
|   |           *****                                             |   |
|   +-------------------------------------------------------------+   |
|                                                                     |
|   +-------------------------------------------------------------+   |
|   |  [Photo]  "As a student, $12/month covers my data           |   |
|   |  Ahmed    plan. Phone does everything automatically."       |   |
|   |  NG                                                         |   |
|   |           *****                                             |   |
|   +-------------------------------------------------------------+   |
|                                                                     |
|   [Earnings screenshot grid - 4-6 real dashboard images]            |
|                                                                     |
|          JOIN 500+ ACTIVE EARNERS IN OUR DISCORD -->                |
|                                                                     |
+---------------------------------------------------------------------+
```

---

#### FAQ Section

```
+---------------------------------------------------------------------+
|                                                                     |
|                    FREQUENTLY ASKED QUESTIONS                       |
|                                                                     |
|   > How much will I actually earn?                                  |
|     Earnings range from $4-30/month depending on location,          |
|     uptime, and network demand. Average is $5-9/month.              |
|     Top performers with 95%+ uptime can reach $20-30/month.         |
|                                                                     |
|   > Why the $1.99/month fee?                                        |
|     The fee covers operational costs and ensures committed          |
|     users. You earn $4-30/month, making it profitable from          |
|     day one. Split options: 50:50 to 60:40 (you/operator).          |
|                                                                     |
|   > Will it drain my battery?                                       |
|     No. Optimized for less than 5% battery impact. Most users       |
|     don't notice any difference.                                    |
|                                                                     |
|   > What does the app actually do?                                  |
|     It helps telecom companies verify their network quality.        |
|     Your phone runs small connectivity tests in the background.     |
|                                                                     |
|   > How do I withdraw my earnings?                                  |
|     Withdraw to BTC, ETH, USDC, or other crypto. Minimum $5.        |
|                                                                     |
|   > Is my data safe?                                                |
|     Yes. No personal data is collected or transmitted. Only         |
|     anonymized network telemetry.                                   |
|                                                                     |
+---------------------------------------------------------------------+
```

---

### Technical Specifications

| Requirement | Specification |
|-------------|---------------|
| **Platform** | Rust/Leptos (WASM) or Webflow (no-code) |
| **Load Time** | < 2.5 seconds LCP (Largest Contentful Paint) |
| **Mobile** | Mobile-first responsive design |
| **CDN** | Cloudflare or Vercel Edge |
| **Analytics** | Google Analytics 4 + Hotjar heatmaps |
| **Forms** | ConvertKit or Mailchimp integration |
| **SSL** | Required (HTTPS only) |
| **Domain** | Custom domain (e.g., earnwith[name].com) |

### Recommended Tech Stack

**Option A: No-Code (Fastest)**
- Webflow or Framer
- ConvertKit for email
- Tally or Typeform for forms
- ~$20/month

**Option B: Rust/Leptos (Performance + Control)**
- Leptos (Rust WASM framework)
- Tailwind CSS for styling
- Shuttle.rs or Fly.io hosting
- Resend for email
- ~$5-15/month

---

### Conversion Optimization Tactics

| Tactic | Implementation |
|--------|----------------|
| **Exit Intent Popup** | "Wait! Get our quick-start guide before you go" |
| **Social Proof Notifications** | "Maria from Philippines just started earning!" |
| **Urgency (Authentic)** | "500+ people joined this month" |
| **Sticky CTA** | Mobile: fixed bottom button |
| **Live Chat Widget** | Discord widget or Intercom |
| **Trust Badges** | "No Personal Data Collected" badge |

---

## 2. Logo & Brand Identity

### Logo Concept Options

#### Option A: "U + Signal Waves"

```
     +-----+
    /|     |\
   / |     | \
  /  |     |  \
 |   |     |   |    UNODE
 |   |     |   |    Passive Earnings
 |   |     |   |
  \  |     |  /
   \ |     | /
    \|     |/
     +-----+
       )))

Concept: Stylized "U" with emanating signal waves
Represents: Connectivity, network, passive transmission
```

#### Option B: "Phone + Dollar"

```
    +---------+
    |  +---+  |
    |  | $ |  |    UNODE
    |  |   |  |    Your Phone. Your Money.
    |  +---+  |
    |    o    |
    +---------+

Concept: Smartphone with dollar symbol
Represents: Phone earning money, simple concept
```

#### Option C: "Upward Arrow / Growth"

```
       ^
      /|\
     / | \
    /  |  \
   /   |   \      UNODE
  /----+----\     Passive Growth
       |
       |

Concept: Upward arrow with "U" shape
Represents: Growth, passive increase, earning
```

---

### Brand Color Palette

| Color | Hex | Usage |
|-------|-----|-------|
| **Primary Dark** | `#0A0E17` | Backgrounds, headers |
| **Primary Accent** | `#00D4AA` | CTAs, highlights, links |
| **Secondary** | `#6366F1` | Secondary buttons, gradients |
| **Success** | `#22C55E` | Positive earnings, confirmations |
| **Warning** | `#F59E0B` | Alerts, important info |
| **Text Primary** | `#FFFFFF` | Headlines on dark |
| **Text Secondary** | `#94A3B8` | Body text, descriptions |

### Color Application

```
+---------------------------------------------------------------------+
|                                                                     |
|   BACKGROUND: #0A0E17 (dark navy/black)                             |
|                                                                     |
|   +-------------------------------------------------------------+   |
|   |                                                             |   |
|   |   HEADLINE (#FFFFFF)                                        |   |
|   |   Body text (#94A3B8)                                       |   |
|   |                                                             |   |
|   |   [ CTA BUTTON (#00D4AA) ]                                  |   |
|   |                                                             |   |
|   |   Link text (#00D4AA)                                       |   |
|   |                                                             |   |
|   +-------------------------------------------------------------+   |
|                                                                     |
+---------------------------------------------------------------------+
```

---

### Typography

| Element | Font | Weight | Size |
|---------|------|--------|------|
| **Headlines** | Inter / Poppins | Bold (700) | 32-48px |
| **Subheadlines** | Inter / Poppins | Semi-bold (600) | 20-24px |
| **Body** | Inter / Open Sans | Regular (400) | 16-18px |
| **CTAs** | Inter / Poppins | Bold (700) | 16-18px |
| **Captions** | Inter / Open Sans | Regular (400) | 12-14px |

---

### Logo Specifications

| Format | Use Case |
|--------|----------|
| **Primary (Full)** | Website header, documents |
| **Icon Only** | App icon, favicons, social avatars |
| **Horizontal** | Email signatures, banners |
| **White Version** | Dark backgrounds |
| **Dark Version** | Light backgrounds |
| **Minimum Size** | 24px height (icon), 100px width (full) |
| **Clear Space** | 1x logo height on all sides |

---

### Brand Voice Guidelines

| Attribute | Do | Don't |
|-----------|-----|-------|
| **Tone** | Friendly, honest, direct | Corporate jargon, hype |
| **Language** | Simple, accessible | Technical crypto terms |
| **Claims** | Realistic ($4-30/month) | Overpromise ($100+/month) |
| **Approach** | Transparent, helpful | Salesy, pushy |

**Brand Personality Keywords:**
- Trustworthy
- Simple
- Honest
- Modern
- Accessible

---

## 3. AI Agent Marketing Automation

### AI Tools Stack

#### Content Generation

| Tool | Use Case | Cost | Priority |
|------|----------|------|----------|
| **ChatGPT / Claude** | Ad copy, scripts, content ideas | $20/month | High |
| **Jasper** | Marketing copy at scale | $49/month | Medium |
| **CapCut** | AI video editing, captions | Free | High |
| **Canva AI** | Graphics, thumbnails, templates | $13/month | High |
| **ElevenLabs** | AI voiceovers for videos | $5-22/month | Medium |
| **Midjourney/DALL-E** | Custom graphics, mockups | $10-20/month | Low |

---

#### Social Media Automation

| Tool | Function | Cost |
|------|----------|------|
| **Buffer** | Schedule posts across platforms | Free-$15/month |
| **Hootsuite** | Advanced scheduling + analytics | $99/month |
| **Later** | Visual planning, link in bio | Free-$25/month |
| **Manychat** | Instagram/FB DM automation | $15/month |

---

#### Ad Automation

| Tool | Function | Cost |
|------|----------|------|
| **Synter** | AI ad creation for 14+ platforms | $99/month |
| **AdCreative.ai** | Generate ad creatives at scale | $29/month |
| **Madgicx** | Meta ads AI optimization | $49/month |
| **Revealbot** | Automated ad rules and scaling | $99/month |

---

#### Chatbots & Support

| Tool | Function | Cost |
|------|----------|------|
| **Intercom Fin** | AI-powered customer support | $29/seat/month |
| **Tidio** | Website chatbot + AI responses | Free-$29/month |
| **ManyChat** | Instagram/Messenger automation | $15/month |
| **Discord Bots** | Community automation (MEE6, etc.) | Free-$12/month |

---

### AI Workflow Implementation

#### Content Creation Workflow

```
+---------------------------------------------------------------------+
|                    AI CONTENT WORKFLOW                              |
+---------------------------------------------------------------------+
|                                                                     |
|   1. IDEATION (ChatGPT/Claude)                                      |
|      +-- Generate 10 content ideas per week                         |
|          Input: "Give me 10 TikTok video ideas for a passive        |
|                  income phone app targeting students"               |
|                                                                     |
|   2. SCRIPTING (ChatGPT/Claude)                                     |
|      +-- Write scripts for top 3 ideas                              |
|          Input: "Write a 15-second TikTok script for:               |
|                  'Earnings reveal' format, casual tone"             |
|                                                                     |
|   3. VISUAL CREATION (Canva AI + CapCut)                            |
|      +-- Create thumbnails, edit videos                             |
|          - Canva: Thumbnails, carousel graphics                     |
|          - CapCut: Auto-captions, trending effects                  |
|                                                                     |
|   4. SCHEDULING (Buffer/Later)                                      |
|      +-- Auto-post to all platforms                                 |
|          - Set optimal posting times per platform                   |
|          - Queue 1 week of content at once                          |
|                                                                     |
|   5. ENGAGEMENT (Manychat)                                          |
|      +-- Auto-respond to DMs                                        |
|          - "Send me link" -> Auto-reply with landing page           |
|          - FAQ automation                                           |
|                                                                     |
+---------------------------------------------------------------------+
```

---

#### Ad Automation Workflow

```
+---------------------------------------------------------------------+
|                    AI ADS WORKFLOW                                  |
+---------------------------------------------------------------------+
|                                                                     |
|   1. CREATIVE GENERATION (AdCreative.ai / Synter)                   |
|      +-- Generate 10 ad variations                                  |
|          - Headlines, images, videos                                |
|          - Platform-optimized sizes                                 |
|                                                                     |
|   2. A/B TESTING (Platform Native)                                  |
|      +-- Launch 5 creatives per ad set                              |
|          - Let algorithms find winners                              |
|          - 3-5 day testing window                                   |
|                                                                     |
|   3. OPTIMIZATION (Madgicx / Revealbot)                             |
|      +-- Automated rules                                            |
|          - Pause ads with CPA > $10                                 |
|          - Scale ads with CPA < $5                                  |
|          - Auto-duplicate winning ad sets                           |
|                                                                     |
|   4. REPORTING (Platform + Google Sheets)                           |
|      +-- Weekly automated reports                                   |
|          - Spend, conversions, CPA                                  |
|          - Top creatives, underperformers                           |
|                                                                     |
+---------------------------------------------------------------------+
```

---

#### Community AI Automation

```
+---------------------------------------------------------------------+
|                 DISCORD BOT AUTOMATION                              |
+---------------------------------------------------------------------+
|                                                                     |
|   BOT: MEE6 / Carl-bot + Custom                                     |
|                                                                     |
|   Auto-Welcome:                                                     |
|   +-- New member joins -> DM with quick-start guide                 |
|                                                                     |
|   FAQ Bot:                                                          |
|   +-- "!earnings" -> Explains typical earnings                      |
|   +-- "!withdraw" -> Explains withdrawal process                    |
|   +-- "!setup" -> Links to tutorial video                           |
|                                                                     |
|   Milestone Celebration:                                            |
|   +-- User posts in #earnings-proof -> Auto-react + congratulate    |
|                                                                     |
|   Moderation:                                                       |
|   +-- Auto-delete spam, scam links                                  |
|   +-- Auto-mute rule violators                                      |
|                                                                     |
|   Engagement:                                                       |
|   +-- Daily "How much did you earn today?" prompt                   |
|   +-- Weekly leaderboard post                                       |
|                                                                     |
+---------------------------------------------------------------------+
```

---

### Recommended AI Stack (Budget Tiers)

#### Tier 1: Minimal ($0-50/month)

| Tool | Cost | Function |
|------|------|----------|
| ChatGPT Free | $0 | Content ideas, basic copywriting |
| Canva Free | $0 | Basic graphics |
| CapCut Free | $0 | Video editing with AI captions |
| Buffer Free | $0 | Schedule 3 channels |
| MEE6 Free | $0 | Basic Discord automation |
| **Total** | $0 | |

#### Tier 2: Growth ($100-200/month)

| Tool | Cost | Function |
|------|------|----------|
| ChatGPT Plus | $20 | Advanced content, GPT-4 |
| Canva Pro | $13 | Full template access |
| AdCreative.ai | $29 | AI ad creatives |
| Buffer Essentials | $15 | More channels, analytics |
| Manychat Pro | $15 | DM automation |
| Tidio | $29 | Website chatbot |
| **Total** | $121 | |

#### Tier 3: Scale ($300-500/month)

| Tool | Cost | Function |
|------|------|----------|
| Claude Pro | $20 | Advanced reasoning, long context |
| Jasper | $49 | Marketing copy at scale |
| Synter | $99 | Multi-platform ad automation |
| Hootsuite | $99 | Enterprise social management |
| Intercom | $29 | Professional support |
| Revealbot | $99 | Ads automation |
| **Total** | $395 | |

---

### AI Prompt Templates

#### Content Generation

```
PROMPT: TikTok Script Generator

You are a viral TikTok content creator. Create a 15-second script
for a video about a passive income phone app ($1.99/mo, earns $4-30/mo).

Target audience: Students and gig workers in emerging markets
Tone: Casual, authentic, slightly surprised/excited
Format: Hook (3 sec) -> Body (9 sec) -> CTA (3 sec)
Key message: Earn $4-30/month passively with your phone

The video should feel like a real person sharing a discovery,
NOT like an ad. Include suggested visual actions in brackets.
```

#### Ad Copy Generation

```
PROMPT: Facebook Ad Copy Variations

Create 5 variations of Facebook ad copy for:
- Product: Free passive income phone app
- Audience: 25-40, side hustle seekers
- Key benefit: $4-30/month automatically
- Differentiator: Low cost ($1.99), transparent revenue split (50:50 to 60:40)

Each variation should have:
- Primary text (125 chars max)
- Headline (40 chars max)
- Description (30 chars max)

Styles: 1) Direct benefit, 2) Question hook, 3) Social proof,
        4) Problem-solution, 5) Curiosity gap
```

#### FAQ Response Generator

```
PROMPT: Discord FAQ Bot Responses

Create friendly, concise responses for these common questions
about a passive income phone app:

1. "How much will I earn?"
2. "Is this a scam?"
3. "Will it drain my battery?"
4. "How do I withdraw?"
5. "Can I use multiple phones?"

Tone: Helpful, honest, not salesy
Length: 2-3 sentences each
Include relevant details but stay concise
```

---

## 4. Risk Management & Platform Dependency

### Critical Risk Assessment

> **Warning:** The entire business model depends on Unetwork's continued operation. This section addresses platform dependency and mitigation strategies.

### Platform Dependency Risks

| Risk | Severity | Likelihood | Impact |
|------|----------|------------|--------|
| **Unetwork reduces payout rates** | High | Medium | Direct revenue reduction |
| **Unetwork technical outage** | High | Low | User churn, support burden |
| **Regulatory action against Unetwork** | Critical | Low | Business cessation |
| **Token price collapse (MNTx/WMTx)** | High | Medium | Reduced effective earnings |
| **Telecom demand decrease** | Medium | Low | Lower task availability |
| **Competitor platform emerges** | Medium | Medium | User migration |

### Mitigation Strategies

#### 1. Documentation & Contracts

- [ ] Obtain written agreement from Unetwork on payout terms
- [ ] Document minimum notice period for rate changes
- [ ] Clarify node ownership rights and transferability
- [ ] Establish escalation path for disputes

#### 2. Communication Protocol

```
+---------------------------------------------------------------------+
|              PLATFORM CHANGE COMMUNICATION PLAN                     |
+---------------------------------------------------------------------+
|                                                                     |
|   IF: Unetwork announces rate reduction                             |
|   THEN:                                                             |
|   1. Immediate Discord announcement (within 2 hours)                |
|   2. Email to all users (within 24 hours)                           |
|   3. Updated earnings projections on website                        |
|   4. AMA to address concerns                                        |
|                                                                     |
|   IF: Unetwork experiences outage                                   |
|   THEN:                                                             |
|   1. Status post in Discord (immediate)                             |
|   2. Regular updates every 2 hours                                  |
|   3. Post-mortem summary when resolved                              |
|                                                                     |
|   IF: Regulatory concerns emerge                                    |
|   THEN:                                                             |
|   1. Legal consultation (immediate)                                 |
|   2. Transparent communication with community                       |
|   3. Exit strategy activation if needed                             |
|                                                                     |
+---------------------------------------------------------------------+
```

#### 3. Revenue Diversification (Post-Establishment)

After reaching 100+ active licenses, consider:

| Diversification | Description | Timeline |
|-----------------|-------------|----------|
| **Other DePIN projects** | Add Grass, Honeygain, etc. as alternatives | Month 6+ |
| **Crypto education** | Monetize community with courses | Month 9+ |
| **Affiliate partnerships** | Exchange referrals, wallet partnerships | Month 6+ |
| **Consulting** | Help other UNOs launch | Month 12+ |

#### 4. Exit Strategy

If Unetwork fails or becomes unviable:

1. **License Transfer:** Document how to transfer licenses to another UNO
2. **Community Pivot:** Transition Discord to general DePIN/passive income community
3. **Asset Recovery:** Understand node sale/transfer options
4. **Communication:** Pre-drafted message for community if shutdown needed

### Risk Monitoring Dashboard

| Indicator | Source | Check Frequency | Alert Threshold |
|-----------|--------|-----------------|-----------------|
| Unetwork official announcements | Twitter, Discord | Daily | Any policy change |
| MNTx/WMTx token price | CoinGecko | Weekly | >30% drop |
| User earnings reports | Community Discord | Weekly | Consistent complaints |
| Competitor activity | Google Alerts | Weekly | New major competitor |
| Telecom industry news | Industry publications | Monthly | Regulatory changes |

---

## 5. Fiat Off-Ramp Partnerships

### UPDATE: Unetwork Native Bank Withdrawals (Coming Soon)

> **Important:** Unetwork is launching direct bank withdrawals in 47 countries, significantly reducing the need for manual fiat conversion services.

#### Supported Countries for Bank Withdrawals

| Region | Countries |
|--------|-----------|
| **Europe** | Austria, Belgium, Croatia, Cyprus, Czech Republic, Denmark, Estonia, Finland, France, Germany, Greece, Hungary, Iceland, Ireland, Italy, Latvia, Liechtenstein, Lithuania, Luxembourg, Malta, Netherlands, Norway, Poland, Portugal, Romania, Slovakia, Slovenia, Spain, Sweden, Switzerland, United Kingdom |
| **Asia-Pacific** | India, Singapore, Malaysia, Thailand, Indonesia, Philippines, Vietnam, Australia |
| **Africa** | Kenya, South Africa, Ghana, Egypt |
| **Americas** | United States, Canada, Brazil |

#### Impact on Strategy

| Aspect | Before Bank Withdrawals | After Bank Withdrawals |
|--------|-------------------------|------------------------|
| **User friction** | High (crypto exchanges required) | Low (direct to bank) |
| **Your role** | Manual fiat conversion service | Focus on marketing only |
| **Competitive moat** | Fiat service differentiator | Education + community |
| **Target markets** | Limited to crypto-savvy | Broader mainstream appeal |

#### Updated Messaging

**Website/Discord:**
> "Withdraw directly to your bank account in 47 countries -- no crypto exchanges needed! Supported: India (UPI), Philippines, Nigeria, US, UK, EU, and more."

### Legacy Fiat Solutions (For Non-Supported Countries)

For countries not yet supported by Unetwork bank withdrawals:

Target demographics (students, gig workers in emerging markets) often:
- Lack crypto exchange accounts
- Face high withdrawal fees
- Encounter KYC friction
- Prefer local currency

**Impact:** Conversion rates may be lower than projected; churn may be higher post-first-withdrawal.

### Manual Fiat Service (Non-Supported Countries Only)

| Market | Priority | Payment Solution | Integration Method |
|--------|----------|------------------|-------------------|
| **India** | 1 | UPI (PayTM, GPay, PhonePe) | Partner with local crypto-to-fiat service |
| **Philippines** | 1 | GCash, Maya | Direct partnership or Coins.ph |
| **Nigeria** | 1 | Chipper Cash, Flutterwave | Local P2P network |
| **Indonesia** | 2 | OVO, DANA, GoPay | Local exchange partnership |
| **Kenya** | 2 | M-Pesa | BitPesa/AZA Finance |
| **Brazil** | 2 | Pix | Mercado Bitcoin |

### Implementation Options

#### Option A: Manual Concierge Service (Phase 1)

```
+---------------------------------------------------------------------+
|                 MANUAL FIAT CONVERSION SERVICE                      |
+---------------------------------------------------------------------+
|                                                                     |
|   Process:                                                          |
|   1. User requests fiat withdrawal in Discord                       |
|   2. You receive their crypto from Unetwork                         |
|   3. You convert to fiat via local exchange                         |
|   4. You send fiat to user's local wallet                           |
|                                                                     |
|   Fee Structure:                                                    |
|   * 5-8% conversion fee (covers exchange fees + your margin)        |
|   * Minimum withdrawal: $10                                         |
|   * Processing time: 24-48 hours                                    |
|                                                                     |
|   Benefits:                                                         |
|   * Competitive moat (competitors don't offer this)                 |
|   * Higher user satisfaction                                        |
|   * Additional revenue stream                                       |
|                                                                     |
|   Risks:                                                            |
|   * Manual labor intensive                                          |
|   * Exchange rate fluctuation risk                                  |
|   * Requires capital float                                          |
|                                                                     |
+---------------------------------------------------------------------+
```

#### Option B: Partner Integration (Phase 2+)

Partner with existing crypto-to-fiat services:

| Partner Type | Examples | Pros | Cons |
|--------------|----------|------|------|
| **P2P Networks** | Paxful, LocalBitcoins | Wide coverage | Higher fees |
| **Regional Exchanges** | Coins.ph, Luno | Lower fees | Limited regions |
| **Fintech APIs** | Chipper, Flutterwave | Automated | Technical integration |

### Messaging for Fiat Service

**Website/Discord:**
> "We know crypto withdrawals can be complicated. That's why we offer fiat conversion for users in India, Philippines, and Nigeria. Withdraw directly to your GCash, UPI, or mobile money -- no exchange account needed."

**FAQ Addition:**
> **Q: Can I withdraw in my local currency?**
> A: Yes! We offer fiat conversion for select countries. Current options:
> - India: UPI (PayTM, GPay)
> - Philippines: GCash, Maya
> - Nigeria: Mobile money
> Fee: 5-8%. Processing: 24-48 hours.

---

## 6. AI Flier Creation Tools

### Recommended AI Design Tools

| Tool | Best For | Cost | Skill Level |
|------|----------|------|-------------|
| **Canva AI** | All-purpose design, templates | $13/mo (Pro) | Beginner |
| **Adobe Firefly** | Professional graphics | $5/mo (with CC) | Intermediate |
| **Midjourney** | Custom illustrations | $10/mo | Intermediate |
| **Microsoft Designer** | Quick social graphics | Free | Beginner |
| **Recraft AI** | Vector graphics, icons | Free tier | Beginner |
| **Figma AI** | Web mockups, prototypes | Free tier | Intermediate |

### Canva AI Workflow (Recommended)

```
+---------------------------------------------------------------------+
|                    CANVA AI FLIER WORKFLOW                          |
+---------------------------------------------------------------------+
|                                                                     |
|   1. START                                                          |
|      +-- Canva.com -> Create Design -> A5 Document (flier)          |
|                                                                     |
|   2. MAGIC DESIGN                                                   |
|      +-- Click "Magic Design" -> Enter prompt:                      |
|          "Dark tech flier for passive income phone app,             |
|           modern, minimalist, cyan accents, mobile phone image"     |
|                                                                     |
|   3. MAGIC WRITE (AI Copywriting)                                   |
|      +-- Click text box -> "Magic Write" -> Enter prompt:           |
|          "Write 3 bullet points about earning $4-30/month           |
|           passively with your smartphone, casual tone"              |
|                                                                     |
|   4. MAGIC EDIT (Image Enhancement)                                 |
|      +-- Select image -> "Magic Edit" -> Describe changes:          |
|          "Add glowing effect around phone screen"                   |
|                                                                     |
|   5. BACKGROUND REMOVER                                             |
|      +-- Select image -> "BG Remover" -> Auto-remove background     |
|                                                                     |
|   6. MAGIC RESIZE                                                   |
|      +-- "Resize" -> Select formats:                                |
|          * Instagram Story (1080x1920)                              |
|          * Facebook Post (1200x630)                                 |
|          * Twitter Post (1200x675)                                  |
|          -> Auto-adapts layout for each format                      |
|                                                                     |
|   7. EXPORT                                                         |
|      +-- Download as PNG (digital) or PDF (print)                   |
|                                                                     |
+---------------------------------------------------------------------+
```

### AI Prompt Templates for Fliers

#### Hero Image Generation (Midjourney/Firefly)

```
Prompt: "Modern smartphone floating with glowing digital particles,
dark blue background, cyan and purple gradient lighting, passive
income concept, clean minimal style, no text, product photography
style --ar 4:5 --v 6"
```

#### Headline Generation (ChatGPT/Claude)

```
Prompt: "Write 10 headline variations for a flier advertising a
phone app that earns users $4-30/month passively. Target audience:
students and gig workers in emerging markets. Constraints:
- Maximum 8 words
- Focus on benefit, not features
- Avoid words: free, easy, quick
- Include urgency or curiosity element"
```

#### Body Copy Generation

```
Prompt: "Write 4 bullet points (max 10 words each) for a flier:
- Product: Phone app that earns passive income
- Price: $1.99/month
- Earnings: $4-30/month (average $5-9/month)
- USP: No skills needed, runs in background
- Tone: Direct, honest, not salesy"
```

### Localization with AI

| Language | AI Tool | Prompt Modifier |
|----------|---------|-----------------|
| **Spanish** | ChatGPT/Claude | "Translate to Latin American Spanish, casual tone" |
| **Portuguese** | ChatGPT/Claude | "Translate to Brazilian Portuguese" |
| **Hindi** | ChatGPT/Claude | "Translate to Hindi, use Hinglish for tech terms" |
| **Arabic** | ChatGPT/Claude | "Translate to Modern Standard Arabic" |
| **Bahasa Indonesia** | ChatGPT/Claude | "Translate to Bahasa Indonesia, formal" |

### QR Code Integration

| Tool | Features | Cost |
|------|----------|------|
| **QR Code Generator (qr-code-generator.com)** | Custom colors, logo embed | Free |
| **Canva QR** | Built-in, matches design | Included in Pro |
| **Flowcode** | Analytics, dynamic links | Free tier |

**Best Practice:** Use UTM parameters in QR links for tracking:
```
https://yoursite.com/join?utm_source=flier&utm_medium=print&utm_campaign=india_launch
```

---

## 7. Business Automation & Infrastructure

### Website Deployment Options

#### Option A: Single Instance (Cost-Optimized)

```
+---------------------------------------------------------------------+
|              SINGLE INSTANCE ARCHITECTURE                           |
+---------------------------------------------------------------------+
|                                                                     |
|   Google Cloud e2-small ($10-15/month)                              |
|   +-------------------------------------------------------------+   |
|   |                                                             |   |
|   |   +-----------+  +-----------+  +-----------+               |   |
|   |   |   Leptos  |  | PostgreSQL|  |   Resend  |               |   |
|   |   |(Rust WASM)|  |     DB    |  |   Email   |               |   |
|   |   +-----------+  +-----------+  +-----------+               |   |
|   |         |              |              |                     |   |
|   |         +--------------+--------------+                     |   |
|   |                        |                                    |   |
|   |                  +-----+-----+                              |   |
|   |                  |  Caddy/   |                              |   |
|   |                  |  Nginx    |                              |   |
|   |                  +-----------+                              |   |
|   |                        |                                    |   |
|   +------------------------+------------------------------------+   |
|                            |                                        |
|                   +--------+--------+                               |
|                   |   Cloudflare    |                               |
|                   |   (CDN + DNS)   |                               |
|                   +-----------------+                               |
|                                                                     |
|   Pros: Low cost, simple, sufficient for <10K daily visitors        |
|   Cons: Single point of failure, no geographic redundancy           |
|                                                                     |
|   Monthly Cost: ~$15-25                                             |
|   * GCP e2-small: $10-15                                            |
|   * Managed PostgreSQL: $0 (SQLite) or $10 (Cloud SQL)              |
|   * Resend: $0-20 (based on volume)                                 |
|   * Cloudflare: $0 (free tier)                                      |
|                                                                     |
+---------------------------------------------------------------------+
```

#### Option B: Kubernetes Cluster (Production-Grade)

```
+---------------------------------------------------------------------+
|              KUBERNETES ARCHITECTURE (3-NODE)                       |
+---------------------------------------------------------------------+
|                                                                     |
|              GKE Autopilot or EKS (~$150-300/month)                 |
|   +-------------------------------------------------------------+   |
|   |                                                             |   |
|   |   +------------------------------------------------------+  |   |
|   |   |              Ingress Controller (Traefik)            |  |   |
|   |   +------------------------------------------------------+  |   |
|   |                              |                              |   |
|   |             +----------+----------+----------+              |   |
|   |             |  Node 1  |  Node 2  |  Node 3  |              |   |
|   |             |          |          |          |              |   |
|   |             | +------+ | +------+ | +------+ |              |   |
|   |             | | App  | | | App  | | | App  | |              |   |
|   |             | | Pod  | | | Pod  | | | Pod  | |              |   |
|   |             | +------+ | +------+ | +------+ |              |   |
|   |             +----------+----------+----------+              |   |
|   |                              |                              |   |
|   |   +----------------------+-------------------------+        |   |
|   |   |          Cloud SQL (PostgreSQL - HA)           |        |   |
|   |   +------------------------------------------------+        |   |
|   |                                                             |   |
|   +-------------------------------------------------------------+   |
|                                                                     |
|   Monthly Cost: ~$200-350                                           |
|   * GKE Autopilot: $150-200                                         |
|   * Cloud SQL HA: $50-100                                           |
|   * Resend/SendGrid: $20-50                                         |
|                                                                     |
+---------------------------------------------------------------------+
```

### High Availability (Multi-Region)

```
+---------------------------------------------------------------------+
|              GLOBAL HIGH AVAILABILITY ARCHITECTURE                  |
+---------------------------------------------------------------------+
|                                                                     |
|                     +---------------------+                         |
|                     |      Cloudflare     |                         |
|                     |     (Global LB)     |                         |
|                     +----------+----------+                         |
|                                |                                    |
|     +--------------------------+---------------------------+        |
|     |                          |                           |        |
|     v                          v                           v        |
| +-----------+           +-----------+           +-----------+       |
| |  AMERICAS |           |  EUROPE   |           | ASIA-PAC  |       |
| |  us-east1 |           | eu-west1  |           | asia-se1  |       |
| +-----------+           +-----------+           +-----------+       |
| | K8s Pod   |           | K8s Pod   |           | K8s Pod   |       |
| | + DB Read |           | + DB Read |           | + DB Read |       |
| |   Replica |           |   Replica |           |   Replica |       |
| +-----+-----+           +-----+-----+           +-----+-----+       |
|       |                       |                       |             |
|       +-----------------------+-----------------------+             |
|                               |                                     |
|                     +---------+---------+                           |
|                     |     Primary DB    |                           |
|                     |   (us-central1)   |                           |
|                     +-------------------+                           |
|                                                                     |
|   Latency by Region:                                                |
|   * Americas: <50ms                                                 |
|   * Europe: <100ms                                                  |
|   * Asia-Pacific: <150ms                                            |
|   * Africa: <200ms (via Europe)                                     |
|                                                                     |
|   Monthly Cost: ~$500-800                                           |
|                                                                     |
+---------------------------------------------------------------------+
```

### Localization Implementation

#### Architecture for Multi-Language Support

```
+---------------------------------------------------------------------+
|                   LOCALIZATION SYSTEM                               |
+---------------------------------------------------------------------+
|                                                                     |
|   USER REQUEST                                                      |
|        |                                                            |
|        v                                                            |
|   +-------------------------------------------------------------+   |
|   |              LANGUAGE DETECTION                             |   |
|   |  Priority: URL param > Cookie > Browser > IP Geo            |   |
|   +-------------------------------------------------------------+   |
|        |                                                            |
|        v                                                            |
|   +-------------------------------------------------------------+   |
|   |              CONTENT DELIVERY                               |   |
|   |                                                             |   |
|   |   /en/ -> English (default)                                 |   |
|   |   /es/ -> Spanish (Latin America)                           |   |
|   |   /pt/ -> Portuguese (Brazil)                               |   |
|   |   /hi/ -> Hindi (India)                                     |   |
|   |   /ar/ -> Arabic (MENA)                                     |   |
|   |   /id/ -> Bahasa Indonesia                                  |   |
|   |   /tl/ -> Tagalog (Philippines)                             |   |
|   |   /sw/ -> Swahili (East Africa)                             |   |
|   |   /fr/ -> French (West Africa)                              |   |
|   |                                                             |   |
|   +-------------------------------------------------------------+   |
|                                                                     |
+---------------------------------------------------------------------+
```

#### Language Mapping by Country

| Country | Primary Language | Code | Fallback |
|---------|------------------|------|----------|
| **India** | English / Hindi | en/hi | English |
| **Mexico** | Spanish | es-MX | es |
| **Brazil** | Portuguese | pt-BR | pt |
| **UAE/Egypt/Tunisia** | Arabic | ar | English |
| **Indonesia** | Bahasa Indonesia | id | English |
| **Philippines** | English / Tagalog | en/tl | English |
| **Nigeria** | English / Pidgin | en | English |
| **Kenya** | English / Swahili | en/sw | English |
| **Vietnam** | Vietnamese | vi | English |
| **Thailand** | Thai | th | English |

#### Implementation Options

| Approach | Complexity | Best For |
|----------|------------|----------|
| **i18n Library (leptos-i18n, fluent-rs)** | Medium | Rust/Leptos apps |
| **Headless CMS (Contentful, Sanity)** | Low | No-code translation mgmt |
| **Static JSON files** | Low | Simple sites |
| **AI Translation (DeepL API)** | Low | Auto-translation with review |

### AI Support Bot (24/7)

#### Architecture

```
+---------------------------------------------------------------------+
|                   AI SUPPORT BOT SYSTEM                             |
+---------------------------------------------------------------------+
|                                                                     |
|   USER MESSAGE (Discord/Web/Telegram)                               |
|        |                                                            |
|        v                                                            |
|   +-------------------------------------------------------------+   |
|   |              INTENT CLASSIFICATION                          |   |
|   |              (Claude Haiku / GPT-4o-mini)                   |   |
|   +-------------------------------------------------------------+   |
|        |                                                            |
|        +-- FAQ Intent -> RAG on knowledge base                      |
|        |                                                            |
|        +-- Technical Issue -> Troubleshooting flow                  |
|        |                                                            |
|        +-- Earnings Question -> Pull from user data + explain       |
|        |                                                            |
|        +-- Withdrawal Help -> Step-by-step guide                    |
|        |                                                            |
|        +-- Complex/Sensitive -> Escalate to human                   |
|                                                                     |
|   KNOWLEDGE BASE (RAG):                                             |
|   +-- FAQ documents                                                 |
|   +-- Unetwork user guide                                           |
|   +-- Troubleshooting playbooks                                     |
|   +-- Earnings calculation explainers                               |
|   +-- Past resolved tickets (anonymized)                            |
|                                                                     |
|   ESCALATION TRIGGERS:                                              |
|   +-- User requests human                                           |
|   +-- Sentiment = frustrated (3+ messages)                          |
|   +-- Payment/withdrawal issues                                     |
|   +-- Account security concerns                                     |
|   +-- Bot confidence < 70%                                          |
|                                                                     |
+---------------------------------------------------------------------+
```

#### Bot Implementation Options

| Platform | Best For | Cost | Setup Complexity |
|----------|----------|------|------------------|
| **Intercom Fin** | Website chat | $29/seat + AI usage | Low |
| **Tidio AI** | Website + email | $29/mo | Low |
| **CustomGPT** | Custom trained bot | $49/mo | Medium |
| **Botpress** | Multi-channel, self-hosted | Free (self-hosted) | High |
| **Discord + OpenAI API** | Discord-native | API costs only | Medium |

#### Sample Bot Training Data

```json
{
  "intents": [
    {
      "intent": "earnings_question",
      "examples": [
        "How much will I earn?",
        "What are the earnings like?",
        "How much money can I make?",
        "Is it worth it?"
      ],
      "response": "Earnings range from $4-30/month. Average users earn $5-9/month with 85%+ uptime, while top performers reach $20-30/month. Your earnings depend on: (1) device uptime, (2) enabled tasks, (3) network activity in your region. Check our #earnings-proof channel for real examples!"
    },
    {
      "intent": "withdrawal_help",
      "examples": [
        "How do I withdraw?",
        "How to get my money?",
        "Withdrawal process?"
      ],
      "response": "To withdraw: (1) Open app -> Wallet tab, (2) Click 'Withdraw', (3) Choose method (bank or crypto), (4) Enter amount (min $5), (5) Confirm. Processing: 1-3 business days. Need help? Post in #support!"
    }
  ]
}
```

### Infrastructure Decision Matrix

| Factor | Single Instance | K8s (3-node) | Multi-Region HA |
|--------|-----------------|--------------|-----------------|
| **Monthly Cost** | $15-25 | $200-350 | $500-800 |
| **Max Users** | ~5,000 | ~50,000 | 500,000+ |
| **Uptime SLA** | 99% | 99.5% | 99.9% |
| **Setup Time** | 1-2 hours | 1-2 days | 1 week |
| **Maintenance** | Low | Medium | High |
| **Recommended For** | Launch, MVP | Growth phase | Scale phase |

### Recommended Migration Path

```
Month 1-6:  Single Instance (GCP e2-small)
            +-- Cost: $15-25/month
            +-- Focus: Prove model, iterate fast

Month 6-12: K8s 3-node (GKE Autopilot)
            +-- Cost: $200-350/month
            +-- Trigger: >1,000 active licenses

Month 12+:  Multi-Region HA
            +-- Cost: $500-800/month
            +-- Trigger: >5,000 active licenses
```

---

## Revenue Split Economics

### Split Rate Options & Impact

Understanding how different split rates affect your revenue is critical for pricing strategy and competitive positioning.

```
+-------------------------------------------------------------------------+
|                    SPLIT RATE ECONOMICS OVERVIEW                        |
+-------------------------------------------------------------------------+
|                                                                         |
|   FORMULA: Net Revenue = (User Earnings × Your %) - $1.99 subscription  |
|                                                                         |
|   SPLIT OPTIONS:                                                        |
|   +--------+------------+----------+----------------------------------+ |
|   | Split  | User Gets  | You Get  | Strategy                         | |
|   +--------+------------+----------+----------------------------------+ |
|   | 50:50  |    50%     |   50%    | Maximum operator margin          | |
|   | 52:48  |    52%     |   48%    | Balanced approach                | |
|   | 55:45  |    55%     |   45%    | User incentive focus             | |
|   | 60:40  |    60%     |   40%    | High user attraction             | |
|   +--------+------------+----------+----------------------------------+ |
|                                                                         |
+-------------------------------------------------------------------------+
```

### Net Revenue Per License by Earnings Scenario

The new 8-tier earnings system reflects real-world performance ranges:

```
+---------------------------------------------------------------------------------------------------------------------+
|                           NET REVENUE PER LICENSE (After $1.99 Subscription)                                        |
+---------------------------------------------------------------------------------------------------------------------+
|                                                                                                                     |
|   FORMULA: Net = (Device Earnings x Operator %) - $1.99                                                             |
|                                                                                                                     |
|   Earnings |  50:50   |  52:48   |  55:45   |  60:40   |  Tier       |
|   ---------|----------|----------|----------|----------|-------------|
|   $4/mo    |   $0.01  |  -$0.07  |  -$0.19  |  -$0.39  |  Poor       |
|   $5/mo    |   $0.51  |   $0.41  |   $0.26  |   $0.01  |  Average    |
|   $6/mo    |   $1.01  |   $0.89  |   $0.71  |   $0.41  |  Acceptable |
|   $9/mo    |   $2.51  |   $2.33  |   $2.06  |   $1.61  |  OK         |
|   $12/mo   |   $4.01  |   $3.77  |   $3.41  |   $2.81  |  Good       |
|   $20/mo   |   $8.01  |   $7.61  |   $7.01  |   $6.01  |  Very Good  |
|   $25/mo   |  $10.51  |  $10.01  |   $9.26  |   $8.01  |  Excellent  |
|   $30/mo   |  $13.01  |  $12.41  |  $11.51  |  $10.01  |  Champion   |
|                                                                                                                     |
|   CALCULATION EXAMPLES:                                                                                             |
|   * $9/mo at 50:50 -> $9 x 50% = $4.50 - $1.99 = $2.51 net                                                         |
|   * $20/mo at 55:45 -> $20 x 45% = $9.00 - $1.99 = $7.01 net                                                       |
|   * $30/mo at 60:40 -> $30 x 40% = $12.00 - $1.99 = $10.01 net                                                     |
|                                                                                                                     |
|   BREAK-EVEN EARNINGS (Where Net = $0):                                                                             |
|   50:50: $3.98/mo | 52:48: $4.15/mo | 55:45: $4.42/mo | 60:40: $4.98/mo                                            |
|                                                                                                                     |
+---------------------------------------------------------------------------------------------------------------------+
```

### Monthly Revenue Projections by License Count

#### At $4/mo - Poor Scenario

```
+---------------------------------------------------------------------------------------------------------------------+
|                                    $4/MONTH EARNINGS - MONTHLY NET REVENUE                                          |
+---------------------------------------------------------------------------------------------------------------------+
|   Net/License: 50:50=$0.01 | 52:48=-$0.07 | 55:45=-$0.19 | 60:40=-$0.39                                             |
+---------------------------------------------------------------------------------------------------------------------+
|                                                                                                                     |
|   Licenses |  50:50    |  52:48    |  55:45    |  60:40    |                                                        |
|   ---------|-----------|-----------|-----------|-----------|                                                        |
|      10    |    $0.10  |   -$0.70  |   -$1.90  |   -$3.90  |                                                        |
|      50    |    $0.50  |   -$3.50  |   -$9.50  |  -$19.50  |                                                        |
|     100    |    $1.00  |   -$7.00  |  -$19.00  |  -$39.00  |                                                        |
|     200    |    $2.00  |  -$14.00  |  -$38.00  |  -$78.00  |                                                        |
|     350    |    $3.50  |  -$24.50  |  -$66.50  | -$136.50  |                                                        |
|     500    |    $5.00  |  -$35.00  |  -$95.00  | -$195.00  |                                                        |
|   1,000    |   $10.00  |  -$70.00  | -$190.00  | -$390.00  |                                                        |
|   2,200    |   $22.00  | -$154.00  | -$418.00  | -$858.00  |                                                        |
|                                                                                                                     |
+---------------------------------------------------------------------------------------------------------------------+
```

#### At $5/mo - Average Scenario

```
+---------------------------------------------------------------------------------------------------------------------+
|                                    $5/MONTH EARNINGS - MONTHLY NET REVENUE                                          |
+---------------------------------------------------------------------------------------------------------------------+
|   Net/License: 50:50=$0.51 | 52:48=$0.41 | 55:45=$0.26 | 60:40=$0.01                                                |
+---------------------------------------------------------------------------------------------------------------------+
|                                                                                                                     |
|   Licenses |  50:50    |  52:48    |  55:45    |  60:40    |                                                        |
|   ---------|-----------|-----------|-----------|-----------|                                                        |
|      10    |    $5.10  |    $4.10  |    $2.60  |    $0.10  |                                                        |
|      50    |   $25.50  |   $20.50  |   $13.00  |    $0.50  |                                                        |
|     100    |   $51.00  |   $41.00  |   $26.00  |    $1.00  |                                                        |
|     200    |  $102.00  |   $82.00  |   $52.00  |    $2.00  |                                                        |
|     350    |  $178.50  |  $143.50  |   $91.00  |    $3.50  |                                                        |
|     500    |  $255.00  |  $205.00  |  $130.00  |    $5.00  |                                                        |
|   1,000    |  $510.00  |  $410.00  |  $260.00  |   $10.00  |                                                        |
|   2,200    |$1,122.00  |  $902.00  |  $572.00  |   $22.00  |                                                        |
|                                                                                                                     |
+---------------------------------------------------------------------------------------------------------------------+
```

#### At $6/mo - Acceptable Scenario

```
+---------------------------------------------------------------------------------------------------------------------+
|                                    $6/MONTH EARNINGS - MONTHLY NET REVENUE                                          |
+---------------------------------------------------------------------------------------------------------------------+
|   Net/License: 50:50=$1.01 | 52:48=$0.89 | 55:45=$0.71 | 60:40=$0.41                                                |
+---------------------------------------------------------------------------------------------------------------------+
|                                                                                                                     |
|   Licenses |  50:50    |  52:48    |  55:45    |  60:40    |                                                        |
|   ---------|-----------|-----------|-----------|-----------|                                                        |
|      10    |   $10.10  |    $8.90  |    $7.10  |    $4.10  |                                                        |
|      50    |   $50.50  |   $44.50  |   $35.50  |   $20.50  |                                                        |
|     100    |  $101.00  |   $89.00  |   $71.00  |   $41.00  |                                                        |
|     200    |  $202.00  |  $178.00  |  $142.00  |   $82.00  |                                                        |
|     350    |  $353.50  |  $311.50  |  $248.50  |  $143.50  |                                                        |
|     500    |  $505.00  |  $445.00  |  $355.00  |  $205.00  |                                                        |
|   1,000    |$1,010.00  |  $890.00  |  $710.00  |  $410.00  |                                                        |
|   2,200    |$2,222.00  |$1,958.00  |$1,562.00  |  $902.00  |                                                        |
|                                                                                                                     |
+---------------------------------------------------------------------------------------------------------------------+
```

#### At $9/mo - OK Scenario

```
+---------------------------------------------------------------------------------------------------------------------+
|                                    $9/MONTH EARNINGS - MONTHLY NET REVENUE                                          |
+---------------------------------------------------------------------------------------------------------------------+
|   Net/License: 50:50=$2.51 | 52:48=$2.33 | 55:45=$2.06 | 60:40=$1.61                                                |
+---------------------------------------------------------------------------------------------------------------------+
|                                                                                                                     |
|   Licenses |  50:50    |  52:48    |  55:45    |  60:40    |                                                        |
|   ---------|-----------|-----------|-----------|-----------|                                                        |
|      10    |   $25.10  |   $23.30  |   $20.60  |   $16.10  |                                                        |
|      50    |  $125.50  |  $116.50  |  $103.00  |   $80.50  |                                                        |
|     100    |  $251.00  |  $233.00  |  $206.00  |  $161.00  |                                                        |
|     200    |  $502.00  |  $466.00  |  $412.00  |  $322.00  |                                                        |
|     350    |  $878.50  |  $815.50  |  $721.00  |  $563.50  |                                                        |
|     500    |$1,255.00  |$1,165.00  |$1,030.00  |  $805.00  |                                                        |
|   1,000    |$2,510.00  |$2,330.00  |$2,060.00  |$1,610.00  |                                                        |
|   2,200    |$5,522.00  |$5,126.00  |$4,532.00  |$3,542.00  |                                                        |
|                                                                                                                     |
+---------------------------------------------------------------------------------------------------------------------+
```

#### At $12/mo - Good Scenario

```
+---------------------------------------------------------------------------------------------------------------------+
|                                   $12/MONTH EARNINGS - MONTHLY NET REVENUE                                          |
+---------------------------------------------------------------------------------------------------------------------+
|   Net/License: 50:50=$4.01 | 52:48=$3.77 | 55:45=$3.41 | 60:40=$2.81                                                |
+---------------------------------------------------------------------------------------------------------------------+
|                                                                                                                     |
|   Licenses |  50:50    |  52:48    |  55:45    |  60:40    |                                                        |
|   ---------|-----------|-----------|-----------|-----------|                                                        |
|      10    |   $40.10  |   $37.70  |   $34.10  |   $28.10  |                                                        |
|      50    |  $200.50  |  $188.50  |  $170.50  |  $140.50  |                                                        |
|     100    |  $401.00  |  $377.00  |  $341.00  |  $281.00  |                                                        |
|     200    |  $802.00  |  $754.00  |  $682.00  |  $562.00  |                                                        |
|     350    |$1,403.50  |$1,319.50  |$1,193.50  |  $983.50  |                                                        |
|     500    |$2,005.00  |$1,885.00  |$1,705.00  |$1,405.00  |                                                        |
|   1,000    |$4,010.00  |$3,770.00  |$3,410.00  |$2,810.00  |                                                        |
|   2,200    |$8,822.00  |$8,294.00  |$7,502.00  |$6,182.00  |                                                        |
|                                                                                                                     |
+---------------------------------------------------------------------------------------------------------------------+
```

#### At $20/mo - Very Good Scenario

```
+---------------------------------------------------------------------------------------------------------------------+
|                                   $20/MONTH EARNINGS - MONTHLY NET REVENUE                                          |
+---------------------------------------------------------------------------------------------------------------------+
|   Net/License: 50:50=$8.01 | 52:48=$7.61 | 55:45=$7.01 | 60:40=$6.01                                                |
+---------------------------------------------------------------------------------------------------------------------+
|                                                                                                                     |
|   Licenses |  50:50     |  52:48     |  55:45     |  60:40     |                                                    |
|   ---------|------------|------------|------------|------------|                                                    |
|      10    |    $80.10  |    $76.10  |    $70.10  |    $60.10  |                                                    |
|      50    |   $400.50  |   $380.50  |   $350.50  |   $300.50  |                                                    |
|     100    |   $801.00  |   $761.00  |   $701.00  |   $601.00  |                                                    |
|     200    | $1,602.00  | $1,522.00  | $1,402.00  | $1,202.00  |                                                    |
|     350    | $2,803.50  | $2,663.50  | $2,453.50  | $2,103.50  |                                                    |
|     500    | $4,005.00  | $3,805.00  | $3,505.00  | $3,005.00  |                                                    |
|   1,000    | $8,010.00  | $7,610.00  | $7,010.00  | $6,010.00  |                                                    |
|   2,200    |$17,622.00  |$16,742.00  |$15,422.00  |$13,222.00  |                                                    |
|                                                                                                                     |
+---------------------------------------------------------------------------------------------------------------------+
```

#### At $25/mo - Excellent Scenario

```
+---------------------------------------------------------------------------------------------------------------------+
|                                  $25/MONTH EARNINGS - MONTHLY NET REVENUE                                           |
+---------------------------------------------------------------------------------------------------------------------+
|   Net/License: 50:50=$10.51 | 52:48=$10.01 | 55:45=$9.26 | 60:40=$8.01                                              |
+---------------------------------------------------------------------------------------------------------------------+
|                                                                                                                     |
|   Licenses |  50:50     |  52:48     |  55:45     |  60:40     |                                                    |
|   ---------|------------|------------|------------|------------|                                                    |
|      10    |   $105.10  |   $100.10  |    $92.60  |    $80.10  |                                                    |
|      50    |   $525.50  |   $500.50  |   $463.00  |   $400.50  |                                                    |
|     100    | $1,051.00  | $1,001.00  |   $926.00  |   $801.00  |                                                    |
|     200    | $2,102.00  | $2,002.00  | $1,852.00  | $1,602.00  |                                                    |
|     350    | $3,678.50  | $3,503.50  | $3,241.00  | $2,803.50  |                                                    |
|     500    | $5,255.00  | $5,005.00  | $4,630.00  | $4,005.00  |                                                    |
|   1,000    |$10,510.00  |$10,010.00  | $9,260.00  | $8,010.00  |                                                    |
|   2,200    |$23,122.00  |$22,022.00  |$20,372.00  |$17,622.00  |                                                    |
|                                                                                                                     |
+---------------------------------------------------------------------------------------------------------------------+
```

#### At $30/mo - Champion Scenario

```
+---------------------------------------------------------------------------------------------------------------------+
|                                  $30/MONTH EARNINGS - MONTHLY NET REVENUE                                           |
+---------------------------------------------------------------------------------------------------------------------+
|   Net/License: 50:50=$13.01 | 52:48=$12.41 | 55:45=$11.51 | 60:40=$10.01                                            |
+---------------------------------------------------------------------------------------------------------------------+
|                                                                                                                     |
|   Licenses |  50:50     |  52:48     |  55:45     |  60:40     |                                                    |
|   ---------|------------|------------|------------|------------|                                                    |
|      10    |   $130.10  |   $124.10  |   $115.10  |   $100.10  |                                                    |
|      50    |   $650.50  |   $620.50  |   $575.50  |   $500.50  |                                                    |
|     100    | $1,301.00  | $1,241.00  | $1,151.00  | $1,001.00  |                                                    |
|     200    | $2,602.00  | $2,482.00  | $2,302.00  | $2,002.00  |                                                    |
|     350    | $4,553.50  | $4,343.50  | $4,028.50  | $3,503.50  |                                                    |
|     500    | $6,505.00  | $6,205.00  | $5,755.00  | $5,005.00  |                                                    |
|   1,000    |$13,010.00  |$12,410.00  |$11,510.00  |$10,010.00  |                                                    |
|   2,200    |$28,622.00  |$27,302.00  |$25,322.00  |$22,022.00  |                                                    |
|                                                                                                                     |
+---------------------------------------------------------------------------------------------------------------------+
```

### Annual Revenue Projections (Full Capacity: 2,200 Licenses)

```
+-------------------------------------------------------------------------------------------------------------------------------------+
|                                       ANNUAL NET REVENUE AT FULL CAPACITY (2,200 LICENSES)                                          |
+-------------------------------------------------------------------------------------------------------------------------------------+
|                                                                                                                                     |
|   Tier       | Earnings |   50:50    |   52:48    |   55:45    |   60:40    |
|   -----------|----------|------------|------------|------------|------------|
|   Poor       |  $4/mo   |      $264  |   -$1,848  |   -$5,016  |  -$10,296  |
|   Average    |  $5/mo   |   $13,464  |   $10,824  |    $6,864  |      $264  |
|   Acceptable |  $6/mo   |   $26,664  |   $23,496  |   $18,744  |   $10,824  |
|   OK         |  $9/mo   |   $66,264  |   $61,512  |   $54,384  |   $42,504  |
|   Good       | $12/mo   |  $105,864  |   $99,528  |   $90,024  |   $74,184  |
|   Very Good  | $20/mo   |  $211,464  |  $200,904  |  $185,064  |  $158,664  |
|   Excellent  | $25/mo   |  $277,464  |  $264,264  |  $244,464  |  $211,464  |
|   Champion   | $30/mo   |  $343,464  |  $327,624  |  $303,864  |  $264,264  |
|                                                                                                                                     |
|   Note: These projections assume 100% utilization and 0% churn.                                                                     |
|   Realistic Year 1 targets should assume 50-70% of these figures.                                                                   |
|                                                                                                                                     |
|   REALISTIC YEAR 1 ESTIMATES (at 60% of full capacity):                                                                             |
|   -----------|----------|------------|------------|------------|------------|
|   Average    |  $5/mo   |    $8,078  |    $6,494  |    $4,118  |      $158  |
|   OK         |  $9/mo   |   $39,758  |   $36,907  |   $32,630  |   $25,502  |
|   Good       | $12/mo   |   $63,518  |   $59,717  |   $54,014  |   $44,510  |
|   Very Good  | $20/mo   |  $126,878  |  $120,542  |  $111,038  |   $95,198  |
|   Excellent  | $25/mo   |  $166,478  |  $158,558  |  $146,678  |  $126,878  |
|   Champion   | $30/mo   |  $206,078  |  $196,574  |  $182,318  |  $158,558  |
|                                                                                                                                     |
+-------------------------------------------------------------------------------------------------------------------------------------+
```

### Split Strategy Decision Matrix

```
+-------------------------------------------------------------------------------------------------------------------------------------+
|                                                 SPLIT RATE SELECTION GUIDE                                                          |
+-------------------------------------------------------------------------------------------------------------------------------------+
|                                                                                                                                     |
|   MARKET CONDITIONS              | RECOMMENDED SPLIT       | RATIONALE                                                             |
|   -------------------------------|-------------------------|-----------------------------------------------------------------------|
|   Heavy UNO competition          | 55:45 or 60:40          | Attract users with higher share                                       |
|   Low/no competition             | 50:50                   | Maximize margins                                                      |
|   Poor earnings ($4/mo)          | 50:50 ONLY              | Only marginally profitable split                                      |
|   Average earnings ($5/mo)       | 50:50 to 55:45          | All profitable except 60:40 marginal                                  |
|   Acceptable earnings ($6/mo)    | 50:50 to 60:40          | All splits profitable                                                 |
|   OK earnings ($9/mo)            | 50:50 to 60:40          | All splits profitable                                                 |
|   Good earnings ($12/mo)         | 50:50 to 60:40          | Growth focus                                                          |
|   Very Good+ earnings ($20+/mo)  | 50:50 to 60:40          | Max user attraction                                                   |
|   High churn (>15%/mo)           | 55:45 or 60:40          | Improve retention                                                     |
|   Strong retention (<8%/mo)      | 50:50                   | Optimize revenue                                                      |
|   Launch phase                   | 55:45                   | Build user base                                                       |
|   Established (500+ users)       | 50:50 or 52:48          | Balance growth                                                        |
|                                                                                                                                     |
|   PROFITABILITY BY SPLIT (Break-Even Device Earnings):                                                                              |
|   -------------------------------|-------------------------|-----------------------------------------------------------------------|
|   50:50 split                    | $3.98/mo+               | Maximum operator margin                                               |
|   52:48 split                    | $4.15/mo+               | Balanced approach                                                     |
|   55:45 split                    | $4.42/mo+               | User incentive focus                                                  |
|   60:40 split                    | $4.98/mo+               | High user attraction                                                  |
|                                                                                                                                     |
|   EARNINGS TIER RECOMMENDATIONS:                                                                                                    |
|   -------------------------------|-------------------------|-----------------------------------------------------------------------|
|   Poor ($4/mo)                   | 50:50                   | Only marginally profitable                                            |
|   Average ($5/mo)                | 50:50 to 55:45          | All profitable                                                        |
|   Acceptable ($6/mo)             | 50:50 to 60:40          | All profitable                                                        |
|   OK ($9/mo)                     | 50:50 to 60:40          | All profitable                                                        |
|   Good ($12/mo)                  | 50:50 to 60:40          | Strong returns                                                        |
|   Very Good ($20/mo)             | 50:50 to 60:40          | Growth priority                                                       |
|   Excellent ($25/mo)             | 50:50 to 60:40          | Max user benefit                                                      |
|   Champion ($30/mo)              | 50:50 to 60:40          | Premium offering                                                      |
|                                                                                                                                     |
+-------------------------------------------------------------------------------------------------------------------------------------+
```

### Website Earnings Calculator Implementation

When building the earnings calculator for your website, include these split scenarios:

```javascript
// Earnings Calculator Logic
function calculateUserEarnings(avgMonthlyEarnings, splitRate) {
  const splits = {
    '50:50': 0.50,
    '52:48': 0.52,
    '55:45': 0.55,
    '60:40': 0.60
  };

  const userShare = avgMonthlyEarnings * splits[splitRate];
  return userShare;
}

function calculateOperatorNet(avgMonthlyEarnings, splitRate, licenseCount) {
  const splits = {
    '50:50': 0.50,
    '52:48': 0.48,
    '55:45': 0.45,
    '60:40': 0.40
  };

  const subscriptionCost = 1.99;
  const operatorShare = avgMonthlyEarnings * splits[splitRate];
  const netPerLicense = operatorShare - subscriptionCost;

  return netPerLicense * licenseCount;
}

// Break-even earnings by split rate
const breakEvenEarnings = {
  '50:50': 3.98,  // $1.99 / 0.50
  '52:48': 4.15,  // $1.99 / 0.48
  '55:45': 4.42,  // $1.99 / 0.45
  '60:40': 4.98   // $1.99 / 0.40
};
```

---

## Cost Summary

### Infrastructure Costs by Phase

| Phase | Monthly Infrastructure | Monthly AI/Tools | Total |
|-------|----------------------|------------------|-------|
| **MVP (Month 1-3)** | $15-25 | $0-50 | $15-75 |
| **Growth (Month 4-6)** | $25-50 | $100-150 | $125-200 |
| **Scale (Month 7-12)** | $200-350 | $200-400 | $400-750 |
| **Enterprise (Month 12+)** | $500-800 | $300-500 | $800-1,300 |

### Key Technical Decisions

1. **Start with Single Instance** - GCP e2-small is sufficient for MVP
2. **Use Rust/Leptos** - Performance benefits, low hosting costs
3. **Cloudflare Free Tier** - CDN and DDoS protection at no cost
4. **AI Tools Tier 1** - Start free, upgrade as revenue grows
5. **Manual Fiat** - Only for non-bank-withdrawal countries
6. **Discord-first Support** - Community-driven, low cost

---

*Document Version: 2.0*
*Last Updated: Earnings scenarios updated to 8-tier system ($4-30/mo)*
*Source: Marketing Implementation Guide v5.1*
*Audience: Technical Team / DevOps*
