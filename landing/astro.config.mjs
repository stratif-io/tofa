import { defineConfig } from 'astro/config';
import react from '@astrojs/react';
import sitemap from '@astrojs/sitemap';

export default defineConfig({
  site: 'https://tofa.stratif.io',
  integrations: [
    react(),
    sitemap(),
  ],
  build: { format: 'directory' },
});
