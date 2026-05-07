# Changelog

## [0.9.0](https://github.com/stratif-io/tofa/compare/v0.8.0...v0.9.0) (2026-05-07)


### Miscellaneous

* version bumped to keep the linked-versions group aligned with tofa-core 0.9.0; no tofa code changes since 0.8.0.

## [0.8.0](https://github.com/stratif-io/tofa/compare/v0.7.0...v0.8.0) (2026-05-07)


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
* **tauri:** scan screen + camera QR, tray context menu ([69d336b](https://github.com/stratif-io/tofa/commit/69d336b00fe3946699589f4f5ddc0a2f44b6ca32))
* **tofa/cli:** add brand colors, voice messages, and expires column to list output ([4809628](https://github.com/stratif-io/tofa/commit/48096280accd9a4804de72b896359da1bdbacc15))
* **tofa:** migrate TUI to tofa-theme palette — replace GitHub blue with Sir Wink purple ([e33edef](https://github.com/stratif-io/tofa/commit/e33edefb2d013454c9bfd4e96e845848ea0fdc25))
* **tui,app:** accept pasted list of otpauth:// URIs as bulk import ([0eb62a8](https://github.com/stratif-io/tofa/commit/0eb62a84f696b1ddc8272211fd490ac2b32e1dd8))
* **tui,app:** list-of-otpauth multi-QR export in TUI and Tauri app ([316fdce](https://github.com/stratif-io/tofa/commit/316fdce3cce3847591907e1c3c7b54347b39bad1))
* **tui:** brand-styled hints + fix Detail modal sizing for the URI row ([09a9be3](https://github.com/stratif-io/tofa/commit/09a9be361f632cf95e4c812be0e2c85b3d62c006))
* **tui:** click outside export modal closes it ([6eb7e66](https://github.com/stratif-io/tofa/commit/6eb7e66a55be827c5e8ba5dc9fc75deaf5e3069c))
* **tui:** click outside modal closes it, click on toast dismisses it ([8295b66](https://github.com/stratif-io/tofa/commit/8295b665f693b8fa2522d121561a298b43ec44b8))
* **tui:** direct navigation between modals (fullscreen↔detail) ([b7c0ea4](https://github.com/stratif-io/tofa/commit/b7c0ea438493623cf6a701f277a5a0ac361df523))
* **tui:** first-time passphrase confirmation + vault path display with clipboard copy ([06ce7ba](https://github.com/stratif-io/tofa/commit/06ce7ba61b33ddf0fea530273abe25a7e3dcff22))
* **tui:** honour --vault flag when launching TUI ([4398766](https://github.com/stratif-io/tofa/commit/43987665250613323d63921a5fe03fc8f52849e6))
* **tui:** i toggles detail view, footer with all shortcuts ([a2b4591](https://github.com/stratif-io/tofa/commit/a2b459162a77f12f1c55697b250e7777b6373e6f))
* **tui:** lock screen with l key, zeroise passphrase cache ([7d87350](https://github.com/stratif-io/tofa/commit/7d873507f2d4d37153af2164cd9079032acedfef))
* **tui:** main list redesign — 1 line/entry, contextual header, slate+blue palette ([31a6f2f](https://github.com/stratif-io/tofa/commit/31a6f2fc84a152abab37e115f3b5063b6bbb610a))
* **tui:** multi-file selection in the picker ([33f5486](https://github.com/stratif-io/tofa/commit/33f548632b9816d9405d418fb7590670adc3fa40))
* **tui:** new slate+blue color palette ([1d322c2](https://github.com/stratif-io/tofa/commit/1d322c2a8c8c5c6dfad0d3dedfee377bc8344225))
* **tui:** OTP code near name + per-entry expiry indicator ([bbb6bc6](https://github.com/stratif-io/tofa/commit/bbb6bc67c0c065efb67070d0b8d6a0c059781a1e))
* **tui:** OTP codes aligned in column + light separator between entries ([562e9d8](https://github.com/stratif-io/tofa/commit/562e9d8f1c6865cc4492e9f6abbd425993fdb53a))
* **tui:** show algorithm, digits, period in OTP detail view ([25a7afc](https://github.com/stratif-io/tofa/commit/25a7afc858a81d38a9aeb88cf3669328c9abd69a))
* **tui:** show otpauth URI on the detail screen ([9f7368b](https://github.com/stratif-io/tofa/commit/9f7368bf8c827f68aad2fa0076598b0ed7d65aa6))
* **tui:** show seconds before each code, smoother 50ms tick ([14768b4](https://github.com/stratif-io/tofa/commit/14768b4e6dd3bf1e468e8a36be6133127bd3fa0b))
* **tui:** space to enter fullscreen ([66e894a](https://github.com/stratif-io/tofa/commit/66e894ac6c4bce86bc497f909b9f919362d5d727))
* **tui:** space toggles fullscreen (enter removed) ([c00632f](https://github.com/stratif-io/tofa/commit/c00632f08f8bc7b06b7807e77579abd848472a22))
* **tui:** u in export shows URIs; r in view shows the entry's QR ([40472e8](https://github.com/stratif-io/tofa/commit/40472e8f805583fd1472bc4c4d9ccf7d9db071e3))
* **tui:** u to copy entry URI; u in export to save URI list to .txt ([c5fe690](https://github.com/stratif-io/tofa/commit/c5fe6905fb7827637b4862c73bcdd7554e796c0a))
* **tui:** unlock screen redesign — large logo, blue separator, input box ([2daa3f0](https://github.com/stratif-io/tofa/commit/2daa3f077333221374009dde4c1e0fc30bb4ac74))


### Bug Fixes

* **ci:** resolve merge conflict — use version 0.2.1 from main ([be001d6](https://github.com/stratif-io/tofa/commit/be001d60598077c46bc232500886c73c09f7f77d))
* **cli:** check vault exists before prompting for passphrase ([bb13511](https://github.com/stratif-io/tofa/commit/bb135115dfa7dabf6d8f8a323577b191c728e290))
* **cli:** dedup imports in `add`, `cam`, and `scan` ([a39dd77](https://github.com/stratif-io/tofa/commit/a39dd771d6bede3e25fb9e255b7aafef8420cc46))
* **cli:** emit full otpauth URI from `tofa qr <name>` ([305068a](https://github.com/stratif-io/tofa/commit/305068ad5548e79f8d77a5852f1e3f675054fdbe))
* **cli:** use format_code for list --codes, fixing 8-digit split to 4+4 ([61effe0](https://github.com/stratif-io/tofa/commit/61effe06fc34288c3d03800a321baf9b358ab834))
* **cli:** use tofa_core::qr::parse_json_bytes for import command ([21413fd](https://github.com/stratif-io/tofa/commit/21413fde6bc5fb6a314a5a852315bea033b008fc))
* **core,app:** centralise dedup in Vault::add_entry_if_unique ([7b97628](https://github.com/stratif-io/tofa/commit/7b97628395be94064ffb12aafd719561b1dbd52b))
* **core:** preserve per-account algorithm and digits in migration export ([49641a8](https://github.com/stratif-io/tofa/commit/49641a8dcd0ca1885db7d9c56f471301c6b2a7ff))
* proper 8-digit code formatting (XXXX XXXX) in TUI and Tauri app ([acf2582](https://github.com/stratif-io/tofa/commit/acf2582104b15e7cacaea5ec0b508264b0c11f71))
* **release:** bump tofa-core to 0.4.0 and tofa to 0.5.0 ([ee895da](https://github.com/stratif-io/tofa/commit/ee895da58bc6d81bd52f4d670a002ea7bfc45f88))
* repair broken 0.4.0 publish, harden release pipeline, preserve OTP params in migration export ([654a6a9](https://github.com/stratif-io/tofa/commit/654a6a910f1bb954ebb2030b4aac29e19bb74ca0))
* resolve all clippy warnings ([5a3f218](https://github.com/stratif-io/tofa/commit/5a3f21828887f1578c70f583704530bb401c593c))
* **tests:** add missing id field in VaultEntry initializers and fix list output ([4cd0c7e](https://github.com/stratif-io/tofa/commit/4cd0c7ee5b7b61df80884c47600a8c6d11710343))
* **theme:** rename theme::theme to theme::mode to fix module_inceptio… ([75b60f9](https://github.com/stratif-io/tofa/commit/75b60f9cfc0d078accb796f0698a11fd43fb6131))
* **theme:** rename theme::theme to theme::mode to fix module_inception clippy lint ([a15def5](https://github.com/stratif-io/tofa/commit/a15def5622fe954394653d0477faca182ab2831c))
* **tofa:** add version to tofa-core path dependency for crates.io publish ([9719644](https://github.com/stratif-io/tofa/commit/9719644d30c3aa76482fbc02b0066293e88011a8))
* **tui:** 100ms redraw tick, 20-block bar with sub-second ms precision ([43bf70c](https://github.com/stratif-io/tofa/commit/43bf70ce820e0f46bb52f800496225c84f6ea8af))
* **tui:** align progress bars when mixing 6- and 8-digit entries ([08db8ab](https://github.com/stratif-io/tofa/commit/08db8abbdec112a9825b2890f8003cfebba1b37d))
* **tui:** copy toast visible on all screens (fullscreen, detail, list) ([3fe231c](https://github.com/stratif-io/tofa/commit/3fe231c3b7238205d697ac836a5a021369233c1e))
* **tui:** fixed gap between name and OTP, remove right alignment ([3dee0f3](https://github.com/stratif-io/tofa/commit/3dee0f303f714e56ea1ccf5d2f899ba0bbe58c5f))
* **tui:** ignore clicks on separator lines ([722460a](https://github.com/stratif-io/tofa/commit/722460af48c88fe1025abddeadc06de164dfbffb))
* **tui:** increase shortcut hint contrast in list footer ([d9f69e0](https://github.com/stratif-io/tofa/commit/d9f69e098607924051f12c4e7b4789e71c584a4a))
* **tui:** input box height 3 lines, separator proportional to name ([9f7a9b3](https://github.com/stratif-io/tofa/commit/9f7a9b369802e54fc69cab62a0d58b654625a660))
* **tui:** make shortcut hints visible in list footer ([1a8973b](https://github.com/stratif-io/tofa/commit/1a8973b62b164e2b7d2a0e10e84fbe419c0d2d9d))
* **tui:** move seconds counter to right of progress bar ([e49ec54](https://github.com/stratif-io/tofa/commit/e49ec54c13dcad6b6caed1f1067309a491434f58))
* **tui:** move vault path to bottom of unlock screen, dimmed ([f7cf5db](https://github.com/stratif-io/tofa/commit/f7cf5dba239bb9f14929eb657afdfcb1819db4bf))
* **tui:** prevent bar width jitter when partial block is zero ([2a926d1](https://github.com/stratif-io/tofa/commit/2a926d1034af62c96449f6c5e6c64d843741b40e))
* **tui:** replace dotted empty bar chars with spaces ([df113c4](https://github.com/stratif-io/tofa/commit/df113c4eec8e92fbf673e4236d7f1415465b1755))
* **tui:** replace rotp with tofa on unlock screen ([cef0418](https://github.com/stratif-io/tofa/commit/cef0418473ed5eff4d9a3ce6b5418730f3c9a937))
* **tui:** restrict click-to-copy to content width, ignore blank space ([e5f2d6f](https://github.com/stratif-io/tofa/commit/e5f2d6f63f7f096804aab4f457b6794cea89efe2))
* **tui:** restrict mouse copy to visible list rows only ([95bbff3](https://github.com/stratif-io/tofa/commit/95bbff3dda2b526810520b7a1ac74543a05f1644))
* **tui:** smooth progress bar with 1/8-block unicode precision ([4f36e57](https://github.com/stratif-io/tofa/commit/4f36e57522a503a0fc25f25f7b8c7f045be898a0))
* **tui:** unicode-width for padding, compute remaining secs once per frame ([5e243fc](https://github.com/stratif-io/tofa/commit/5e243fc82f8ab529f3be041deed53a35da1f2609))
* **tui:** use TEXT color for shortcut descriptions in footer ([9edd64d](https://github.com/stratif-io/tofa/commit/9edd64d569c275a1d007d15a715971453fb89668))
* **tui:** widen fullscreen modal for 8-digit OTP codes ([488207b](https://github.com/stratif-io/tofa/commit/488207be3f6509e83076c89ab3797a9c88810618))
* use is_some_and instead of is_ok_and on Option in scan.rs ([d037344](https://github.com/stratif-io/tofa/commit/d0373442b822f125471b29568789a20d148ca0f0))


### Reverts

* **cli:** drop xcap, use platform subprocesses for screen capture ([b51fc8d](https://github.com/stratif-io/tofa/commit/b51fc8d6da2d3df55ed1d8f25f5dd2161fc294a7))

## [0.7.0](https://github.com/stratif-io/tofa/compare/v0.6.0...v0.7.0) (2026-05-06)


### Features

* **cli:** \`tofa qr --all --multi --output-dir <dir>\` writes one otpauth:// PNG per entry ([a9ecb02](https://github.com/stratif-io/tofa/commit/a9ecb02cb772911ba74a585ca5641c7947876bf0))
* **core:** build_selection_uri dispatches by selection shape ([6695d10](https://github.com/stratif-io/tofa/commit/6695d106b119d36fe3811034ce05569bc457c6f5))
* full-fidelity QR export rules across CLI, TUI, and Tauri app ([b0cb6de](https://github.com/stratif-io/tofa/commit/b0cb6decb0820186368619ff9f09d420c94b64ba))
* **tui,app:** list-of-otpauth multi-QR export in TUI and Tauri app ([316fdce](https://github.com/stratif-io/tofa/commit/316fdce3cce3847591907e1c3c7b54347b39bad1))


### Bug Fixes

* **cli:** emit full otpauth URI from `tofa qr <name>` ([305068a](https://github.com/stratif-io/tofa/commit/305068ad5548e79f8d77a5852f1e3f675054fdbe))

## [0.6.0](https://github.com/stratif-io/tofa/compare/v0.5.0...v0.6.0) (2026-05-06)


### Features

* **app:** redesign UI — settings page, nav bar, TOFA branding, icon regen ([aa7edf9](https://github.com/stratif-io/tofa/commit/aa7edf9900bfb568c393b4f85d80499c26db4436))
* **ci:** add crates.io publish workflow and complete crate metadata ([3428908](https://github.com/stratif-io/tofa/commit/34289083e9c917eab298df202d467b1fd74afbed))
* **ci:** crates.io publish workflow and complete crate metadata ([62a26a0](https://github.com/stratif-io/tofa/commit/62a26a0c177fe812a2e702abb2caf84d0d9b9845))
* **cli:** display entry id in list output ([ac7d16d](https://github.com/stratif-io/tofa/commit/ac7d16d59b50a18d1c63f330c95abc9ef6702c50))
* multi-format import, UI redesign, Homebrew tap, TOFA branding ([30b7fd9](https://github.com/stratif-io/tofa/commit/30b7fd99276ead2450ee02e59304598b67fd1bfb))
* **tauri:** full Tauri app UI — loader, drag-drop, scan feedback, lock/unlock UX ([7d5fde0](https://github.com/stratif-io/tofa/commit/7d5fde0449aa31770a486d04083c095c7c105c40))
* **tauri:** scan screen + camera QR, tray context menu ([69d336b](https://github.com/stratif-io/tofa/commit/69d336b00fe3946699589f4f5ddc0a2f44b6ca32))
* **tofa/cli:** add brand colors, voice messages, and expires column to list output ([4809628](https://github.com/stratif-io/tofa/commit/48096280accd9a4804de72b896359da1bdbacc15))
* **tofa:** migrate TUI to tofa-theme palette — replace GitHub blue with Sir Wink purple ([e33edef](https://github.com/stratif-io/tofa/commit/e33edefb2d013454c9bfd4e96e845848ea0fdc25))
* **tui:** click outside export modal closes it ([6eb7e66](https://github.com/stratif-io/tofa/commit/6eb7e66a55be827c5e8ba5dc9fc75deaf5e3069c))
* **tui:** click outside modal closes it, click on toast dismisses it ([8295b66](https://github.com/stratif-io/tofa/commit/8295b665f693b8fa2522d121561a298b43ec44b8))
* **tui:** direct navigation between modals (fullscreen↔detail) ([b7c0ea4](https://github.com/stratif-io/tofa/commit/b7c0ea438493623cf6a701f277a5a0ac361df523))
* **tui:** first-time passphrase confirmation + vault path display with clipboard copy ([06ce7ba](https://github.com/stratif-io/tofa/commit/06ce7ba61b33ddf0fea530273abe25a7e3dcff22))
* **tui:** honour --vault flag when launching TUI ([4398766](https://github.com/stratif-io/tofa/commit/43987665250613323d63921a5fe03fc8f52849e6))
* **tui:** i toggles detail view, footer with all shortcuts ([a2b4591](https://github.com/stratif-io/tofa/commit/a2b459162a77f12f1c55697b250e7777b6373e6f))
* **tui:** lock screen with l key, zeroise passphrase cache ([7d87350](https://github.com/stratif-io/tofa/commit/7d873507f2d4d37153af2164cd9079032acedfef))
* **tui:** main list redesign — 1 line/entry, contextual header, slate+blue palette ([31a6f2f](https://github.com/stratif-io/tofa/commit/31a6f2fc84a152abab37e115f3b5063b6bbb610a))
* **tui:** new slate+blue color palette ([1d322c2](https://github.com/stratif-io/tofa/commit/1d322c2a8c8c5c6dfad0d3dedfee377bc8344225))
* **tui:** OTP code near name + per-entry expiry indicator ([bbb6bc6](https://github.com/stratif-io/tofa/commit/bbb6bc67c0c065efb67070d0b8d6a0c059781a1e))
* **tui:** OTP codes aligned in column + light separator between entries ([562e9d8](https://github.com/stratif-io/tofa/commit/562e9d8f1c6865cc4492e9f6abbd425993fdb53a))
* **tui:** show algorithm, digits, period in OTP detail view ([25a7afc](https://github.com/stratif-io/tofa/commit/25a7afc858a81d38a9aeb88cf3669328c9abd69a))
* **tui:** show seconds before each code, smoother 50ms tick ([14768b4](https://github.com/stratif-io/tofa/commit/14768b4e6dd3bf1e468e8a36be6133127bd3fa0b))
* **tui:** space to enter fullscreen ([66e894a](https://github.com/stratif-io/tofa/commit/66e894ac6c4bce86bc497f909b9f919362d5d727))
* **tui:** space toggles fullscreen (enter removed) ([c00632f](https://github.com/stratif-io/tofa/commit/c00632f08f8bc7b06b7807e77579abd848472a22))
* **tui:** unlock screen redesign — large logo, blue separator, input box ([2daa3f0](https://github.com/stratif-io/tofa/commit/2daa3f077333221374009dde4c1e0fc30bb4ac74))


### Bug Fixes

* **ci:** resolve merge conflict — use version 0.2.1 from main ([be001d6](https://github.com/stratif-io/tofa/commit/be001d60598077c46bc232500886c73c09f7f77d))
* **cli:** check vault exists before prompting for passphrase ([bb13511](https://github.com/stratif-io/tofa/commit/bb135115dfa7dabf6d8f8a323577b191c728e290))
* **cli:** use format_code for list --codes, fixing 8-digit split to 4+4 ([61effe0](https://github.com/stratif-io/tofa/commit/61effe06fc34288c3d03800a321baf9b358ab834))
* **cli:** use tofa_core::qr::parse_json_bytes for import command ([21413fd](https://github.com/stratif-io/tofa/commit/21413fde6bc5fb6a314a5a852315bea033b008fc))
* **core:** preserve per-account algorithm and digits in migration export ([49641a8](https://github.com/stratif-io/tofa/commit/49641a8dcd0ca1885db7d9c56f471301c6b2a7ff))
* proper 8-digit code formatting (XXXX XXXX) in TUI and Tauri app ([acf2582](https://github.com/stratif-io/tofa/commit/acf2582104b15e7cacaea5ec0b508264b0c11f71))
* **release:** bump tofa-core to 0.4.0 and tofa to 0.5.0 ([ee895da](https://github.com/stratif-io/tofa/commit/ee895da58bc6d81bd52f4d670a002ea7bfc45f88))
* repair broken 0.4.0 publish, harden release pipeline, preserve OTP params in migration export ([654a6a9](https://github.com/stratif-io/tofa/commit/654a6a910f1bb954ebb2030b4aac29e19bb74ca0))
* resolve all clippy warnings ([5a3f218](https://github.com/stratif-io/tofa/commit/5a3f21828887f1578c70f583704530bb401c593c))
* **tests:** add missing id field in VaultEntry initializers and fix list output ([4cd0c7e](https://github.com/stratif-io/tofa/commit/4cd0c7ee5b7b61df80884c47600a8c6d11710343))
* **theme:** rename theme::theme to theme::mode to fix module_inceptio… ([75b60f9](https://github.com/stratif-io/tofa/commit/75b60f9cfc0d078accb796f0698a11fd43fb6131))
* **theme:** rename theme::theme to theme::mode to fix module_inception clippy lint ([a15def5](https://github.com/stratif-io/tofa/commit/a15def5622fe954394653d0477faca182ab2831c))
* **tofa:** add version to tofa-core path dependency for crates.io publish ([9719644](https://github.com/stratif-io/tofa/commit/9719644d30c3aa76482fbc02b0066293e88011a8))
* **tui:** 100ms redraw tick, 20-block bar with sub-second ms precision ([43bf70c](https://github.com/stratif-io/tofa/commit/43bf70ce820e0f46bb52f800496225c84f6ea8af))
* **tui:** align progress bars when mixing 6- and 8-digit entries ([08db8ab](https://github.com/stratif-io/tofa/commit/08db8abbdec112a9825b2890f8003cfebba1b37d))
* **tui:** copy toast visible on all screens (fullscreen, detail, list) ([3fe231c](https://github.com/stratif-io/tofa/commit/3fe231c3b7238205d697ac836a5a021369233c1e))
* **tui:** fixed gap between name and OTP, remove right alignment ([3dee0f3](https://github.com/stratif-io/tofa/commit/3dee0f303f714e56ea1ccf5d2f899ba0bbe58c5f))
* **tui:** ignore clicks on separator lines ([722460a](https://github.com/stratif-io/tofa/commit/722460af48c88fe1025abddeadc06de164dfbffb))
* **tui:** increase shortcut hint contrast in list footer ([d9f69e0](https://github.com/stratif-io/tofa/commit/d9f69e098607924051f12c4e7b4789e71c584a4a))
* **tui:** input box height 3 lines, separator proportional to name ([9f7a9b3](https://github.com/stratif-io/tofa/commit/9f7a9b369802e54fc69cab62a0d58b654625a660))
* **tui:** make shortcut hints visible in list footer ([1a8973b](https://github.com/stratif-io/tofa/commit/1a8973b62b164e2b7d2a0e10e84fbe419c0d2d9d))
* **tui:** move seconds counter to right of progress bar ([e49ec54](https://github.com/stratif-io/tofa/commit/e49ec54c13dcad6b6caed1f1067309a491434f58))
* **tui:** move vault path to bottom of unlock screen, dimmed ([f7cf5db](https://github.com/stratif-io/tofa/commit/f7cf5dba239bb9f14929eb657afdfcb1819db4bf))
* **tui:** prevent bar width jitter when partial block is zero ([2a926d1](https://github.com/stratif-io/tofa/commit/2a926d1034af62c96449f6c5e6c64d843741b40e))
* **tui:** replace dotted empty bar chars with spaces ([df113c4](https://github.com/stratif-io/tofa/commit/df113c4eec8e92fbf673e4236d7f1415465b1755))
* **tui:** replace rotp with tofa on unlock screen ([cef0418](https://github.com/stratif-io/tofa/commit/cef0418473ed5eff4d9a3ce6b5418730f3c9a937))
* **tui:** restrict click-to-copy to content width, ignore blank space ([e5f2d6f](https://github.com/stratif-io/tofa/commit/e5f2d6f63f7f096804aab4f457b6794cea89efe2))
* **tui:** restrict mouse copy to visible list rows only ([95bbff3](https://github.com/stratif-io/tofa/commit/95bbff3dda2b526810520b7a1ac74543a05f1644))
* **tui:** smooth progress bar with 1/8-block unicode precision ([4f36e57](https://github.com/stratif-io/tofa/commit/4f36e57522a503a0fc25f25f7b8c7f045be898a0))
* **tui:** unicode-width for padding, compute remaining secs once per frame ([5e243fc](https://github.com/stratif-io/tofa/commit/5e243fc82f8ab529f3be041deed53a35da1f2609))
* **tui:** use TEXT color for shortcut descriptions in footer ([9edd64d](https://github.com/stratif-io/tofa/commit/9edd64d569c275a1d007d15a715971453fb89668))
* **tui:** widen fullscreen modal for 8-digit OTP codes ([488207b](https://github.com/stratif-io/tofa/commit/488207be3f6509e83076c89ab3797a9c88810618))
* use is_some_and instead of is_ok_and on Option in scan.rs ([d037344](https://github.com/stratif-io/tofa/commit/d0373442b822f125471b29568789a20d148ca0f0))

## [0.4.0](https://github.com/stratif-io/tofa/compare/v0.3.0...v0.4.0) (2026-05-05)


### Features

* **app:** redesign UI — settings page, nav bar, TOFA branding, icon regen ([aa7edf9](https://github.com/stratif-io/tofa/commit/aa7edf9900bfb568c393b4f85d80499c26db4436))
* **cli:** display entry id in list output ([ac7d16d](https://github.com/stratif-io/tofa/commit/ac7d16d59b50a18d1c63f330c95abc9ef6702c50))
* multi-format import, UI redesign, Homebrew tap, TOFA branding ([30b7fd9](https://github.com/stratif-io/tofa/commit/30b7fd99276ead2450ee02e59304598b67fd1bfb))


### Bug Fixes

* **cli:** check vault exists before prompting for passphrase ([bb13511](https://github.com/stratif-io/tofa/commit/bb135115dfa7dabf6d8f8a323577b191c728e290))
* **cli:** use format_code for list --codes, fixing 8-digit split to 4+4 ([61effe0](https://github.com/stratif-io/tofa/commit/61effe06fc34288c3d03800a321baf9b358ab834))
* **cli:** use tofa_core::qr::parse_json_bytes for import command ([21413fd](https://github.com/stratif-io/tofa/commit/21413fde6bc5fb6a314a5a852315bea033b008fc))
* **tui:** increase shortcut hint contrast in list footer ([d9f69e0](https://github.com/stratif-io/tofa/commit/d9f69e098607924051f12c4e7b4789e71c584a4a))
* **tui:** make shortcut hints visible in list footer ([1a8973b](https://github.com/stratif-io/tofa/commit/1a8973b62b164e2b7d2a0e10e84fbe419c0d2d9d))
* **tui:** use TEXT color for shortcut descriptions in footer ([9edd64d](https://github.com/stratif-io/tofa/commit/9edd64d569c275a1d007d15a715971453fb89668))

## [0.3.0](https://github.com/stratif-io/tofa/compare/v0.2.1...v0.3.0) (2026-05-04)


### Features

* **ci:** add crates.io publish workflow and complete crate metadata ([3428908](https://github.com/stratif-io/tofa/commit/34289083e9c917eab298df202d467b1fd74afbed))
* **ci:** crates.io publish workflow and complete crate metadata ([62a26a0](https://github.com/stratif-io/tofa/commit/62a26a0c177fe812a2e702abb2caf84d0d9b9845))
* **tauri:** full Tauri app UI — loader, drag-drop, scan feedback, lock/unlock UX ([7d5fde0](https://github.com/stratif-io/tofa/commit/7d5fde0449aa31770a486d04083c095c7c105c40))
* **tauri:** scan screen + camera QR, tray context menu ([69d336b](https://github.com/stratif-io/tofa/commit/69d336b00fe3946699589f4f5ddc0a2f44b6ca32))
* **tofa/cli:** add brand colors, voice messages, and expires column to list output ([4809628](https://github.com/stratif-io/tofa/commit/48096280accd9a4804de72b896359da1bdbacc15))
* **tofa:** migrate TUI to tofa-theme palette — replace GitHub blue with Sir Wink purple ([e33edef](https://github.com/stratif-io/tofa/commit/e33edefb2d013454c9bfd4e96e845848ea0fdc25))
* **tui:** click outside export modal closes it ([6eb7e66](https://github.com/stratif-io/tofa/commit/6eb7e66a55be827c5e8ba5dc9fc75deaf5e3069c))
* **tui:** click outside modal closes it, click on toast dismisses it ([8295b66](https://github.com/stratif-io/tofa/commit/8295b665f693b8fa2522d121561a298b43ec44b8))
* **tui:** direct navigation between modals (fullscreen↔detail) ([b7c0ea4](https://github.com/stratif-io/tofa/commit/b7c0ea438493623cf6a701f277a5a0ac361df523))
* **tui:** first-time passphrase confirmation + vault path display with clipboard copy ([06ce7ba](https://github.com/stratif-io/tofa/commit/06ce7ba61b33ddf0fea530273abe25a7e3dcff22))
* **tui:** honour --vault flag when launching TUI ([4398766](https://github.com/stratif-io/tofa/commit/43987665250613323d63921a5fe03fc8f52849e6))
* **tui:** i toggles detail view, footer with all shortcuts ([a2b4591](https://github.com/stratif-io/tofa/commit/a2b459162a77f12f1c55697b250e7777b6373e6f))
* **tui:** lock screen with l key, zeroise passphrase cache ([7d87350](https://github.com/stratif-io/tofa/commit/7d873507f2d4d37153af2164cd9079032acedfef))
* **tui:** main list redesign — 1 line/entry, contextual header, slate+blue palette ([31a6f2f](https://github.com/stratif-io/tofa/commit/31a6f2fc84a152abab37e115f3b5063b6bbb610a))
* **tui:** new slate+blue color palette ([1d322c2](https://github.com/stratif-io/tofa/commit/1d322c2a8c8c5c6dfad0d3dedfee377bc8344225))
* **tui:** OTP code near name + per-entry expiry indicator ([bbb6bc6](https://github.com/stratif-io/tofa/commit/bbb6bc67c0c065efb67070d0b8d6a0c059781a1e))
* **tui:** OTP codes aligned in column + light separator between entries ([562e9d8](https://github.com/stratif-io/tofa/commit/562e9d8f1c6865cc4492e9f6abbd425993fdb53a))
* **tui:** show algorithm, digits, period in OTP detail view ([25a7afc](https://github.com/stratif-io/tofa/commit/25a7afc858a81d38a9aeb88cf3669328c9abd69a))
* **tui:** show seconds before each code, smoother 50ms tick ([14768b4](https://github.com/stratif-io/tofa/commit/14768b4e6dd3bf1e468e8a36be6133127bd3fa0b))
* **tui:** space to enter fullscreen ([66e894a](https://github.com/stratif-io/tofa/commit/66e894ac6c4bce86bc497f909b9f919362d5d727))
* **tui:** space toggles fullscreen (enter removed) ([c00632f](https://github.com/stratif-io/tofa/commit/c00632f08f8bc7b06b7807e77579abd848472a22))
* **tui:** unlock screen redesign — large logo, blue separator, input box ([2daa3f0](https://github.com/stratif-io/tofa/commit/2daa3f077333221374009dde4c1e0fc30bb4ac74))


### Bug Fixes

* **ci:** resolve merge conflict — use version 0.2.1 from main ([be001d6](https://github.com/stratif-io/tofa/commit/be001d60598077c46bc232500886c73c09f7f77d))
* proper 8-digit code formatting (XXXX XXXX) in TUI and Tauri app ([acf2582](https://github.com/stratif-io/tofa/commit/acf2582104b15e7cacaea5ec0b508264b0c11f71))
* resolve all clippy warnings ([5a3f218](https://github.com/stratif-io/tofa/commit/5a3f21828887f1578c70f583704530bb401c593c))
* **tests:** add missing id field in VaultEntry initializers and fix list output ([4cd0c7e](https://github.com/stratif-io/tofa/commit/4cd0c7ee5b7b61df80884c47600a8c6d11710343))
* **theme:** rename theme::theme to theme::mode to fix module_inceptio… ([75b60f9](https://github.com/stratif-io/tofa/commit/75b60f9cfc0d078accb796f0698a11fd43fb6131))
* **theme:** rename theme::theme to theme::mode to fix module_inception clippy lint ([a15def5](https://github.com/stratif-io/tofa/commit/a15def5622fe954394653d0477faca182ab2831c))
* **tofa:** add version to tofa-core path dependency for crates.io publish ([9719644](https://github.com/stratif-io/tofa/commit/9719644d30c3aa76482fbc02b0066293e88011a8))
* **tui:** 100ms redraw tick, 20-block bar with sub-second ms precision ([43bf70c](https://github.com/stratif-io/tofa/commit/43bf70ce820e0f46bb52f800496225c84f6ea8af))
* **tui:** align progress bars when mixing 6- and 8-digit entries ([08db8ab](https://github.com/stratif-io/tofa/commit/08db8abbdec112a9825b2890f8003cfebba1b37d))
* **tui:** copy toast visible on all screens (fullscreen, detail, list) ([3fe231c](https://github.com/stratif-io/tofa/commit/3fe231c3b7238205d697ac836a5a021369233c1e))
* **tui:** fixed gap between name and OTP, remove right alignment ([3dee0f3](https://github.com/stratif-io/tofa/commit/3dee0f303f714e56ea1ccf5d2f899ba0bbe58c5f))
* **tui:** ignore clicks on separator lines ([722460a](https://github.com/stratif-io/tofa/commit/722460af48c88fe1025abddeadc06de164dfbffb))
* **tui:** input box height 3 lines, separator proportional to name ([9f7a9b3](https://github.com/stratif-io/tofa/commit/9f7a9b369802e54fc69cab62a0d58b654625a660))
* **tui:** move seconds counter to right of progress bar ([e49ec54](https://github.com/stratif-io/tofa/commit/e49ec54c13dcad6b6caed1f1067309a491434f58))
* **tui:** move vault path to bottom of unlock screen, dimmed ([f7cf5db](https://github.com/stratif-io/tofa/commit/f7cf5dba239bb9f14929eb657afdfcb1819db4bf))
* **tui:** prevent bar width jitter when partial block is zero ([2a926d1](https://github.com/stratif-io/tofa/commit/2a926d1034af62c96449f6c5e6c64d843741b40e))
* **tui:** replace dotted empty bar chars with spaces ([df113c4](https://github.com/stratif-io/tofa/commit/df113c4eec8e92fbf673e4236d7f1415465b1755))
* **tui:** replace rotp with tofa on unlock screen ([cef0418](https://github.com/stratif-io/tofa/commit/cef0418473ed5eff4d9a3ce6b5418730f3c9a937))
* **tui:** restrict click-to-copy to content width, ignore blank space ([e5f2d6f](https://github.com/stratif-io/tofa/commit/e5f2d6f63f7f096804aab4f457b6794cea89efe2))
* **tui:** restrict mouse copy to visible list rows only ([95bbff3](https://github.com/stratif-io/tofa/commit/95bbff3dda2b526810520b7a1ac74543a05f1644))
* **tui:** smooth progress bar with 1/8-block unicode precision ([4f36e57](https://github.com/stratif-io/tofa/commit/4f36e57522a503a0fc25f25f7b8c7f045be898a0))
* **tui:** unicode-width for padding, compute remaining secs once per frame ([5e243fc](https://github.com/stratif-io/tofa/commit/5e243fc82f8ab529f3be041deed53a35da1f2609))
* **tui:** widen fullscreen modal for 8-digit OTP codes ([488207b](https://github.com/stratif-io/tofa/commit/488207be3f6509e83076c89ab3797a9c88810618))
* use is_some_and instead of is_ok_and on Option in scan.rs ([d037344](https://github.com/stratif-io/tofa/commit/d0373442b822f125471b29568789a20d148ca0f0))

## [0.2.1](https://github.com/cabichahine/tofa/compare/v0.2.0...v0.2.1) (2026-05-04)


### Bug Fixes

* **ci:** resolve merge conflict — use version 0.2.1 from main ([be001d6](https://github.com/cabichahine/tofa/commit/be001d60598077c46bc232500886c73c09f7f77d))

## [0.2.0](https://github.com/cabichahine/tofa/compare/v0.1.0...v0.2.0) (2026-05-04)


### Features

* **tauri:** full Tauri app UI — loader, drag-drop, scan feedback, lock/unlock UX ([7d5fde0](https://github.com/cabichahine/tofa/commit/7d5fde0449aa31770a486d04083c095c7c105c40))
* **tauri:** scan screen + camera QR, tray context menu ([69d336b](https://github.com/cabichahine/tofa/commit/69d336b00fe3946699589f4f5ddc0a2f44b6ca32))
* **tofa/cli:** add brand colors, voice messages, and expires column to list output ([4809628](https://github.com/cabichahine/tofa/commit/48096280accd9a4804de72b896359da1bdbacc15))
* **tofa:** migrate TUI to tofa-theme palette — replace GitHub blue with Sir Wink purple ([e33edef](https://github.com/cabichahine/tofa/commit/e33edefb2d013454c9bfd4e96e845848ea0fdc25))
* **tui:** click outside export modal closes it ([6eb7e66](https://github.com/cabichahine/tofa/commit/6eb7e66a55be827c5e8ba5dc9fc75deaf5e3069c))
* **tui:** click outside modal closes it, click on toast dismisses it ([8295b66](https://github.com/cabichahine/tofa/commit/8295b665f693b8fa2522d121561a298b43ec44b8))
* **tui:** direct navigation between modals (fullscreen↔detail) ([b7c0ea4](https://github.com/cabichahine/tofa/commit/b7c0ea438493623cf6a701f277a5a0ac361df523))
* **tui:** first-time passphrase confirmation + vault path display with clipboard copy ([06ce7ba](https://github.com/cabichahine/tofa/commit/06ce7ba61b33ddf0fea530273abe25a7e3dcff22))
* **tui:** honour --vault flag when launching TUI ([4398766](https://github.com/cabichahine/tofa/commit/43987665250613323d63921a5fe03fc8f52849e6))
* **tui:** i toggles detail view, footer with all shortcuts ([a2b4591](https://github.com/cabichahine/tofa/commit/a2b459162a77f12f1c55697b250e7777b6373e6f))
* **tui:** lock screen with l key, zeroise passphrase cache ([7d87350](https://github.com/cabichahine/tofa/commit/7d873507f2d4d37153af2164cd9079032acedfef))
* **tui:** main list redesign — 1 line/entry, contextual header, slate+blue palette ([31a6f2f](https://github.com/cabichahine/tofa/commit/31a6f2fc84a152abab37e115f3b5063b6bbb610a))
* **tui:** new slate+blue color palette ([1d322c2](https://github.com/cabichahine/tofa/commit/1d322c2a8c8c5c6dfad0d3dedfee377bc8344225))
* **tui:** OTP code near name + per-entry expiry indicator ([bbb6bc6](https://github.com/cabichahine/tofa/commit/bbb6bc67c0c065efb67070d0b8d6a0c059781a1e))
* **tui:** OTP codes aligned in column + light separator between entries ([562e9d8](https://github.com/cabichahine/tofa/commit/562e9d8f1c6865cc4492e9f6abbd425993fdb53a))
* **tui:** show algorithm, digits, period in OTP detail view ([25a7afc](https://github.com/cabichahine/tofa/commit/25a7afc858a81d38a9aeb88cf3669328c9abd69a))
* **tui:** show seconds before each code, smoother 50ms tick ([14768b4](https://github.com/cabichahine/tofa/commit/14768b4e6dd3bf1e468e8a36be6133127bd3fa0b))
* **tui:** space to enter fullscreen ([66e894a](https://github.com/cabichahine/tofa/commit/66e894ac6c4bce86bc497f909b9f919362d5d727))
* **tui:** space toggles fullscreen (enter removed) ([c00632f](https://github.com/cabichahine/tofa/commit/c00632f08f8bc7b06b7807e77579abd848472a22))
* **tui:** unlock screen redesign — large logo, blue separator, input box ([2daa3f0](https://github.com/cabichahine/tofa/commit/2daa3f077333221374009dde4c1e0fc30bb4ac74))


### Bug Fixes

* proper 8-digit code formatting (XXXX XXXX) in TUI and Tauri app ([acf2582](https://github.com/cabichahine/tofa/commit/acf2582104b15e7cacaea5ec0b508264b0c11f71))
* resolve all clippy warnings ([5a3f218](https://github.com/cabichahine/tofa/commit/5a3f21828887f1578c70f583704530bb401c593c))
* **tests:** add missing id field in VaultEntry initializers and fix list output ([4cd0c7e](https://github.com/cabichahine/tofa/commit/4cd0c7ee5b7b61df80884c47600a8c6d11710343))
* **tui:** 100ms redraw tick, 20-block bar with sub-second ms precision ([43bf70c](https://github.com/cabichahine/tofa/commit/43bf70ce820e0f46bb52f800496225c84f6ea8af))
* **tui:** align progress bars when mixing 6- and 8-digit entries ([08db8ab](https://github.com/cabichahine/tofa/commit/08db8abbdec112a9825b2890f8003cfebba1b37d))
* **tui:** copy toast visible on all screens (fullscreen, detail, list) ([3fe231c](https://github.com/cabichahine/tofa/commit/3fe231c3b7238205d697ac836a5a021369233c1e))
* **tui:** fixed gap between name and OTP, remove right alignment ([3dee0f3](https://github.com/cabichahine/tofa/commit/3dee0f303f714e56ea1ccf5d2f899ba0bbe58c5f))
* **tui:** ignore clicks on separator lines ([722460a](https://github.com/cabichahine/tofa/commit/722460af48c88fe1025abddeadc06de164dfbffb))
* **tui:** input box height 3 lines, separator proportional to name ([9f7a9b3](https://github.com/cabichahine/tofa/commit/9f7a9b369802e54fc69cab62a0d58b654625a660))
* **tui:** move seconds counter to right of progress bar ([e49ec54](https://github.com/cabichahine/tofa/commit/e49ec54c13dcad6b6caed1f1067309a491434f58))
* **tui:** move vault path to bottom of unlock screen, dimmed ([f7cf5db](https://github.com/cabichahine/tofa/commit/f7cf5dba239bb9f14929eb657afdfcb1819db4bf))
* **tui:** prevent bar width jitter when partial block is zero ([2a926d1](https://github.com/cabichahine/tofa/commit/2a926d1034af62c96449f6c5e6c64d843741b40e))
* **tui:** replace dotted empty bar chars with spaces ([df113c4](https://github.com/cabichahine/tofa/commit/df113c4eec8e92fbf673e4236d7f1415465b1755))
* **tui:** replace rotp with tofa on unlock screen ([cef0418](https://github.com/cabichahine/tofa/commit/cef0418473ed5eff4d9a3ce6b5418730f3c9a937))
* **tui:** restrict click-to-copy to content width, ignore blank space ([e5f2d6f](https://github.com/cabichahine/tofa/commit/e5f2d6f63f7f096804aab4f457b6794cea89efe2))
* **tui:** restrict mouse copy to visible list rows only ([95bbff3](https://github.com/cabichahine/tofa/commit/95bbff3dda2b526810520b7a1ac74543a05f1644))
* **tui:** smooth progress bar with 1/8-block unicode precision ([4f36e57](https://github.com/cabichahine/tofa/commit/4f36e57522a503a0fc25f25f7b8c7f045be898a0))
* **tui:** unicode-width for padding, compute remaining secs once per frame ([5e243fc](https://github.com/cabichahine/tofa/commit/5e243fc82f8ab529f3be041deed53a35da1f2609))
* **tui:** widen fullscreen modal for 8-digit OTP codes ([488207b](https://github.com/cabichahine/tofa/commit/488207be3f6509e83076c89ab3797a9c88810618))
* use is_some_and instead of is_ok_and on Option in scan.rs ([d037344](https://github.com/cabichahine/tofa/commit/d0373442b822f125471b29568789a20d148ca0f0))
