# Tofa — Claude Code Guidelines

## Files not to commit

`docs/superpowers/` contains AI planning specs and must never be committed — it is gitignored.

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

## Workflow

All feature work is done in **git worktrees** managed by the `superpowers:using-git-worktrees` skill. Never work directly on `main`. Each task gets its own worktree + branch, following the branch naming convention above.

## Releases

release-please manages **`tofa`**, **`tofa-core`**, and **`tofa-macos`** (`tofa-app/src-tauri`). Each is an independent package — no `linked-versions` grouping (that caused empty release PR loops).

- `tofa` tags as `vX.Y.Z` (no component prefix) — matches what `release.yml` and `publish-crates.yml` expect
- `tofa-core` tags as `tofa-core-vX.Y.Z`
- `tofa-macos` tags as `tofa-macos-vX.Y.Z`

### When `tofa-core` bumps

release-please does not update the `tofa-core` dep constraint in `tofa/Cargo.toml`. After a `tofa-core` release lands, bump it manually:

```
tofa-core = { path = "../tofa-core", version = "X.Y.Z" }
```
