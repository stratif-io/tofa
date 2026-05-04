# Changelog

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
