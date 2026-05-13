# Deploying `tofa.stratif.io`

Static HTML/CSS/JS in `dist/`. Any web server works. Two reference configs.

## Caddy (recommended)

`/etc/caddy/Caddyfile`:

```caddy
tofa.stratif.io {
  root * /var/www/tofa.stratif.io
  encode zstd gzip
  file_server

  @assets path /assets/* /_astro/*
  header @assets Cache-Control "public, max-age=31536000, immutable"

  @mp4 path *.mp4
  header @mp4 Accept-Ranges "bytes"

  @html path /index.html / /404.html
  header @html Cache-Control "public, max-age=3600"

  header Strict-Transport-Security "max-age=31536000; includeSubDomains; preload"
  header X-Content-Type-Options "nosniff"
  header Referrer-Policy "strict-origin-when-cross-origin"
}

www.tofa.stratif.io {
  redir https://tofa.stratif.io{uri} permanent
}
```

Reload: `sudo systemctl reload caddy`.

## nginx

`/etc/nginx/sites-enabled/tofa.stratif.io.conf`:

```nginx
server {
  listen 443 ssl http2;
  listen [::]:443 ssl http2;
  server_name tofa.stratif.io;

  ssl_certificate     /etc/letsencrypt/live/tofa.stratif.io/fullchain.pem;
  ssl_certificate_key /etc/letsencrypt/live/tofa.stratif.io/privkey.pem;

  root /var/www/tofa.stratif.io;
  index index.html;

  gzip on;
  gzip_types text/css application/javascript image/svg+xml application/json text/plain;
  add_header Strict-Transport-Security "max-age=31536000; includeSubDomains; preload" always;
  add_header X-Content-Type-Options "nosniff" always;

  location ~* \.(css|js|svg|png|webp|avif|woff2)$ { expires 1y; add_header Cache-Control "public, immutable"; }
  location ~* \.mp4$ { add_header Accept-Ranges bytes; }
  location = /index.html { add_header Cache-Control "public, max-age=3600"; }

  error_page 404 /404.html;
  location / { try_files $uri $uri/ =404; }
}

server {
  listen 80;
  listen [::]:80;
  server_name tofa.stratif.io www.tofa.stratif.io;
  return 301 https://tofa.stratif.io$request_uri;
}
```

Reload: `sudo nginx -t && sudo systemctl reload nginx`.

## DNS

- `tofa.stratif.io` A/AAAA → your server's public IP.
- `www.tofa.stratif.io` CNAME → `tofa.stratif.io` (or duplicate A/AAAA).

## Deploying

From your machine:

```bash
export TOFA_DEPLOY_USER=carlo
export TOFA_DEPLOY_HOST=tofa.stratif.io
export TOFA_DEPLOY_PATH=/var/www/tofa.stratif.io
cd landing
make deploy
```

The first deploy needs the target directory to exist with correct ownership:

```bash
ssh "$TOFA_DEPLOY_USER@$TOFA_DEPLOY_HOST" \
  "sudo mkdir -p $TOFA_DEPLOY_PATH && sudo chown $TOFA_DEPLOY_USER:$TOFA_DEPLOY_USER $TOFA_DEPLOY_PATH"
```

## TLS

Caddy provisions Let's Encrypt automatically the first time the site is hit.
For nginx, run `sudo certbot --nginx -d tofa.stratif.io -d www.tofa.stratif.io` once after DNS is pointed.

## Verification after deploy

```bash
curl -sI https://tofa.stratif.io/ | head
curl -s  https://tofa.stratif.io/sitemap-index.xml | head
curl -s  https://tofa.stratif.io/robots.txt
```

Expected: 200 on `/`, valid XML for the sitemap, robots.txt referencing the sitemap.
