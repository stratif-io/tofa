# tofa.stratif.io — landing page

Astro + React + Tailwind + shadcn/ui static site. Ships as a Docker image to
GHCR; the OVH server runs it on `stratifio-net` behind the shared Caddy
reverse-proxy. See [DEPLOY.md](./DEPLOY.md) for full operational details.

## Quick reference

```bash
make install        # npm ci
make build          # → dist/
make preview        # serve dist/ locally at :4321
make test           # vitest islands
make e2e            # playwright smoke
make lhci           # build + Lighthouse CI gates
make og             # regenerate Open Graph image (run when hero copy changes)
make posters        # regenerate demo poster PNGs (run when mp4s change)
make docker-build   # build the production image (tag tofa-landing:dev)
make docker-run     # build + run locally on :8080
make docker-stop    # stop + remove the local container
```

Deploys to production happen automatically when `main` is pushed with changes
under `landing/**`. See `.github/workflows/landing-deploy.yml`.

## Where stuff lives

- `src/pages/index.astro` — composes the page.
- `src/components/*.astro` — static sections.
- `src/components/*.tsx` — React islands (demos, install, FAQ, copy button).
- `src/styles/tokens.css` — design tokens (mirror of `docs/design/assets/css/tokens.css`).
- `src/content/faq.json` — FAQ entries (drives both UI and JSON-LD schema).
- `public/demos/` — `.mp4` + `-poster.png` for each demo.
- `Dockerfile` + `nginx.conf` — production image.
- `.github/workflows/landing-deploy.yml` — build, push, deploy.
- `DEPLOY.md` — full deploy + revert guide.
