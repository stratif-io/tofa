'use strict';

// === Issuer brand icons ===
//
// Maps a normalised issuer name (lowercase, alphanumerics only) to the
// corresponding sprite symbol id and a tint colour.
//
// Sources:
//  - SVG paths: simpleicons.org (CC0), bundled into assets/svg/sprite.svg
//  - Default colours: simpleicons brand hex
//  - Overrides: a few entries use a brighter alternative when the upstream
//    brand colour is too dark (#000-ish) to read on the dark theme.
//
// To add an icon:
//  1. Drop the upstream 24x24 SVG path into sprite.svg as
//     <symbol id="issuer-<slug>" viewBox="0 0 24 24"><path d="…"/></symbol>
//  2. Add an entry to ICONS below keyed by the lowercase alphanumeric slug.
//  3. (Optional) Add an alias in ALIASES if users may type the issuer
//     differently.

const ICONS = {
  github:           { id: 'issuer-github',          color: '#FFFFFF' }, // brand #181717 too dark
  google:           { id: 'issuer-google',          color: '#4285F4' },
  microsoft:        { id: 'issuer-microsoft',       color: '#00A4EF' },
  amazonaws:        { id: 'issuer-amazonaws',       color: '#FF9900' }, // AWS orange (brand hex too dark)
  apple:            { id: 'issuer-apple',           color: '#FFFFFF' }, // brand #000 too dark
  x:                { id: 'issuer-x',               color: '#FFFFFF' }, // brand #000 too dark
  facebook:         { id: 'issuer-facebook',        color: '#0866FF' },
  discord:          { id: 'issuer-discord',         color: '#5865F2' },
  slack:            { id: 'issuer-slack',           color: '#4A154B' },
  dropbox:          { id: 'issuer-dropbox',         color: '#0061FF' },
  cloudflare:       { id: 'issuer-cloudflare',      color: '#F38020' },
  linkedin:         { id: 'issuer-linkedin',        color: '#0A66C2' },
  gitlab:           { id: 'issuer-gitlab',          color: '#FC6D26' },
  bitbucket:        { id: 'issuer-bitbucket',       color: '#0052CC' },
  vercel:           { id: 'issuer-vercel',          color: '#FFFFFF' }, // brand #000 too dark
  notion:           { id: 'issuer-notion',          color: '#FFFFFF' }, // brand #000 too dark
  paypal:           { id: 'issuer-paypal',          color: '#003087' },
  coinbase:         { id: 'issuer-coinbase',        color: '#0052FF' },
  npm:              { id: 'issuer-npm',             color: '#CB3837' },
  docker:           { id: 'issuer-docker',          color: '#2496ED' },
  reddit:           { id: 'issuer-reddit',          color: '#FF4500' },
  twitch:           { id: 'issuer-twitch',          color: '#9146FF' },
  shopify:          { id: 'issuer-shopify',         color: '#7AB55C' },
  stripe:           { id: 'issuer-stripe',          color: '#635BFF' },
  atlassian:        { id: 'issuer-atlassian',       color: '#0052CC' },
  heroku:           { id: 'issuer-heroku',          color: '#79589F' },
  digitalocean:     { id: 'issuer-digitalocean',    color: '#0080FF' },
  wordpress:        { id: 'issuer-wordpress',       color: '#21759B' },
  bitwarden:        { id: 'issuer-bitwarden',       color: '#175DDC' },
  protonmail:       { id: 'issuer-protonmail',      color: '#6D4AFF' },
  mailchimp:        { id: 'issuer-mailchimp',       color: '#FFE01B' },
  adobe:            { id: 'issuer-adobe',           color: '#FF0000' },
};

// Common alternative names users may enter. Mapped to the canonical slug
// in ICONS. Match is case-insensitive and ignores non-alphanumerics.
const ALIASES = {
  aws:                  'amazonaws',
  amazon:               'amazonaws',
  amazonwebservices:    'amazonaws',
  twitter:              'x',
  meta:                 'facebook',
  microsoft365:         'microsoft',
  office365:            'microsoft',
  azure:                'microsoft',
  proton:               'protonmail',
  protonvpn:            'protonmail',
  gcp:                  'google',
  googlecloud:          'google',
  gmail:                'google',
  googleworkspace:      'google',
};

function normaliseSlug(s) {
  return String(s || '').toLowerCase().replace(/[^a-z0-9]+/g, '');
}

function iconForIssuer(issuer) {
  const slug = normaliseSlug(issuer);
  if (!slug) return null;
  const key = ALIASES[slug] || slug;
  return ICONS[key] || null;
}

window.IssuerIcons = { iconForIssuer };
