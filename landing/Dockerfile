# syntax=docker/dockerfile:1.7

# ── Stage 1: build the Astro site ──────────────────────────────────────────────
FROM node:22-alpine AS builder
WORKDIR /app

# Install deps with a stable cache layer
COPY package.json package-lock.json ./
RUN npm ci --no-audit --no-fund

# Build-time env: Umami website id is baked into the static output.
# Leave empty to ship without analytics.
ARG PUBLIC_UMAMI_WEBSITE_ID=""
ENV PUBLIC_UMAMI_WEBSITE_ID=$PUBLIC_UMAMI_WEBSITE_ID

# Copy the rest and build
COPY . .
RUN npm run build

# ── Stage 2: serve dist/ via nginx ─────────────────────────────────────────────
FROM nginx:1.27-alpine AS runtime

# Custom server config (cache headers, 404 fallback, byte-range for mp4)
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Static assets emitted by Astro
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD wget -q --spider http://127.0.0.1/index.html || exit 1
