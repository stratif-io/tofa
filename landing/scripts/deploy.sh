#!/usr/bin/env bash
set -euo pipefail

: "${TOFA_DEPLOY_USER:?Set TOFA_DEPLOY_USER (e.g. carlo)}"
: "${TOFA_DEPLOY_HOST:?Set TOFA_DEPLOY_HOST (e.g. tofa.stratif.io)}"
: "${TOFA_DEPLOY_PATH:?Set TOFA_DEPLOY_PATH (e.g. /var/www/tofa.stratif.io)}"

cd "$(dirname "$0")/.."

if [ ! -d dist ]; then
  echo "✗ dist/ not found — run 'npm run build' first."
  exit 1
fi

echo "→ Uploading dist/ to ${TOFA_DEPLOY_USER}@${TOFA_DEPLOY_HOST}:${TOFA_DEPLOY_PATH}/"
rsync -avz --delete \
    --exclude='.DS_Store' \
    dist/ "${TOFA_DEPLOY_USER}@${TOFA_DEPLOY_HOST}:${TOFA_DEPLOY_PATH}/"

echo "✓ Deployed to https://tofa.stratif.io/"
