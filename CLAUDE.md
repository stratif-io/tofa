# Tofa — Claude Code Guidelines

## Commit messages

All commits must follow **Conventional Commits** (`https://www.conventionalcommits.org`):

```
<type>(<optional scope>): <description>

[optional body]

[optional footer]
```

Allowed types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `ci`, `perf`, `build`.

Examples:
- `feat(cli): add scan command for QR codes`
- `fix(core): handle missing id field in VaultEntry`
- `ci: split macOS and Linux jobs`
- `docs: update README with import formats`

Never use free-form commit messages like `"update stuff"` or `"wip"`.

## Branch names

All branches must follow the pattern `<type>/<short-description>`:

```
feat/qr-import
fix/list-display-name
ci/release-please-config
docs/readme-rewrite
refactor/remove-drag-drop
```

Types mirror the commit types above. Words separated by hyphens, all lowercase.
