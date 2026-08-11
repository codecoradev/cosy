---
title: Cosy MVP Scope — Final
created: 2026-08-11
updated: 2026-08-11
status: locked
decision_by: multi-agent consensus (CTO, CFO, CLO, COO, CMO) Round 3
---

# MVP Scope: Cosy
*Locked decisions from 3-round multi-agent discussion.*

## Executive Summary

Cosy MVP = **7-day build → 4-week internal dogfooding → 2-week closed alpha → Product Hunt launch Week 7.**

## MVP Definition (What "Done" Looks Like)

The MVP is complete when ALL of these work:

1. `cosy render --template social-quote --data input.json -o ./out.png` produces a visually correct 1080×1350 PNG in <100ms
2. `cosy render --template carousel --data input.json -o ./slides/` produces slide-01.png through slide-05.png
3. `echo '{"headline":"Hello"}' | cosy render --template og-image --stdin -o ./og.png` works (AI agent pipe)
4. `cosy render ... --json-output` prints `{"files":["out.png"],"render_time_ms":47}` to stdout
5. 3 templates bundled: og-image, social-quote, carousel-5
6. Cross-compiled binaries for Linux (x86_64 + ARM), macOS
7. CI green on push

## 7-Day Build Order

| Day | Deliverable | Issue |
|-----|-------------|-------|
| 1 | serde schema + minijinja integration + bundle Inter font | #3, #9 |
| 2 | SVG → resvg → PNG render pipeline | #1 |
| 3 | clap CLI: render + templates commands, --json/--stdin | #5, #10 |
| 4 | Template #1: og-image + Template #2: social-quote | #4, #7 |
| 5 | Template #3: carousel-5 (multi-slide stress test) | #6 |
| 6 | Schema validation + structured error JSON + --json-output | #2, #11 |
| 7 | CI cross-compile + README + examples + release binary | #12 |

## 4-Week Dogfooding Plan

| Week | Template | Replaces | Owner |
|------|----------|----------|-------|
| W1 | OG Image | Manual Canva blog OG | CTO builds, COO integrates Ghost |
| W2 | GitHub Social Preview | Manual Figma repo cards | CTO builds, COO applies to repos |
| W3 | 6-Slide Carousel | Manual Canva carousels | CTO builds, COO replaces Threads |
| W4 | Dev.to Banner + Newsletter Header | Manual Canva headers | CTO builds, COO swaps pipelines |

**Exit gate W4:** All 4 templates in production. Zero Canva for these assets.

## Kill / Pivot Criteria

**Two-strike rule:**
- **Strike 1** (Week 4): <10 internal renders/week → PIVOT (not kill). Pivot options: AI-agent-first positioning OR internal-only tool.
- **Strike 2** (Week 6): No external beta interest AND <10 renders/week → KILL.

**Success gate for public launch (Week 7):** 5+ closed-beta users actively rendering, 4 templates in production, CLI stable 7 consecutive days.

## Pricing (Post-MVP, Phase 4)

| Tier | Price | Includes |
|------|-------|----------|
| CLI (OSS) | Free | Engine + 5 basic templates, unlimited renders |
| Template Packs | $9-29/pack | 5 curated templates per pack (one-time) |
| SaaS API Starter | $19/mo | 1K renders/mo, API access |
| SaaS API Pro | $49/mo | 10K renders/mo, priority |
| SaaS API Scale | $99/mo | 50K renders/mo, SLA |

Bundle all 5 packs = $69 (save $18).

## License Decision (UNRESOLVED — needs user call)

The multi-agent discussion produced a split:

**Option A: BSL 1.1 (CTO Round 3 conceded, CFO)**
- Engine + CLI: BSL 1.1, Change Date 3 years, converts to Apache-2.0
- Templates: Proprietary
- SaaS API: Proprietary

**Option B: Apache-2.0 Split (CLO Round 3 final, CMO, CTO Round 2 original)**
- Engine + CLI: Apache-2.0
- Templates: Proprietary
- SaaS API: Proprietary
- Value capture doesn't depend on CLI license

**Current repo has BSL 1.1 committed.** CLO Round 3 final position recommends switching to Apache-2.0 split.

## Technical Stack (Locked)

- **Language:** Rust, edition 2021
- **Template engine:** minijinja (Jinja2-compatible)
- **Rendering:** resvg v0.48.1 → usvg → PNG
- **Fonts:** fontdb + bundled Inter Regular/Bold, JetBrains Mono Regular
- **Text:** textwrap crate for pre-SVG wrapping
- **CLI:** clap v4 (derive)
- **Schema:** serde + serde_json
- **Future SaaS:** axum + tokio

## Non-Goals (Explicitly Deferred)

- ❌ Template marketplace (Year 2 if demand exists)
- ❌ Visual/template editor
- ❌ Enterprise tier
- ❌ White-label
- ❌ AI template generation (premium feature, post-launch)
- ❌ Multi-language UI
