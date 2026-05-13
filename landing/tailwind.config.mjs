/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{astro,html,js,jsx,md,mdx,ts,tsx}'],
  darkMode: ['selector', '[data-theme="light"]'],
  theme: {
    extend: {
      colors: {
        bg:           'var(--bg)',
        'bg-elevated':'var(--bg-elevated)',
        'bg-sunken':  'var(--bg-sunken)',
        surface:      'var(--surface)',
        'surface-2':  'var(--surface-2)',
        border:       'var(--border)',
        'border-strong':'var(--border-strong)',
        text:         'var(--text)',
        'text-muted': 'var(--text-muted)',
        'text-subtle':'var(--text-subtle)',
        brand:        'var(--brand)',
        'brand-bg':   'var(--brand-bg)',
        'brand-hover':'var(--brand-hover)',
        'on-brand':   'var(--on-brand)',
        success:      'var(--success)',
        warning:      'var(--warning)',
        danger:       'var(--danger)',
        accent:       'var(--accent)',
        purple: {
          50:'var(--purple-50)',100:'var(--purple-100)',200:'var(--purple-200)',
          300:'var(--purple-300)',400:'var(--purple-400)',500:'var(--purple-500)',
          600:'var(--purple-600)',700:'var(--purple-700)',800:'var(--purple-800)',
          900:'var(--purple-900)',
        },
        ink: {
          50:'var(--ink-50)',100:'var(--ink-100)',200:'var(--ink-200)',
          300:'var(--ink-300)',400:'var(--ink-400)',500:'var(--ink-500)',
          600:'var(--ink-600)',700:'var(--ink-700)',800:'var(--ink-800)',
          900:'var(--ink-900)',
        },
      },
      fontFamily: {
        display: ['var(--font-display)', 'Georgia', 'serif'],
        sans:    ['var(--font-body)',    'system-ui', 'sans-serif'],
        mono:    ['var(--font-mono)',    'SF Mono', 'monospace'],
      },
      borderRadius: { 'tofa-sm':'var(--r-sm)', 'tofa-md':'var(--r-md)', 'tofa-lg':'var(--r-lg)', 'tofa-xl':'var(--r-xl)' },
      transitionTimingFunction: { 'tofa-out':'var(--ease-out)', 'tofa-inout':'var(--ease-in-out)' },
    },
  },
  plugins: [],
};
