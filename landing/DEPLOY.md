# Deploying `tofa.stratif.io`

The landing site ships as a **Docker image** on GHCR. The image bundles the
Astro build with a tiny nginx that serves static files. The OVH server runs
the container on the shared `stratifio-net` Docker network; the host Caddy
(provisioned by `papa-data-infra`) reverse-proxies `tofa.stratif.io` to it.

## Flow

1. Push to `main` with paths under `landing/**` (or run the workflow manually).
2. `.github/workflows/landing-deploy.yml` builds the image and pushes to GHCR
   (`ghcr.io/stratif-io/tofa-landing:<sha>` + `:latest`).
3. The same workflow SSHes into the OVH server, pulls the new image,
   stops/removes the old `tofa-landing` container, and starts a new one
   attached to `stratifio-net`.
4. Caddy (managed by `papa-data-infra`) already routes
   `tofa.stratif.io` → `tofa-landing:80`. New requests hit the new container
   instantly.

## Secrets required (GitHub Actions on this repo)

| Secret | Purpose |
|---|---|
| `OVH_SSH_PRIVATE_KEY` | SSH key for `ubuntu@ns3150446.ip-51-83-100.eu` |
| `TOFA_UMAMI_WEBSITE_ID` | Umami site id (UUID) for `tofa.stratif.io`; baked into the image at build time as `PUBLIC_UMAMI_WEBSITE_ID`. (Named `TOFA_…` to avoid colliding with the `UMAMI_WEBSITE_ID` secret already in use on other repos.) Leave unset to ship without analytics. |

Sync from Bitwarden via `papa-data-infra`'s `scripts/sync-secrets.sh`.

The built-in `GITHUB_TOKEN` handles GHCR auth — no manual registry setup.

## Umami analytics

Visits and click events are reported to the self-hosted Umami at
`analytics.stratif.io` (which already runs on the same OVH box and is wired
into Caddy with public bypasses on `/script.js` and `/api/send`).

**Setup (one-time):**

1. Open `https://analytics.stratif.io`, log in, and add a website for
   `tofa.stratif.io`. Copy the generated **Website ID** (a UUID).
2. Set it as a GitHub Actions secret on this repo: `TOFA_UMAMI_WEBSITE_ID`.
   (The `TOFA_` prefix avoids collision with the `UMAMI_WEBSITE_ID` secret
   already in use on other repos that track different sites.)
3. The next deploy bakes it into the image as `PUBLIC_UMAMI_WEBSITE_ID`
   (Astro env). The build emits the `<script>` tag only when the var is
   present, so local dev / unconfigured environments ship without tracking.

**Tracked events** (via `data-umami-event` attributes; visible under
*Events* in the Umami dashboard):

| Event | Where |
|---|---|
| `cta-hero-install`, `cta-hero-demos` | Hero CTAs |
| `link-topbar-github`, `link-footer-{docs,install,releases,github,discussions,contribute,license}` | nav + footer |
| `install-tab-{macos,linux,cargo,source}` · `install-copy-{macos,linux,cargo,source}` | Install section |
| `demo-{tour,scan-cam,app}` | Demo gallery clicks |
| `faq-open-{1..9}` | FAQ accordion items |
| `link-security-threat-model`, `link-import-docs`, `link-import-issue`, `link-install-unsigned-build` | inline prose links |

## One-time GHCR visibility

After the first push, set the package visibility to **public** at
https://github.com/orgs/stratif-io/packages/container/tofa-landing/settings
so the OVH server can pull without authenticating.

## Running the image locally

```bash
cd landing
make docker-run          # → http://localhost:8080
make docker-stop
```

## Reverting

Re-run the workflow against an earlier commit:

```bash
gh workflow run "Landing · deploy" --repo stratif-io/tofa --ref <earlier-sha>
```

Or SSH and pull a previous tag manually:

```bash
ssh ubuntu@ns3150446.ip-51-83-100.eu
docker pull ghcr.io/stratif-io/tofa-landing:<old-sha>
docker rm -f tofa-landing
docker run -d --name tofa-landing --network stratifio-net \
  --restart unless-stopped \
  ghcr.io/stratif-io/tofa-landing:<old-sha>
```

## Image internals

- **Builder stage**: `node:20-alpine`, runs `npm ci && npm run build`.
- **Runtime stage**: `nginx:1.27-alpine`, serves `dist/` from `/usr/share/nginx/html`.
- **Nginx config** (`landing/nginx.conf`): immutable caching for `/_astro/*`,
  byte-range streaming for `*.mp4`, short TTL on HTML, SPA-style 404 fallback.
- The outer Caddy still handles TLS, HSTS, and security headers — nginx just
  serves bytes.

## Verifying a deploy

After the workflow completes:

```bash
curl -sI https://tofa.stratif.io/ | head
curl -sI https://tofa.stratif.io/og.png | head
curl -s  https://tofa.stratif.io/sitemap-index.xml | head -5
curl -s  https://tofa.stratif.io/robots.txt
```
