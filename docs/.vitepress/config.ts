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
    ['meta', { property: 'og:description', content: 'Generate social media images from JSON templates. CLI + HTTP API. Rust-powered.' }],
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
          text: 'Templates',
          items: [
            { text: 'Overview', link: '/templates/' },
            { text: 'Stat Card', link: '/templates/stat-card' },
            { text: 'Social Quote', link: '/templates/social-quote' },
            { text: 'Dev Quote', link: '/templates/dev-quote' },
            { text: 'Twitter/X Quote', link: '/templates/twitter-quote' },
            { text: 'TikTok Quote', link: '/templates/tiktok-quote' },
            { text: 'OG Image', link: '/templates/og-image' },
            { text: 'Announcement', link: '/templates/announcement' },
            { text: 'Carousel', link: '/templates/carousel-default' },
            { text: 'Checklist', link: '/templates/checklist' },
            { text: 'Comparison', link: '/templates/comparison' },
            { text: 'Event Banner', link: '/templates/event-banner' },
            { text: 'GitHub README', link: '/templates/github-readme' },
            { text: 'Instagram Story', link: '/templates/instagram-story' },
            { text: 'LinkedIn Card', link: '/templates/linkedin-card' },
            { text: 'Newsletter Header', link: '/templates/newsletter-header' },
            { text: 'Podcast Cover', link: '/templates/podcast-cover' },
            { text: 'Testimonial', link: '/templates/testimonial' },
            { text: 'YouTube Thumbnail', link: '/templates/youtube-thumb' },
          ],
        },
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
