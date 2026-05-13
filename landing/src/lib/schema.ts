import faq from '../content/faq.json';

export const softwareApplicationSchema = {
  '@context': 'https://schema.org',
  '@type': 'SoftwareApplication',
  name: 'TOFA',
  operatingSystem: 'macOS, Linux',
  applicationCategory: 'SecurityApplication',
  description: 'Offline, open-source 2FA in your terminal and macOS menu bar. Imports from Authy, Google Authenticator, Aegis, 2FAS, 1Password and more.',
  offers: { '@type': 'Offer', price: '0', priceCurrency: 'USD' },
  downloadUrl: 'https://github.com/stratif-io/tofa/releases',
  license: 'https://opensource.org/licenses/MIT',
  author: { '@type': 'Person', name: 'Carlo Abi Chahine' },
  publisher: { '@type': 'Organization', name: 'Stratif', url: 'https://stratif.io' },
} as const;

export const faqPageSchema = {
  '@context': 'https://schema.org',
  '@type': 'FAQPage',
  mainEntity: faq.entries.map((e) => ({
    '@type': 'Question',
    name: e.q,
    acceptedAnswer: { '@type': 'Answer', text: e.a },
  })),
} as const;

export const organizationSchema = {
  '@context': 'https://schema.org',
  '@type': 'Organization',
  name: 'Stratif',
  url: 'https://stratif.io',
  logo: 'https://tofa.stratif.io/favicon.svg',
} as const;
