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

Sync from Bitwarden via `papa-data-infra`'s `scripts/sync-secrets.sh`.

The built-in `GITHUB_TOKEN` handles GHCR auth — no manual registry setup.

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
