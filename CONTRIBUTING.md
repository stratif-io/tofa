# Contributing to TOFA

Thanks for your interest in TOFA. This file is short on purpose — most of
what you need is already in [`CLAUDE.md`](./CLAUDE.md) and the
[docs site](https://docs.tofa.stratif.io/).

## Reporting a bug

Open an issue at <https://github.com/stratif-io/tofa/issues/new>. Include
your OS, the TOFA version (`tofa --version`), and steps to reproduce.

For security issues, please use a **private** advisory:
<https://github.com/stratif-io/tofa/security/advisories/new>.

## Requesting a vendor migration

Want TOFA to import from an authenticator we don't yet support? Open an issue
with the label **migration** and include the export format (sample file, if
you can share one without secrets — anonymize first). The current matrix is
in the README.

## Pull requests

- Branch from `main` using the pattern `<type>/<short-description>` (e.g.
  `feat/import-1password`, `fix/list-truncation`, `docs/recipe-aegis`).
- Commits must follow [Conventional Commits](https://www.conventionalcommits.org).
  Allowed types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`,
  `ci`, `perf`, `build`.
- Run `cargo fmt && cargo clippy --all-targets -- -D warnings` before pushing.
- Add tests for new behavior. The CLI uses `assert_cmd`; the core uses
  standard `cargo test`.
- One logical change per PR. Bundle related fixes; split unrelated ones.

## Discussions

For open-ended questions, design ideas, or "is anyone using TOFA for X?",
open a [discussion](https://github.com/stratif-io/tofa/discussions) instead
of an issue.

## License

By contributing, you agree your work is released under the same MIT license
as the rest of the project.
