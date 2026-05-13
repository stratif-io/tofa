export interface SeoProps {
  title?: string;
  description?: string;
  canonical?: string;
  ogImage?: string;
}

export const SITE = {
  url: 'https://tofa.stratif.io',
  name: 'TOFA',
  defaultTitle: 'TOFA — The 2FA app for your terminal & menu bar. Authy alternative, open source.',
  defaultDescription: 'Offline, open-source 2FA in your terminal and macOS menu bar. Import from Authy, Google Authenticator, Aegis, 2FAS, 1Password and 6 more. MIT, no account, no cloud.',
  ogImage: 'https://tofa.stratif.io/og.png',
  repo: 'https://github.com/stratif-io/tofa',
} as const;

export function resolveSeo(props: SeoProps = {}) {
  return {
    title:       props.title       ?? SITE.defaultTitle,
    description: props.description ?? SITE.defaultDescription,
    canonical:   props.canonical   ?? SITE.url,
    ogImage:     props.ogImage     ?? SITE.ogImage,
  };
}
