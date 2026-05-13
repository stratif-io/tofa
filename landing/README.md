# tofa.stratif.io — landing page

Astro + React + Tailwind + shadcn/ui static site. Deployed manually to a self-hosted server.

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
make deploy         # rsync dist/ to $TOFA_DEPLOY_HOST
```

## Where stuff lives

- `src/pages/index.astro` — composes the page.
- `src/components/*.astro` — static sections.
- `src/components/*.tsx` — React islands (theme toggle, demos, install, FAQ, copy).
- `src/styles/tokens.css` — design tokens (mirror of `docs/design/assets/css/tokens.css`).
- `src/content/faq.json` — FAQ entries (drives both UI and JSON-LD schema).
- `public/demos/` — `.mp4` + `-poster.png` for each demo.
- `scripts/deploy.sh` — rsync template.
- `DEPLOY.md` — server config + DNS.
