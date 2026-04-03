# ADR-002: LGPD/GDPR compliance for the documentation site

**Date:** 2026-04-03
**Status:** Accepted
**Context:** Determine what the Ralph Engine documentation site needs for LGPD (Brazil) and GDPR (EU) compliance.

## Decision

Minimal compliance: Privacy Policy page only. No cookie banner, no consent mechanism.

## Current state (no action needed beyond Privacy Policy)

- **No cookies set** — static site, Cloudflare Pages basic plan does not set cookies
- **No analytics** — no tracking scripts, no user behavior collection
- **No forms** — no user input collected
- **No user accounts** — documentation site only
- **Pagefind search** — runs entirely client-side, no data sent to servers
- **Privacy Policy** — created at `site/src/content/docs/privacy.md` (EN + PT-BR)

## If analytics are added later

| Provider | Cookie banner needed? | Effort |
|----------|----------------------|--------|
| **Plausible** or **Fathom** | No — cookieless, GDPR-compliant by design | Update Privacy Policy only |
| **Google Analytics** | Yes — full consent banner, opt-in before script loads | High — cookie policy, consent UI, GTM config |

**Recommendation:** Use Plausible if analytics are needed. Avoids the cookie consent burden entirely.

## Cloudflare-specific notes

- Cloudflare Pages (static hosting) does **not** set cookies on visitors
- `__cf_bm` and `cf_clearance` cookies only appear when Bot Management or WAF challenges are active — not enabled on basic Pages plans
- Cloudflare processes IP addresses in CDN logs — disclosed in Privacy Policy with link to Cloudflare's own privacy policy

## What the Privacy Policy contains

- Statement that no personal data is collected
- Statement that no cookies are set
- Hosting provider disclosure (Cloudflare, US-based)
- Cloudflare CDN IP processing note
- Contact email for privacy inquiries
- MIT license reference for open source

## Research sources

- LGPD Art. 41 (DPO requirement — contact email suffices for zero-data sites)
- GDPR Art. 13/14 (transparency requirements — Privacy Policy satisfies)
- [Cloudflare Privacy Policy](https://www.cloudflare.com/privacypolicy/)
- Pagefind documentation (client-side only, no external requests)
