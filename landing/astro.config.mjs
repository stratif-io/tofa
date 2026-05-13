import { defineConfig } from 'astro/config';
import react from '@astrojs/react';
import tailwind from '@astrojs/tailwind';

export default defineConfig({
  site: 'https://tofa.stratif.io',
  integrations: [
    react(),
    tailwind({ applyBaseStyles: false }),
  ],
  build: { format: 'directory' },
});
