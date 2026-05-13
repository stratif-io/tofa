export interface SeoProps {
  title?: string;
  description?: string;
  canonical?: string;
  ogImage?: string;
}

export const SITE = {
  url: 'https://tofa.stratif.io',
  name: 'TOFA',
  defaultTitle: 'TOFA — Offline TOTP & 2FA for your terminal and menu bar. Open-source Authy alternative.',
  defaultDescription: 'TOFA is an offline, open-source TOTP authenticator in your terminal and macOS menu bar. The 2FA app that imports OTP secrets from Authy, Google Authenticator, Aegis, 2FAS, 1Password and 6 more. MIT, no account, no cloud.',
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
