# Changelog

## [0.12.1](https://github.com/stratif-io/tofa/compare/v0.12.0...v0.12.1) (2026-05-15)


### Bug Fixes

* **cli:** add #[must_use] to file_picker::filtered and fix release CI ([#133](https://github.com/stratif-io/tofa/issues/133)) ([b850e0a](https://github.com/stratif-io/tofa/commit/b850e0a94f8e926542d383acba29813d620ae304))

## [0.12.0](https://github.com/stratif-io/tofa/compare/v0.11.0...v0.12.0) (2026-05-15)


### Features

* **app:** redesign UI — settings page, nav bar, TOFA branding, icon regen ([aa7edf9](https://github.com/stratif-io/tofa/commit/aa7edf9900bfb568c393b4f85d80499c26db4436))
* **ci:** add crates.io publish workflow and complete crate metadata ([3428908](https://github.com/stratif-io/tofa/commit/34289083e9c917eab298df202d467b1fd74afbed))
* **ci:** crates.io publish workflow and complete crate metadata ([62a26a0](https://github.com/stratif-io/tofa/commit/62a26a0c177fe812a2e702abb2caf84d0d9b9845))
* **cli,tui,app:** route every import surface through the unified dispatcher ([5591dcb](https://github.com/stratif-io/tofa/commit/5591dcbfeb7e80d0b425ef8c045c0b9afa480e1c))
* **cli:** \`tofa qr --all --multi --output-dir <dir>\` writes one otpauth:// PNG per entry ([a9ecb02](https://github.com/stratif-io/tofa/commit/a9ecb02cb772911ba74a585ca5641c7947876bf0))
* **cli:** display entry id in list output ([ac7d16d](https://github.com/stratif-io/tofa/commit/ac7d16d59b50a18d1c63f330c95abc9ef6702c50))
* **cli:** export --format uris and code --uri ([7b6b44d](https://github.com/stratif-io/tofa/commit/7b6b44dca16d0fd3a4b921048df626eb0f137b1a))
* **cli:** scan all displays for QR codes, not just the main one ([fba719b](https://github.com/stratif-io/tofa/commit/fba719bd0bb9608aac025a872ceac33cbc45c233))
* **cli:** scan all displays for QR codes, not just the main one ([041f0c1](https://github.com/stratif-io/tofa/commit/041f0c187e2b71a7bcd22a71b5ffff2da6ee42f6))
* **cli:** show spinner during scan and silence screencapture stderr ([1021533](https://github.com/stratif-io/tofa/commit/1021533444030c566fe07a668e0a43bbc05254d8))
* **core,cli:** expose per-pass scan progress and surface it in the spinner ([08da8af](https://github.com/stratif-io/tofa/commit/08da8af76513c7df8ce05dc95dbcdf02e81e6703))
* **core:** build_selection_uri dispatches by selection shape ([6695d10](https://github.com/stratif-io/tofa/commit/6695d106b119d36fe3811034ce05569bc457c6f5))
* full-fidelity QR export rules across CLI, TUI, and Tauri app ([b0cb6de](https://github.com/stratif-io/tofa/commit/b0cb6decb0820186368619ff9f09d420c94b64ba))
* import dispatcher, URI export, dedup, and architectural audit pass ([9d8f2d4](https://github.com/stratif-io/tofa/commit/9d8f2d4432b59c2b43016e67792013a9c1be3cfa))
* multi-format import, UI redesign, Homebrew tap, TOFA branding ([30b7fd9](https://github.com/stratif-io/tofa/commit/30b7fd99276ead2450ee02e59304598b67fd1bfb))
* **tauri:** full Tauri app UI — loader, drag-drop, scan feedback, lock/unlock UX ([7d5fde0](https://github.com/stratif-io/tofa/commit/7d5fde0449aa31770a486d04083c095c7c105c40))
* **tofa/cli:** add brand colors, voice messages, and expires column to list output ([4809628](https://github.com/stratif-io/tofa/commit/48096280accd9a4804de72b896359da1bdbacc15))
* **tofa:** migrate TUI to tofa-theme palette — replace GitHub blue with Sir Wink purple ([e33edef](https://github.com/stratif-io/tofa/commit/e33edefb2d013454c9bfd4e96e845848ea0fdc25))
* **tui,app:** accept pasted list of otpauth:// URIs as bulk import ([0eb62a8](https://github.com/stratif-io/tofa/commit/0eb62a84f696b1ddc8272211fd490ac2b32e1dd8))
* **tui,app:** list-of-otpauth multi-QR export in TUI and Tauri app ([316fdce](https://github.com/stratif-io/tofa/commit/316fdce3cce3847591907e1c3c7b54347b39bad1))
* **tui:** brand-styled hints + fix Detail modal sizing for the URI row ([09a9be3](https://github.com/stratif-io/tofa/commit/09a9be361f632cf95e4c812be0e2c85b3d62c006))
* **tui:** multi-file selection in the picker ([33f5486](https://github.com/stratif-io/tofa/commit/33f548632b9816d9405d418fb7590670adc3fa40))
* **tui:** show otpauth URI on the detail screen ([9f7368b](https://github.com/stratif-io/tofa/commit/9f7368bf8c827f68aad2fa0076598b0ed7d65aa6))
* **tui:** u in export shows URIs; r in view shows the entry's QR ([40472e8](https://github.com/stratif-io/tofa/commit/40472e8f805583fd1472bc4c4d9ccf7d9db071e3))
* **tui:** u to copy entry URI; u in export to save URI list to .txt ([c5fe690](https://github.com/stratif-io/tofa/commit/c5fe6905fb7827637b4862c73bcdd7554e796c0a))


### Bug Fixes

* **app:** vendor jsQR locally to remove CDN dependency ([5f9fc36](https://github.com/stratif-io/tofa/commit/5f9fc367247f596cece37e7cceaf7d06dbf0123d))
* **ci:** resolve merge conflict — use version 0.2.1 from main ([be001d6](https://github.com/stratif-io/tofa/commit/be001d60598077c46bc232500886c73c09f7f77d))
* **cli:** check vault exists before prompting for passphrase ([bb13511](https://github.com/stratif-io/tofa/commit/bb135115dfa7dabf6d8f8a323577b191c728e290))
* **cli:** dedup imports in `add`, `cam`, and `scan` ([a39dd77](https://github.com/stratif-io/tofa/commit/a39dd771d6bede3e25fb9e255b7aafef8420cc46))
* **cli:** emit full otpauth URI from `tofa qr <name>` ([305068a](https://github.com/stratif-io/tofa/commit/305068ad5548e79f8d77a5852f1e3f675054fdbe))
* **cli:** use format_code for list --codes, fixing 8-digit split to 4+4 ([61effe0](https://github.com/stratif-io/tofa/commit/61effe06fc34288c3d03800a321baf9b358ab834))
* **cli:** use tofa_core::qr::parse_json_bytes for import command ([21413fd](https://github.com/stratif-io/tofa/commit/21413fde6bc5fb6a314a5a852315bea033b008fc))
* **core,app:** centralise dedup in Vault::add_entry_if_unique ([7b97628](https://github.com/stratif-io/tofa/commit/7b97628395be94064ffb12aafd719561b1dbd52b))
* **core:** preserve per-account algorithm and digits in migration export ([49641a8](https://github.com/stratif-io/tofa/commit/49641a8dcd0ca1885db7d9c56f471301c6b2a7ff))
* **landing:** use node:22-alpine in Dockerfile for Astro v6 ([1ed6256](https://github.com/stratif-io/tofa/commit/1ed6256a0bb426b35f01ee56155f663c97ae4fd1))
* **release:** bump tofa-core to 0.4.0 and tofa to 0.5.0 ([ee895da](https://github.com/stratif-io/tofa/commit/ee895da58bc6d81bd52f4d670a002ea7bfc45f88))
* repair broken 0.4.0 publish, harden release pipeline, preserve OTP params in migration export ([654a6a9](https://github.com/stratif-io/tofa/commit/654a6a910f1bb954ebb2030b4aac29e19bb74ca0))
* resolve all clippy warnings ([5a3f218](https://github.com/stratif-io/tofa/commit/5a3f21828887f1578c70f583704530bb401c593c))
* **security:** vendor jsQR + self-host fonts; add AUDIT.md ([7a4ca32](https://github.com/stratif-io/tofa/commit/7a4ca32a4d1f606319dca441c705b51eba50f9f0))
* **tests:** add missing id field in VaultEntry initializers and fix list output ([4cd0c7e](https://github.com/stratif-io/tofa/commit/4cd0c7ee5b7b61df80884c47600a8c6d11710343))
* **theme:** rename theme::theme to theme::mode to fix module_inceptio… ([75b60f9](https://github.com/stratif-io/tofa/commit/75b60f9cfc0d078accb796f0698a11fd43fb6131))
* **theme:** rename theme::theme to theme::mode to fix module_inception clippy lint ([a15def5](https://github.com/stratif-io/tofa/commit/a15def5622fe954394653d0477faca182ab2831c))
* **tofa:** add version to tofa-core path dependency for crates.io publish ([9719644](https://github.com/stratif-io/tofa/commit/9719644d30c3aa76482fbc02b0066293e88011a8))
* **tui:** increase shortcut hint contrast in list footer ([d9f69e0](https://github.com/stratif-io/tofa/commit/d9f69e098607924051f12c4e7b4789e71c584a4a))
* **tui:** make shortcut hints visible in list footer ([1a8973b](https://github.com/stratif-io/tofa/commit/1a8973b62b164e2b7d2a0e10e84fbe419c0d2d9d))
* **tui:** use TEXT color for shortcut descriptions in footer ([9edd64d](https://github.com/stratif-io/tofa/commit/9edd64d569c275a1d007d15a715971453fb89668))
* use is_some_and instead of is_ok_and on Option in scan.rs ([d037344](https://github.com/stratif-io/tofa/commit/d0373442b822f125471b29568789a20d148ca0f0))


### Reverts

* **cli:** drop xcap, use platform subprocesses for screen capture ([b51fc8d](https://github.com/stratif-io/tofa/commit/b51fc8d6da2d3df55ed1d8f25f5dd2161fc294a7))
