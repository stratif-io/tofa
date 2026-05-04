#!/bin/sh
set -e
hooks_dir="$(git rev-parse --git-common-dir)/hooks"
mkdir -p "$hooks_dir"
cp scripts/pre-commit "$hooks_dir/pre-commit"
chmod +x "$hooks_dir/pre-commit"
echo "Git hooks installed in $hooks_dir."
