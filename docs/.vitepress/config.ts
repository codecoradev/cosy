import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Cosy',
  description: 'Template-based image generation in Rust',
  lang: 'en',
  base: '/cosy/',
  cleanUrls: true,
  lastUpdated: true,
  ignoreDeadLinks: true,

  head: [
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:title', content: 'Cosy — Template-based Image Generation' }],
    ['meta', { property: 'og:description', content: 'Generate social media images from JSON templates. CLI + HTTP API. Rust-powered. 148 templates.' }],
    ['meta', { name: 'twitter:card', content: 'summary_large_image' }],
  ],

  themeConfig: {
    siteTitle: 'Cosy',
    socialLinks: [
      { icon: 'github', link: 'https://github.com/codecoradev/cosy' },
    ],

    nav: [
      { text: 'Guide', link: '/guide/getting-started' },
      { text: 'Templates', link: '/templates/' },
      { text: 'API', link: '/api/' },
    ],

    sidebar: {
      '/guide/': [
        {
          text: 'Getting Started',
          items: [
            { text: 'Installation', link: '/guide/getting-started' },
            { text: 'Quick Start', link: '/guide/quick-start' },
            { text: 'CLI Reference', link: '/guide/cli' },
          ],
        },
        {
          text: 'Guides',
          items: [
            { text: 'HTTP Server', link: '/guide/server' },
            { text: 'Docker', link: '/guide/docker' },
            { text: 'Template Authoring', link: '/guide/template-authoring' },
          ],
        },
        {
          text: 'Reference',
          items: [
            { text: 'Configuration', link: '/guide/configuration' },
            { text: 'Branding', link: '/guide/branding' },
          ],
        },
      ],
      '/templates/': [
        {
          text: 'Overview',
          items: [
            { text: 'All Templates (148)', link: '/templates/' },
          ],
        },
        {
          text: 'Social Media',
          collapsed: true,
          items: [
            { text: 'Social Quote', link: '/templates/social-quote' },            { text: 'Dev Quote', link: '/templates/dev-quote' },            { text: 'Twitter/X Quote', link: '/templates/twitter-quote' },            { text: 'TikTok Quote', link: '/templates/tiktok-quote' },            { text: 'Instagram Story', link: '/templates/instagram-story' },            { text: 'LinkedIn Card', link: '/templates/linkedin-card' },            { text: 'Pinterest Pin', link: '/templates/pinterest-pin' },            { text: 'Social Poll', link: '/templates/social-poll' },            { text: 'Tweet Embed', link: '/templates/tweet-embed' }
          ],
        },
        {
          text: 'OG & Web',
          collapsed: true,
          items: [
            { text: 'OG Image', link: '/templates/og-image' },            { text: 'Blog Hero', link: '/templates/blog-hero' },            { text: 'Newsletter Header', link: '/templates/newsletter-header' },            { text: 'Newsletter CTA', link: '/templates/newsletter-cta' },            { text: 'Author Bio', link: '/templates/author-bio' },            { text: 'Archive Card', link: '/templates/archive-card' },            { text: 'FAQ Card', link: '/templates/faq-card' },            { text: 'FAQ Single', link: '/templates/faq-single' },            { text: 'Error 404', link: '/templates/error-404' }
          ],
        },
        {
          text: 'Stats & Metrics',
          collapsed: true,
          items: [
            { text: 'Stat Card', link: '/templates/stat-card' },            { text: 'Stat Comparison', link: '/templates/stat-comparison' },            { text: 'Stat Highlight', link: '/templates/stat-highlight' },            { text: 'Stats Dashboard', link: '/templates/stats-dashboard' },            { text: 'Metric Grid', link: '/templates/metric-grid' },            { text: 'Big Number', link: '/templates/big-number' },            { text: 'Data Counter', link: '/templates/data-counter' },            { text: 'Bar Chart', link: '/templates/chart-bar' }
          ],
        },
        {
          text: 'DevOps & Infra',
          collapsed: true,
          items: [
            { text: 'API Endpoint', link: '/templates/api-endpoint' },            { text: 'API Response', link: '/templates/api-response' },            { text: 'API Key Display', link: '/templates/api-key-display' },            { text: 'Health Check', link: '/templates/health-check' },            { text: 'Rate Limit', link: '/templates/rate-limit' },            { text: 'Status Badge', link: '/templates/status-badge' },            { text: 'Uptime Monitor', link: '/templates/uptime-monitor' },            { text: 'Incident Report', link: '/templates/incident-report' },            { text: 'Deploy Status', link: '/templates/deploy-status' },            { text: 'Migration Status', link: '/templates/migration-status' },            { text: 'Security Advisory', link: '/templates/security-advisory' }
          ],
        },
        {
          text: 'Code Snippets',
          collapsed: true,
          items: [
            { text: 'Code Snippet', link: '/templates/code-snippet' },            { text: 'Rust Snippet', link: '/templates/rust-snippet' },            { text: 'CSS Snippet', link: '/templates/css-snippet' },            { text: 'Go Snippet', link: '/templates/snippet-go' },            { text: 'Python Snippet', link: '/templates/snippet-python' },            { text: 'TypeScript Snippet', link: '/templates/snippet-typescript' },            { text: 'Rust Snippet', link: '/templates/snippet-rust' },            { text: 'Bash Snippet', link: '/templates/snippet-bash' },            { text: 'SQL Snippet', link: '/templates/snippet-sql' },            { text: 'YAML Snippet', link: '/templates/snippet-yaml' },            { text: 'Terminal Output', link: '/templates/terminal-output' },            { text: 'Terminal Prompt', link: '/templates/terminal-prompt' },            { text: 'Command Card', link: '/templates/command-card' },            { text: 'Command Result', link: '/templates/command-result' },            { text: 'ASCII Art', link: '/templates/ascii-art' }
          ],
        },
        {
          text: 'GitHub & Dev',
          collapsed: true,
          items: [
            { text: 'GitHub README Banner', link: '/templates/github-readme' },            { text: 'GitHub Issue', link: '/templates/github-issue' },            { text: 'GitHub PR', link: '/templates/github-pr' },            { text: 'GitHub Profile', link: '/templates/github-profile' },            { text: 'GitHub Star', link: '/templates/github-star' },            { text: 'Contributor Card', link: '/templates/contributor-card' },            { text: 'Release Tag', link: '/templates/release-tag' },            { text: 'Release Notes', link: '/templates/release-notes' },            { text: 'Changelog', link: '/templates/changelog' },            { text: 'Changelog Entry', link: '/templates/changelog-entry' },            { text: 'Pull Request', link: '/templates/pull-request' },            { text: 'Code Review', link: '/templates/code-review' },            { text: 'Dependency Graph', link: '/templates/dependency-graph' },            { text: 'Feature Flags', link: '/templates/feature-flags' },            { text: 'Kudos Card', link: '/templates/kudos-card' },            { text: 'Star Tracker', link: '/templates/star-tracker' }
          ],
        },
        {
          text: 'CI/CD',
          collapsed: true,
          items: [
            { text: 'CI Pipeline', link: '/templates/ci-pipeline' },            { text: 'Package JSON', link: '/templates/package-json' },            { text: 'Dockerfile', link: '/templates/dockerfile' },            { text: 'Docker Command', link: '/templates/docker-command' },            { text: 'Makefile', link: '/templates/makefile' },            { text: 'Environment Setup', link: '/templates/env-setup' },            { text: 'Config Display', link: '/templates/config-display' },            { text: 'Database Schema', link: '/templates/database-schema' },            { text: 'Log Viewer', link: '/templates/log-viewer' },            { text: 'Certificate Pinned', link: '/templates/cert-pinned' },            { text: 'License Card', link: '/templates/license-card' }
          ],
        },
        {
          text: 'Design',
          collapsed: true,
          items: [
            { text: 'Color Palette', link: '/templates/color-palette' },            { text: 'Token Display', link: '/templates/token-display' },            { text: 'Gradient Mesh', link: '/templates/gradient-mesh' },            { text: 'Gradient Banner', link: '/templates/gradient-banner' },            { text: 'Gradient Hero', link: '/templates/gradient-hero' },            { text: 'Gradient Quote', link: '/templates/gradient-quote' },            { text: 'Feature Grid', link: '/templates/feature-grid' },            { text: 'Feature Highlight', link: '/templates/feature-highlight' },            { text: 'Feature List', link: '/templates/feature-list' },            { text: 'Wallpaper Quote', link: '/templates/wallpaper-quote' },            { text: 'Minimal Text', link: '/templates/minimal-text' }
          ],
        },
        {
          text: 'Marketing',
          collapsed: true,
          items: [
            { text: 'Announcement', link: '/templates/announcement' },            { text: 'Event Banner', link: '/templates/event-banner' },            { text: 'Event Flyer', link: '/templates/event-flyer' },            { text: 'CTA Card', link: '/templates/cta-card' },            { text: 'Pricing Card', link: '/templates/pricing-card' },            { text: 'Discount Badge', link: '/templates/discount-badge' },            { text: 'Retro Badge', link: '/templates/retro-badge' },            { text: 'Credit Badge', link: '/templates/credit-badge' },            { text: 'Sponsor Card', link: '/templates/sponsor-card' },            { text: 'Leaderboard', link: '/templates/leaderboard' },            { text: 'Benchmark Result', link: '/templates/benchmark-result' },            { text: 'Version Compare', link: '/templates/version-compare' }
          ],
        },
        {
          text: 'Content',
          collapsed: true,
          items: [
            { text: 'Carousel Default', link: '/templates/carousel-default' },            { text: 'Checklist', link: '/templates/checklist' },            { text: 'Comparison', link: '/templates/comparison' },            { text: 'Comparison Table', link: '/templates/comparison-table' },            { text: 'Numbered Tips', link: '/templates/numbered-tips' },            { text: 'Step Process', link: '/templates/step-process' },            { text: 'Timeline', link: '/templates/timeline' },            { text: 'Roadmap Card', link: '/templates/roadmap-card' },            { text: 'Roadmap Timeline', link: '/templates/roadmap-timeline' },            { text: 'Progress Card', link: '/templates/progress-card' },            { text: 'Goal Tracker', link: '/templates/goal-tracker' },            { text: 'Quick Tip', link: '/templates/quick-tip' },            { text: 'Quote Minimal', link: '/templates/quote-minimal' },            { text: 'Quote Stack', link: '/templates/quote-stack' },            { text: 'Polaroid Quote', link: '/templates/polaroid-quote' },            { text: 'Review Card', link: '/templates/review-card' },            { text: 'Testimonial', link: '/templates/testimonial' },            { text: 'Testimonial Grid', link: '/templates/testimonial-grid' },            { text: 'Milestone Celebration', link: '/templates/milestone-celebration' }
          ],
        },
        {
          text: 'Cards & Misc',
          collapsed: true,
          items: [
            { text: 'Podcast Cover', link: '/templates/podcast-cover' },            { text: 'Podcast Episode', link: '/templates/podcast-episode' },            { text: 'YouTube Thumbnail', link: '/templates/youtube-thumb' },            { text: 'Book Cover', link: '/templates/book-cover' },            { text: 'Book Club', link: '/templates/book-club' },            { text: 'Recipe Card', link: '/templates/recipe-card' },            { text: 'Calendar Event', link: '/templates/calendar-event' },            { text: 'Week Schedule', link: '/templates/week-schedule' },            { text: 'Kanban Card', link: '/templates/kanban-card' },            { text: 'Matrix Card', link: '/templates/matrix-card' },            { text: 'Matrix Display', link: '/templates/matrix-display' },            { text: 'Certificate', link: '/templates/certificate' },            { text: 'QR Business Card', link: '/templates/qr-business-card' },            { text: 'QR Code Display', link: '/templates/qr-code-display' },            { text: 'Team Member', link: '/templates/team-member' },            { text: 'Flame Graph', link: '/templates/flame-graph' },            { text: 'Diff Viewer', link: '/templates/diff-viewer' },            { text: 'Git Diff', link: '/templates/git-diff' },            { text: 'Webhook Card', link: '/templates/webhook-card' },            { text: 'Webhook Payload', link: '/templates/webhook-payload' },            { text: 'VS Code Config', link: '/templates/vscode-config' },            { text: 'Password Generator', link: '/templates/password-generator' },            { text: 'Poll Result', link: '/templates/poll-result' },            { text: 'Dev Quote Card', link: '/templates/dev-quote-card' }
          ],
        },
        {
          text: 'Other',
          collapsed: true,
          items: [
            { text: 'Swagger Endpoint', link: '/templates/swagger-endpoint' },            { text: 'Tech Stack', link: '/templates/tech-stack' },            { text: 'Version Comparison', link: '/templates/version-comparison' }
          ],
        }
      ],
      '/api/': [
        {
          text: 'API Reference',
          items: [
            { text: 'Overview', link: '/api/' },
            { text: 'Authentication', link: '/api/authentication' },
            { text: 'Endpoints', link: '/api/endpoints' },
          ],
        },
      ],
    },

    footer: {
      message: 'Released under the BSL-1.0 License.',
      copyright: 'Copyright © 2026 PT Azfirazka Digital Kreatif',
    },

    outline: { level: [2, 3] },
    search: { provider: 'local' },
    lastUpdated: {
      text: 'Updated at',
    },
  },
})
