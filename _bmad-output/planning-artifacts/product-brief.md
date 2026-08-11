# Product Brief: Cosy

*One-pager. Full detail in PRD (`prd.md`) and Architecture (`architecture.md`).*

## What

Cosy (Content Easy) is a Rust-native CLI tool that generates marketing visuals — carousels, OG images, social cards — from JSON + SVG templates at ~50ms/slide. Single ~5MB binary, no browser dependency.

## Why

Current image generation for CodeCora marketing uses Playwright + headless Chromium: 2-5 seconds per slide, 500MB+ runtime dependency. Cosy replaces this with a pure-Rust pipeline that is 40-100x faster and deploys anywhere (no Chromium install needed).

## For Whom

1. **CodeCora** (internal): replaces Playwright pipeline for all marketing visual generation
2. **Indie SaaS founders / dev-tool builders** (market): programmatic image generation without SaaS pricing ($49-149/mo competitors)
3. **AI agents** (Hermes ecosystem): JSON-to-PNG subprocess for automated content pipelines

## Key Differentiators

| | Cosy | Canva | Bannerbear | Playwright |
|---|---|---|---|---|
| Speed | ~50ms/slide | Manual | 1-3s/image | 2-5s/slide |
| Dependency | 5MB binary | Browser app | SaaS API | 500MB Chromium |
| Scriptable | ✅ JSON+CLI | ❌ GUI only | ✅ API | ✅ Code |
| Cost | Free (OSS) | $15/mo | $49-149/mo | Free (infra cost) |
| Self-hosted | ✅ Binary | ❌ | ❌ | ✅ Heavy |

## Business Model (CFO Recommendation)

- **Phase 1-3**: Free CLI (OSS), internal use only. Dogfooding.
- **Phase 4**: SaaS API launch. $19-99/mo tiers. 40-60% below Bannerbear/Placid.
- **Phase 5**: Template marketplace. 70/30 creator split.
- **License**: BSL 1.1 (converts to Apache 2.0 after 4 years). Prevents cloud fork, allows adoption.
- **Path to $1K MRR**: ~37 paying customers (6-8 months post-SaaS launch)
- **Infra cost**: ~$0 on Oracle Cloud free tier (288K renders/day capacity)

## MVP Scope

- `cosy render` + `cosy templates list` commands
- Template engine: JSON schema validation + brand field defaults
- Rendering: minijinja SVG → resvg → PNG (multi-slide + single-slide)
- 3 starter templates: carousel-default, social-quote, og-image
- Bundled fonts: Inter + JetBrains Mono
- Canvas: 1080×1350, 2x retina output
- **Timeline**: 5-7 dev days

## Non-Goals (v1)

No GUI, no SaaS API, no template marketplace, no animation/video, no image editing.

## Risks

- **Technical moat is LOW** (CTO). Rendering engine alone is copyable. Real moat = template quality + AI agent integration lock-in.
- **Template maintenance** = hidden cost (COO). Start with 10 curated templates, not open marketplace.
- **Solo dev bandwidth** (COO): 30% allocation = 7 weeks to public launch. Realistic but tight.

## Next Steps

1. ✅ Repo created: `github.com/codecoradev/cosy` (private)
2. ✅ BMAD installed, PRD + Architecture drafted
3. ⏳ Build Phase 1 (PoC): core pipeline with 1 template
4. ⏳ Build Phase 2 (CLI MVP): 3 templates, full CLI
5. ⏳ Internal dogfooding (Phase 3)
6. ⏳ GitHub issues + milestones
