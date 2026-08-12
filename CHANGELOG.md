# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **HTTP API server** (`cosy serve --port 3000`): RESTful endpoints for rendering images via HTTP
  - `GET /api/health` — health check with version and template count
  - `GET /api/templates` — list all available templates with dimensions
  - `POST /api/render` — render template with JSON data, returns PNG directly
- **15 new templates**: twitter-quote, linkedin-card, dev-quote, instagram-story, youtube-thumb, github-readme, tiktok-quote, newsletter-header, podcast-cover, event-banner, testimonial, stat-card, checklist, comparison, announcement
- **Gradient backgrounds** with customizable `bg_color`, `bg_color_end`, `accent_color`, `accent_color_end`
- **Image support**: `bg_image` (background texture/photo) and `logo` (brand watermark) fields
- **`bg_image_opacity`** field for controlling background image visibility (0.0-1.0)
- **Space Grotesk** display font for headlines (Medium, SemiBold, Bold)
- **Visual enhancements**: dots pattern, decorative corner circles, radial glow
- **`FieldType::Number` and `FieldType::Color`** variants in schema enum
- Conditional gradient overlay (0.7 opacity when bg_image present, 1.0 when not)

### Changed

- Removed CTA elements from all static image templates
- Background image layering: image renders below gradient overlay for text readability
- Podcast cover resized from 3000x3000 to 1080x1080 (render performance)

### Fixed

- Checklist template: hardcoded Y positions (minijinja arithmetic in loops unreliable)
- Template font-size regex corruption from batch enhancement script
