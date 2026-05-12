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

release-please manages **`tofa-core`** and **`tofa-macos`** (`tofa-app/src-tauri`) only. The `tofa` CLI is **not** tracked by release-please — `linked-versions` was removed because it groups release PRs without actually locking versions, which caused an empty-release-PR loop.

### Releasing the `tofa` CLI

1. Bump `version` in `tofa/Cargo.toml`.
2. Update `tofa/CHANGELOG.md` by hand.
3. Commit, merge to `main`, then tag and push:
   ```
   git tag vX.Y.Z
   git push origin vX.Y.Z
   ```
4. `release.yml` builds CLI binaries + dispatches the Homebrew formula update.

### When `tofa-core` bumps

release-please **does not** update the `tofa-core` dep constraint in `tofa/Cargo.toml` anymore. After a `tofa-core` release lands, bump the constraint manually in the same commit or a follow-up:

```
tofa-core = { path = "../tofa-core", version = "X.Y.Z" }
```
