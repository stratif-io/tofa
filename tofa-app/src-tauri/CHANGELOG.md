# Changelog

## [0.11.1](https://github.com/stratif-io/tofa/compare/tofa-macos-v0.11.0...tofa-macos-v0.11.1) (2026-05-11)


### Bug Fixes

* **app:** disable Scan Screen in tray + custom DMG file icon ([89375f7](https://github.com/stratif-io/tofa/commit/89375f77a3ac7c1118c841a09a534c569cb73d68))
* **app:** permanently disable Scan Screen in tray menu + custom DMG icon ([63eb700](https://github.com/stratif-io/tofa/commit/63eb7007dac8230f17228ec52e3aece9f98e6716))

## [0.11.0](https://github.com/stratif-io/tofa/compare/tofa-macos-v0.10.0...tofa-macos-v0.11.0) (2026-05-11)


### Features

* **app:** About window + silent auto-update via tauri-plugin-updater ([fad93aa](https://github.com/stratif-io/tofa/commit/fad93aa0be8c5140990a490bf3e544de125f4e4d))
* **app:** accept all import formats via Open File button ([f30efd0](https://github.com/stratif-io/tofa/commit/f30efd08f519203d529f98e0b8d0cd5d0c47cbb8))
* **app:** add About Tofa tray menu item ([bd6b888](https://github.com/stratif-io/tofa/commit/bd6b8881c6f2edb03ef6ca604e9078557cebdeab))
* **app:** add About window UI (versions + manual check) ([c020aa9](https://github.com/stratif-io/tofa/commit/c020aa98f98e167a831fadd1ba8dd13294fded6d))
* **app:** add about_window module (singleton WebviewWindow) ([ae38067](https://github.com/stratif-io/tofa/commit/ae38067f963b6e35d8055f0a106570acc5ec4a98))
* **app:** add check_for_updates and cached status commands ([83a687a](https://github.com/stratif-io/tofa/commit/83a687a27615d5cbff1dc8c2d4cb05c2c5d839d3))
* **app:** add fetch_and_check + reqwest client builder ([c344b25](https://github.com/stratif-io/tofa/commit/c344b25a5c927ebf6cbc9de69f47e0a55d36a0b1))
* **app:** add get_versions command exposing app and core versions ([8681c50](https://github.com/stratif-io/tofa/commit/8681c502fb13da17214051606ba87ec8abb396c8))
* **app:** add open_release_url command (github-only) ([fcf6399](https://github.com/stratif-io/tofa/commit/fcf63995bd092ac51b266086be9851b68679c34d))
* **app:** add plugin-backed check + download_and_install commands ([10fb8f5](https://github.com/stratif-io/tofa/commit/10fb8f5e4e39228331255c90f5d41034d5f30b1f))
* **app:** black icon background, custom DMG with arrow and drag-to-install ([ac3a8fa](https://github.com/stratif-io/tofa/commit/ac3a8fa65cf13440374676c463a68b82288b83ec))
* **app:** black icon background, custom DMG with drag-to-install ([2ee3549](https://github.com/stratif-io/tofa/commit/2ee35490788ef413fb90945405413857f5dd8d56))
* **app:** configure updater plugin (placeholder pubkey) ([ea8c3d5](https://github.com/stratif-io/tofa/commit/ea8c3d50d02e9000c1f4e29075597695d9bd5094))
* **app:** copy_uri and save_uri_list with matching UI buttons ([fe4da0f](https://github.com/stratif-io/tofa/commit/fe4da0fd19ea9d22a3a8c39a0537c94e99d8a79c))
* **app:** dark/light mode, multi-format import, settings improvements ([494913b](https://github.com/stratif-io/tofa/commit/494913bed231e46fdff192078455e46033181370))
* **app:** disable scan screen (coming soon), add QR export for single and multiple accounts ([ea17dba](https://github.com/stratif-io/tofa/commit/ea17dbad0b54645678f1a71cd375f94b5a7b0aff))
* **app:** highlight non-30s entries + bundle Save All into a single zip ([675fd3e](https://github.com/stratif-io/tofa/commit/675fd3e78688a4430a5beb98c02765e94ca96807))
* **app:** highlight non-30s entries + bundle Save All into a single zip ([8fe922c](https://github.com/stratif-io/tofa/commit/8fe922cd351d21cc06a9d881bc49e974c105f554))
* **app:** implement pick_latest for update version selection ([1a0d1b6](https://github.com/stratif-io/tofa/commit/1a0d1b63a75665aa8a0066232e388423804bf9f3))
* **app:** manage UpdaterState on the AppHandle ([996415e](https://github.com/stratif-io/tofa/commit/996415e811d01cf333da8f99d10eec9e77435efe))
* **app:** pick_and_import_file accepts a multi-file selection ([f489502](https://github.com/stratif-io/tofa/commit/f489502ce76545fcb65d480132f224886f8e4c0a))
* **app:** QR export for single/multiple accounts + disable scan screen ([7ae2229](https://github.com/stratif-io/tofa/commit/7ae2229e830e170318f694e50d3752ae305262d6))
* **app:** redesign UI — settings page, nav bar, TOFA branding, icon regen ([aa7edf9](https://github.com/stratif-io/tofa/commit/aa7edf9900bfb568c393b4f85d80499c26db4436))
* **app:** register tauri-plugin-updater ([3567cf6](https://github.com/stratif-io/tofa/commit/3567cf617bd53add5998364d9fe67905badddcd8))
* **app:** run background update check on launch and every 24h ([7db8f1f](https://github.com/stratif-io/tofa/commit/7db8f1f358c6640329cdd00305b037201aef6ca3))
* **app:** save QR PNG via native save dialog instead of browser download ([5e6a28c](https://github.com/stratif-io/tofa/commit/5e6a28c4361452ac677e26973c3769221d96d035))
* **app:** scaffold updater module with types and error enum ([6431e20](https://github.com/stratif-io/tofa/commit/6431e20c3ead94b23c542437309c4284891cbdd1))
* **app:** show otpauth URI on the entry detail view ([3ab079f](https://github.com/stratif-io/tofa/commit/3ab079f1c2f797fc7594c5733b812b3804d14f03))
* **app:** show Update available item in tray when newer release is published ([b3a47d6](https://github.com/stratif-io/tofa/commit/b3a47d61237e8de780997f8f718ca942943c6b4a))
* **app:** switch About window JS to plugin-backed commands ([cc7888e](https://github.com/stratif-io/tofa/commit/cc7888eb55496c76e14ad1d5a8771926211bdf13))
* **cli,tui,app:** route every import surface through the unified dispatcher ([5591dcb](https://github.com/stratif-io/tofa/commit/5591dcbfeb7e80d0b425ef8c045c0b9afa480e1c))
* **core:** build_selection_uri dispatches by selection shape ([6695d10](https://github.com/stratif-io/tofa/commit/6695d106b119d36fe3811034ce05569bc457c6f5))
* full-fidelity QR export rules across CLI, TUI, and Tauri app ([b0cb6de](https://github.com/stratif-io/tofa/commit/b0cb6decb0820186368619ff9f09d420c94b64ba))
* import dispatcher, URI export, dedup, and architectural audit pass ([9d8f2d4](https://github.com/stratif-io/tofa/commit/9d8f2d4432b59c2b43016e67792013a9c1be3cfa))
* multi-format import, UI redesign, Homebrew tap, TOFA branding ([30b7fd9](https://github.com/stratif-io/tofa/commit/30b7fd99276ead2450ee02e59304598b67fd1bfb))
* smooth rAF progress bar, 10s API refresh, multi-QR screen scan, scan feedback ([f07fb1e](https://github.com/stratif-io/tofa/commit/f07fb1e64be9f638d82228309c9f6cc246661a6b))
* **tauri:** card click opens detail modal, clipboard icon per entry, no auto-hide ([557af15](https://github.com/stratif-io/tofa/commit/557af150c1bf3a30302237fa691342540e3ccbcc))
* **tauri:** delete button on hover ([df3aaf9](https://github.com/stratif-io/tofa/commit/df3aaf9d9076d4ce225ef7b7049b14f5ea62109e))
* **tauri:** display issuer and account separately in entry list ([0d664c7](https://github.com/stratif-io/tofa/commit/0d664c7f04d16c8a3bbf9325b714b3d4ebcd26d1))
* **tauri:** full Tauri app UI — loader, drag-drop, scan feedback, lock/unlock UX ([7d5fde0](https://github.com/stratif-io/tofa/commit/7d5fde0449aa31770a486d04083c095c7c105c40))
* **tauri:** hide Dock icon — menu bar only (activationPolicy accessory) ([a4e33a4](https://github.com/stratif-io/tofa/commit/a4e33a4c1da2e36737156709bc34bd13461c16d3))
* **tauri:** scan screen + camera QR, tray context menu ([69d336b](https://github.com/stratif-io/tofa/commit/69d336b00fe3946699589f4f5ddc0a2f44b6ca32))
* **tauri:** show OTP metadata tooltip on hover ([46878ad](https://github.com/stratif-io/tofa/commit/46878ad819efadc66be1fca762af708d5300d174))
* **tofa-app:** implement unlock, get_entries, copy_code, settings commands ([525ea07](https://github.com/stratif-io/tofa/commit/525ea070dc8d6a46bbf141cdc2ea106200cfead8))
* **tofa-app:** PassphraseCache with 10 min TTL, AppState, command stubs ([bab112d](https://github.com/stratif-io/tofa/commit/bab112dfc152b51e0722db62a24b27b2f81ad73c))
* **tofa-app:** regenerate all icons as Sir Wink from design system SVG ([feff4b4](https://github.com/stratif-io/tofa/commit/feff4b42356cc9e9add6f46fc612e3e1bedfff1e))
* **tofa-app:** scaffold Tauri v2 workspace crate ([274646a](https://github.com/stratif-io/tofa/commit/274646aa81cf13b37b6167f10d195ecdf518b337))
* **tofa-app:** tray icon, popover window, focus-to-close ([2fdc31b](https://github.com/stratif-io/tofa/commit/2fdc31b5606e8b49a7bf91019abcf223b65e1cbf))
* **tray:** show 'Tofa DEV' tooltip in debug builds ([e2b17fc](https://github.com/stratif-io/tofa/commit/e2b17fc0f404c5ed7669f47ea99cd686f03c4c47))
* **tray:** switch icon between locked/open states and fix multi-display positioning ([d09b54e](https://github.com/stratif-io/tofa/commit/d09b54ed972a2b630093db2e580dc25a3d499998))
* **tray:** use colored purple shield icon instead of black template ([6eee419](https://github.com/stratif-io/tofa/commit/6eee4190dadb12207ef5fb949fc9aa25c8de9dec))
* **tui,app:** accept pasted list of otpauth:// URIs as bulk import ([0eb62a8](https://github.com/stratif-io/tofa/commit/0eb62a84f696b1ddc8272211fd490ac2b32e1dd8))
* **tui,app:** list-of-otpauth multi-QR export in TUI and Tauri app ([316fdce](https://github.com/stratif-io/tofa/commit/316fdce3cce3847591907e1c3c7b54347b39bad1))


### Bug Fixes

* **app:** add NSScreenCaptureUsageDescription to enable screen recording permission ([64c9ab6](https://github.com/stratif-io/tofa/commit/64c9ab6fe9bbe6a195b0222ec5e8b8e509bcab73))
* **app:** add NSScreenCaptureUsageDescription to enable screen recording permission ([368a537](https://github.com/stratif-io/tofa/commit/368a53701b4dc87f9686f9657b82721284422597))
* **app:** allow About window IPC + simplify CheckResult::from_status visibility ([ecef10a](https://github.com/stratif-io/tofa/commit/ecef10a9a391a64d31efb463b47c1413ac5d12c4))
* **app:** call CGRequestScreenCaptureAccess before screencapture on macOS Sequoia ([b6e197c](https://github.com/stratif-io/tofa/commit/b6e197ccfe4e9c21a9ef4c957589e241e7e2281d))
* **app:** center About content via body flex with content-sized container ([53a5ef6](https://github.com/stratif-io/tofa/commit/53a5ef67663e27d1d095f119de969f66b695eb0a))
* **app:** correct About icon glyph + use system fonts (no Google Fonts dependency) ([bd4c754](https://github.com/stratif-io/tofa/commit/bd4c7548fa2b497a36731781ef314abab7052155))
* **app:** enable createUpdaterArtifacts; --bundles updater is not a valid Tauri 2 target ([dfd0c0e](https://github.com/stratif-io/tofa/commit/dfd0c0e1dc52f1d5769b41b569270bfffd2d9de2))
* **app:** ensure icon PNGs are RGBA for Tauri compatibility ([d402081](https://github.com/stratif-io/tofa/commit/d4020813f423b6c92a7d8ab4a1c85ec1e050022e))
* **app:** increase About window height so Download button is fully visible ([85061b5](https://github.com/stratif-io/tofa/commit/85061b5d69d5160fc7e555e47febeb1580fd0277))
* **app:** inline About icon SVG to avoid external load + use centered shape ([4804640](https://github.com/stratif-io/tofa/commit/4804640c0488a7b504f3f80dcc6010543c7ceb17))
* **app:** make Quit menu item actually exit the process ([979c9c8](https://github.com/stratif-io/tofa/commit/979c9c8115c36bd96aec1f0ec960c76eb0dda71e))
* **app:** make Quit menu item actually exit the process ([9702a98](https://github.com/stratif-io/tofa/commit/9702a98796d1bdd10a417ad3f1ba7b18ecb77162))
* **app:** move popover to active Space instead of switching desktop ([9f8ad5e](https://github.com/stratif-io/tofa/commit/9f8ad5e69495ca343a56872871ef4c3c72dcadc4))
* **app:** move popover to active Space on click instead of switching desktop ([63ffcb3](https://github.com/stratif-io/tofa/commit/63ffcb3f56c8669b84666270af35f0b02cf75448))
* **app:** popover follows tray to secondary screens, stays open durin… ([9d460d3](https://github.com/stratif-io/tofa/commit/9d460d3df9fb01c4bb4f64774ef36c0ff971876a))
* **app:** popover follows tray to secondary screens, stays open during dialogs ([fee84ad](https://github.com/stratif-io/tofa/commit/fee84ad41b9e9d271cbdad27b9edef57744958c2))
* **app:** position popover flush below tray on any display ([d85d8bd](https://github.com/stratif-io/tofa/commit/d85d8bd3b6d6194733d5914daf23096499574b87))
* **app:** position popover flush below tray on any display ([f71b6f2](https://github.com/stratif-io/tofa/commit/f71b6f234ba98d3cd359b3e1c0d7bdfc67f16814))
* **app:** resolve trigger-download race + ensure tauri.conf.json newline ([5e9f74a](https://github.com/stratif-io/tofa/commit/5e9f74ac8f0816ec0c106d7ca01b6cd953cc141c))
* **app:** set bundle identifier to io.stratif.tofa ([a21344d](https://github.com/stratif-io/tofa/commit/a21344d66a13284be2f5c46bbe1e664c269fb9a8))
* **app:** set bundle identifier to io.stratif.tofa ([2002c45](https://github.com/stratif-io/tofa/commit/2002c456cfc0f70dfb133d30eba19869dc93671b))
* **app:** set popover position via NSWindow.setFrameTopLeftPoint directly ([4b44998](https://github.com/stratif-io/tofa/commit/4b449987372d6298f91863362d172575d7e23c1f))
* **app:** show '------' instead of erroring when TOTP secret is invalid ([7bd4cf0](https://github.com/stratif-io/tofa/commit/7bd4cf072e3c52c3a803c3334c0d11580597d66f))
* **app:** sync tauri.conf.json version to 0.5.3 ([767650a](https://github.com/stratif-io/tofa/commit/767650a34abaaa63c22ec253ea623c97353f6223))
* **app:** sync tauri.conf.json version to 0.6.1 ([666a178](https://github.com/stratif-io/tofa/commit/666a178c6dd7b6597c8556a9e00375889f8af2ed))
* **app:** tighten About window to 320x400 with snug content ([fcabc70](https://github.com/stratif-io/tofa/commit/fcabc70d4662ab92fe28ec6bc1dc0520e53db398))
* **app:** tighten About window to fit content snugly ([cbcb11c](https://github.com/stratif-io/tofa/commit/cbcb11ca9a39661cbf7ee2491f03b04459812c85))
* **app:** use clean wink SVG for About icon (no asymmetric platter) ([1959e2c](https://github.com/stratif-io/tofa/commit/1959e2cf421d8d958d812b986b92c03833a52d1f))
* **app:** use dmg_icon.svg (with black background) for app icon ([79d67ea](https://github.com/stratif-io/tofa/commit/79d67eaa1d965ee06cbe9b9bd2b0e63cd4613f91))
* **app:** use full black square background for app icon ([02e4f37](https://github.com/stratif-io/tofa/commit/02e4f37148019f6561ac8edcd9cc2483166634af))
* **app:** use native Tauri dialog for QR image import ([a8c08a9](https://github.com/stratif-io/tofa/commit/a8c08a98ef921b8d6c32893be122d08a0bd5a69f))
* **app:** use native Tauri dialog for QR image import ([5e50f00](https://github.com/stratif-io/tofa/commit/5e50f00754f7d429bb42684414c457af7d0a788e))
* **app:** use tray click position (not enum-wrapped rect) to find target monitor ([4e58713](https://github.com/stratif-io/tofa/commit/4e587139e3efb31e0a09b7d3af442dc8b9f5d3b7))
* **app:** use tray click position to find target monitor ([cb45998](https://github.com/stratif-io/tofa/commit/cb45998a51fa1d60fdd8cb2d50c009ac9dfc2b72))
* **app:** vertically center About window content ([17a8834](https://github.com/stratif-io/tofa/commit/17a88345ea62657228a5faceb4b9b314e357ff55))
* CI fixes, test fixes, project hygiene and guidelines ([ff87ab4](https://github.com/stratif-io/tofa/commit/ff87ab46329c54b48f6aa388664e96d43cb098df))
* **ci:** align DMG icon positions with background platforms ([a0b047c](https://github.com/stratif-io/tofa/commit/a0b047cac5f93f465383409951384f05acfe4c15))
* **ci:** correct DMG path and bump app version to 0.2.0 ([57e976c](https://github.com/stratif-io/tofa/commit/57e976ca20ee0605f2b44c4a70725d02ba4cd20b))
* **ci:** correct DMG path and bump app version to 0.2.0 ([2a7427e](https://github.com/stratif-io/tofa/commit/2a7427e12bed3c6d6c719f757da2d676702a8dc9))
* **ci:** resolve merge conflict — use version 0.2.1 from main ([be001d6](https://github.com/stratif-io/tofa/commit/be001d60598077c46bc232500886c73c09f7f77d))
* **ci:** sync tauri.conf.json version and add it to release-please extra-files ([775716f](https://github.com/stratif-io/tofa/commit/775716f43e5d42fcf7f6c1cddee49f20cc547ef1))
* **cli:** emit full otpauth URI from `tofa qr <name>` ([305068a](https://github.com/stratif-io/tofa/commit/305068ad5548e79f8d77a5852f1e3f675054fdbe))
* **core,app:** centralise dedup in Vault::add_entry_if_unique ([7b97628](https://github.com/stratif-io/tofa/commit/7b97628395be94064ffb12aafd719561b1dbd52b))
* **core:** preserve per-account algorithm and digits in migration export ([49641a8](https://github.com/stratif-io/tofa/commit/49641a8dcd0ca1885db7d9c56f471301c6b2a7ff))
* **dmg:** move dmg config under macOS key ([dceee44](https://github.com/stratif-io/tofa/commit/dceee447d99cc3480362355a8a598266bb6b297e))
* **dmg:** reduce window to 528x320 and wire background image ([abaac2a](https://github.com/stratif-io/tofa/commit/abaac2ae3aa5f573fa39cc38417053bd7229a454))
* proper 8-digit code formatting (XXXX XXXX) in TUI and Tauri app ([acf2582](https://github.com/stratif-io/tofa/commit/acf2582104b15e7cacaea5ec0b508264b0c11f71))
* remove unused is_locked method from PassphraseCache ([7b7257f](https://github.com/stratif-io/tofa/commit/7b7257f1a927448c08a30fff36a3200c21217806))
* repair broken 0.4.0 publish, harden release pipeline, preserve OTP params in migration export ([654a6a9](https://github.com/stratif-io/tofa/commit/654a6a910f1bb954ebb2030b4aac29e19bb74ca0))
* sync tauri.conf.json version + fix release-please path so it stays in sync ([36e55f9](https://github.com/stratif-io/tofa/commit/36e55f9a9364e8a894b921392687c6225ed51822))
* **tauri:** update vault path in memory and lock on settings save ([36113f6](https://github.com/stratif-io/tofa/commit/36113f6d8d686aff247521b9ffe2894603b79963))
* **tofa-app:** CFBundleDisplayName=Tofa in Info.plist, fixed-width unlock form for centering ([9385899](https://github.com/stratif-io/tofa/commit/9385899e7fbb6f7ec9f123082ad1ca293cf3930e))
* **tofa-app:** distinct error messages for vault-not-found vs wrong passphrase ([28841da](https://github.com/stratif-io/tofa/commit/28841da4f80a09c69bba40e320a793aeb40e60f1))
* **tofa-app:** enable withGlobalTauri so window.__TAURI__ is available ([68e95ba](https://github.com/stratif-io/tofa/commit/68e95bafcd453ff5f5421ea1bac6a6323a859862))
* **tofa-app:** rename to Tofa, hide Dock icon via set_activation_policy, center unlock form ([369d844](https://github.com/stratif-io/tofa/commit/369d8445a784f26d9ebf225187f5aa68a26c4f91))
* **tofa-app:** space-format code in copy_code, guard slice indexing ([1162f55](https://github.com/stratif-io/tofa/commit/1162f5575a5e35b82b48d48fab17580144390793))
* **tofa-app:** use dirs::config_dir() for vault path to match CLI ([c5bc7f3](https://github.com/stratif-io/tofa/commit/c5bc7f3b46c09edaf8b443b7146e2d985b33eb0e))
* **tofa-app:** zeroize passphrase before re-unlock, self-enforce TTL expiry ([c1e6516](https://github.com/stratif-io/tofa/commit/c1e6516cfd8d9ca6b2b6321bad6cefdb5bbab6d4))
* **tray:** remove transparent padding from tray icons ([e5c1f86](https://github.com/stratif-io/tofa/commit/e5c1f86102667b216daa3d79d13760e3fe98b78a))

## [0.10.0](https://github.com/stratif-io/tofa/compare/tofa-macos-v0.9.0...tofa-macos-v0.10.0) (2026-05-11)


### Features

* About window + silent auto-update via tauri-plugin-updater ([#76](https://github.com/stratif-io/tofa/pull/76))

## [0.9.0](https://github.com/stratif-io/tofa/compare/tofa-macos-v0.8.0...tofa-macos-v0.9.0) (2026-05-07)


### Miscellaneous

* version bumped to keep the linked-versions group aligned with tofa-core 0.9.0; no app code changes since 0.8.0.

## [0.8.0](https://github.com/stratif-io/tofa/compare/tofa-macos-v0.7.0...tofa-macos-v0.8.0) (2026-05-07)


### Features

* **app:** copy_uri and save_uri_list with matching UI buttons ([fe4da0f](https://github.com/stratif-io/tofa/commit/fe4da0fd19ea9d22a3a8c39a0537c94e99d8a79c))
* **app:** highlight non-30s entries + bundle Save All into a single zip ([675fd3e](https://github.com/stratif-io/tofa/commit/675fd3e78688a4430a5beb98c02765e94ca96807))
* **app:** highlight non-30s entries + bundle Save All into a single zip ([8fe922c](https://github.com/stratif-io/tofa/commit/8fe922cd351d21cc06a9d881bc49e974c105f554))
* **app:** pick_and_import_file accepts a multi-file selection ([f489502](https://github.com/stratif-io/tofa/commit/f489502ce76545fcb65d480132f224886f8e4c0a))
* **app:** show otpauth URI on the entry detail view ([3ab079f](https://github.com/stratif-io/tofa/commit/3ab079f1c2f797fc7594c5733b812b3804d14f03))
* **cli,tui,app:** route every import surface through the unified dispatcher ([5591dcb](https://github.com/stratif-io/tofa/commit/5591dcbfeb7e80d0b425ef8c045c0b9afa480e1c))
* import dispatcher, URI export, dedup, and architectural audit pass ([9d8f2d4](https://github.com/stratif-io/tofa/commit/9d8f2d4432b59c2b43016e67792013a9c1be3cfa))
* **tui,app:** accept pasted list of otpauth:// URIs as bulk import ([0eb62a8](https://github.com/stratif-io/tofa/commit/0eb62a84f696b1ddc8272211fd490ac2b32e1dd8))


### Bug Fixes

* **core,app:** centralise dedup in Vault::add_entry_if_unique ([7b97628](https://github.com/stratif-io/tofa/commit/7b97628395be94064ffb12aafd719561b1dbd52b))

## [0.7.0](https://github.com/stratif-io/tofa/compare/tofa-macos-v0.6.2...tofa-macos-v0.7.0) (2026-05-06)


### Features

* **core:** build_selection_uri dispatches by selection shape ([6695d10](https://github.com/stratif-io/tofa/commit/6695d106b119d36fe3811034ce05569bc457c6f5))
* full-fidelity QR export rules across CLI, TUI, and Tauri app ([b0cb6de](https://github.com/stratif-io/tofa/commit/b0cb6decb0820186368619ff9f09d420c94b64ba))
* **tui,app:** list-of-otpauth multi-QR export in TUI and Tauri app ([316fdce](https://github.com/stratif-io/tofa/commit/316fdce3cce3847591907e1c3c7b54347b39bad1))


### Bug Fixes

* **cli:** emit full otpauth URI from `tofa qr <name>` ([305068a](https://github.com/stratif-io/tofa/commit/305068ad5548e79f8d77a5852f1e3f675054fdbe))

## [0.6.2](https://github.com/stratif-io/tofa/compare/tofa-macos-v0.6.1...tofa-macos-v0.6.2) (2026-05-06)


### Bug Fixes

* **app:** sync tauri.conf.json version to 0.6.1 ([666a178](https://github.com/stratif-io/tofa/commit/666a178c6dd7b6597c8556a9e00375889f8af2ed))
* sync tauri.conf.json version + fix release-please path so it stays in sync ([36e55f9](https://github.com/stratif-io/tofa/commit/36e55f9a9364e8a894b921392687c6225ed51822))

## [0.6.1](https://github.com/stratif-io/tofa/compare/tofa-macos-v0.6.0...tofa-macos-v0.6.1) (2026-05-06)


### Bug Fixes

* **core:** preserve per-account algorithm and digits in migration export ([49641a8](https://github.com/stratif-io/tofa/commit/49641a8dcd0ca1885db7d9c56f471301c6b2a7ff))
* repair broken 0.4.0 publish, harden release pipeline, preserve OTP params in migration export ([654a6a9](https://github.com/stratif-io/tofa/commit/654a6a910f1bb954ebb2030b4aac29e19bb74ca0))

## [0.6.0](https://github.com/stratif-io/tofa/compare/tofa-macos-v0.5.3...tofa-macos-v0.6.0) (2026-05-05)


### Features

* **app:** disable scan screen (coming soon), add QR export for single and multiple accounts ([ea17dba](https://github.com/stratif-io/tofa/commit/ea17dbad0b54645678f1a71cd375f94b5a7b0aff))
* **app:** QR export for single/multiple accounts + disable scan screen ([7ae2229](https://github.com/stratif-io/tofa/commit/7ae2229e830e170318f694e50d3752ae305262d6))
* **app:** save QR PNG via native save dialog instead of browser download ([5e6a28c](https://github.com/stratif-io/tofa/commit/5e6a28c4361452ac677e26973c3769221d96d035))


### Bug Fixes

* **app:** sync tauri.conf.json version to 0.5.3 ([767650a](https://github.com/stratif-io/tofa/commit/767650a34abaaa63c22ec253ea623c97353f6223))

## [0.5.3](https://github.com/stratif-io/tofa/compare/tofa-macos-v0.5.2...tofa-macos-v0.5.3) (2026-05-05)


### Bug Fixes

* **app:** set bundle identifier to io.stratif.tofa ([a21344d](https://github.com/stratif-io/tofa/commit/a21344d66a13284be2f5c46bbe1e664c269fb9a8))
* **app:** set bundle identifier to io.stratif.tofa ([2002c45](https://github.com/stratif-io/tofa/commit/2002c456cfc0f70dfb133d30eba19869dc93671b))

## [0.5.2](https://github.com/stratif-io/tofa/compare/tofa-macos-v0.5.1...tofa-macos-v0.5.2) (2026-05-05)


### Bug Fixes

* **app:** call CGRequestScreenCaptureAccess before screencapture on macOS Sequoia ([b6e197c](https://github.com/stratif-io/tofa/commit/b6e197ccfe4e9c21a9ef4c957589e241e7e2281d))
* **app:** ensure icon PNGs are RGBA for Tauri compatibility ([d402081](https://github.com/stratif-io/tofa/commit/d4020813f423b6c92a7d8ab4a1c85ec1e050022e))
* **app:** use dmg_icon.svg (with black background) for app icon ([79d67ea](https://github.com/stratif-io/tofa/commit/79d67eaa1d965ee06cbe9b9bd2b0e63cd4613f91))
* **app:** use full black square background for app icon ([02e4f37](https://github.com/stratif-io/tofa/commit/02e4f37148019f6561ac8edcd9cc2483166634af))
* **ci:** sync tauri.conf.json version and add it to release-please extra-files ([775716f](https://github.com/stratif-io/tofa/commit/775716f43e5d42fcf7f6c1cddee49f20cc547ef1))

## [0.5.1](https://github.com/stratif-io/tofa/compare/tofa-macos-v0.5.0...tofa-macos-v0.5.1) (2026-05-05)


### Bug Fixes

* **app:** add NSScreenCaptureUsageDescription to enable screen recording permission ([64c9ab6](https://github.com/stratif-io/tofa/commit/64c9ab6fe9bbe6a195b0222ec5e8b8e509bcab73))
* **app:** add NSScreenCaptureUsageDescription to enable screen recording permission ([368a537](https://github.com/stratif-io/tofa/commit/368a53701b4dc87f9686f9657b82721284422597))

## [0.5.0](https://github.com/stratif-io/tofa/compare/tofa-macos-v0.4.2...tofa-macos-v0.5.0) (2026-05-05)


### Features

* **app:** accept all import formats via Open File button ([f30efd0](https://github.com/stratif-io/tofa/commit/f30efd08f519203d529f98e0b8d0cd5d0c47cbb8))
* **app:** redesign UI — settings page, nav bar, TOFA branding, icon regen ([aa7edf9](https://github.com/stratif-io/tofa/commit/aa7edf9900bfb568c393b4f85d80499c26db4436))
* multi-format import, UI redesign, Homebrew tap, TOFA branding ([30b7fd9](https://github.com/stratif-io/tofa/commit/30b7fd99276ead2450ee02e59304598b67fd1bfb))
* **tray:** show 'Tofa DEV' tooltip in debug builds ([e2b17fc](https://github.com/stratif-io/tofa/commit/e2b17fc0f404c5ed7669f47ea99cd686f03c4c47))
* **tray:** switch icon between locked/open states and fix multi-display positioning ([d09b54e](https://github.com/stratif-io/tofa/commit/d09b54ed972a2b630093db2e580dc25a3d499998))
* **tray:** use colored purple shield icon instead of black template ([6eee419](https://github.com/stratif-io/tofa/commit/6eee4190dadb12207ef5fb949fc9aa25c8de9dec))


### Bug Fixes

* **app:** show '------' instead of erroring when TOTP secret is invalid ([7bd4cf0](https://github.com/stratif-io/tofa/commit/7bd4cf072e3c52c3a803c3334c0d11580597d66f))
* **dmg:** move dmg config under macOS key ([dceee44](https://github.com/stratif-io/tofa/commit/dceee447d99cc3480362355a8a598266bb6b297e))
* **dmg:** reduce window to 528x320 and wire background image ([abaac2a](https://github.com/stratif-io/tofa/commit/abaac2ae3aa5f573fa39cc38417053bd7229a454))
* **tray:** remove transparent padding from tray icons ([e5c1f86](https://github.com/stratif-io/tofa/commit/e5c1f86102667b216daa3d79d13760e3fe98b78a))

## [0.4.2](https://github.com/stratif-io/tofa/compare/tofa-macos-v0.4.1...tofa-macos-v0.4.2) (2026-05-04)


### Bug Fixes

* **app:** position popover flush below tray on any display ([d85d8bd](https://github.com/stratif-io/tofa/commit/d85d8bd3b6d6194733d5914daf23096499574b87))
* **app:** position popover flush below tray on any display ([f71b6f2](https://github.com/stratif-io/tofa/commit/f71b6f234ba98d3cd359b3e1c0d7bdfc67f16814))
* **app:** use native Tauri dialog for QR image import ([a8c08a9](https://github.com/stratif-io/tofa/commit/a8c08a98ef921b8d6c32893be122d08a0bd5a69f))
* **app:** use native Tauri dialog for QR image import ([5e50f00](https://github.com/stratif-io/tofa/commit/5e50f00754f7d429bb42684414c457af7d0a788e))

## [0.4.1](https://github.com/stratif-io/tofa/compare/tofa-macos-v0.4.0...tofa-macos-v0.4.1) (2026-05-04)


### Bug Fixes

* **app:** popover follows tray to secondary screens, stays open durin… ([9d460d3](https://github.com/stratif-io/tofa/commit/9d460d3df9fb01c4bb4f64774ef36c0ff971876a))
* **app:** popover follows tray to secondary screens, stays open during dialogs ([fee84ad](https://github.com/stratif-io/tofa/commit/fee84ad41b9e9d271cbdad27b9edef57744958c2))
* **app:** set popover position via NSWindow.setFrameTopLeftPoint directly ([4b44998](https://github.com/stratif-io/tofa/commit/4b449987372d6298f91863362d172575d7e23c1f))
* **app:** use tray click position (not enum-wrapped rect) to find target monitor ([4e58713](https://github.com/stratif-io/tofa/commit/4e587139e3efb31e0a09b7d3af442dc8b9f5d3b7))
* **app:** use tray click position to find target monitor ([cb45998](https://github.com/stratif-io/tofa/commit/cb45998a51fa1d60fdd8cb2d50c009ac9dfc2b72))
* **ci:** align DMG icon positions with background platforms ([a0b047c](https://github.com/stratif-io/tofa/commit/a0b047cac5f93f465383409951384f05acfe4c15))

## [0.4.0](https://github.com/stratif-io/tofa/compare/tofa-macos-v0.3.0...tofa-macos-v0.4.0) (2026-05-04)


### Features

* **app:** black icon background, custom DMG with arrow and drag-to-install ([ac3a8fa](https://github.com/stratif-io/tofa/commit/ac3a8fa65cf13440374676c463a68b82288b83ec))
* **app:** black icon background, custom DMG with drag-to-install ([2ee3549](https://github.com/stratif-io/tofa/commit/2ee35490788ef413fb90945405413857f5dd8d56))
* **app:** dark/light mode, multi-format import, settings improvements ([494913b](https://github.com/stratif-io/tofa/commit/494913bed231e46fdff192078455e46033181370))
* smooth rAF progress bar, 10s API refresh, multi-QR screen scan, scan feedback ([f07fb1e](https://github.com/stratif-io/tofa/commit/f07fb1e64be9f638d82228309c9f6cc246661a6b))
* **tauri:** card click opens detail modal, clipboard icon per entry, no auto-hide ([557af15](https://github.com/stratif-io/tofa/commit/557af150c1bf3a30302237fa691342540e3ccbcc))
* **tauri:** delete button on hover ([df3aaf9](https://github.com/stratif-io/tofa/commit/df3aaf9d9076d4ce225ef7b7049b14f5ea62109e))
* **tauri:** display issuer and account separately in entry list ([0d664c7](https://github.com/stratif-io/tofa/commit/0d664c7f04d16c8a3bbf9325b714b3d4ebcd26d1))
* **tauri:** full Tauri app UI — loader, drag-drop, scan feedback, lock/unlock UX ([7d5fde0](https://github.com/stratif-io/tofa/commit/7d5fde0449aa31770a486d04083c095c7c105c40))
* **tauri:** hide Dock icon — menu bar only (activationPolicy accessory) ([a4e33a4](https://github.com/stratif-io/tofa/commit/a4e33a4c1da2e36737156709bc34bd13461c16d3))
* **tauri:** scan screen + camera QR, tray context menu ([69d336b](https://github.com/stratif-io/tofa/commit/69d336b00fe3946699589f4f5ddc0a2f44b6ca32))
* **tauri:** show OTP metadata tooltip on hover ([46878ad](https://github.com/stratif-io/tofa/commit/46878ad819efadc66be1fca762af708d5300d174))
* **tofa-app:** implement unlock, get_entries, copy_code, settings commands ([525ea07](https://github.com/stratif-io/tofa/commit/525ea070dc8d6a46bbf141cdc2ea106200cfead8))
* **tofa-app:** PassphraseCache with 10 min TTL, AppState, command stubs ([bab112d](https://github.com/stratif-io/tofa/commit/bab112dfc152b51e0722db62a24b27b2f81ad73c))
* **tofa-app:** regenerate all icons as Sir Wink from design system SVG ([feff4b4](https://github.com/stratif-io/tofa/commit/feff4b42356cc9e9add6f46fc612e3e1bedfff1e))
* **tofa-app:** scaffold Tauri v2 workspace crate ([274646a](https://github.com/stratif-io/tofa/commit/274646aa81cf13b37b6167f10d195ecdf518b337))
* **tofa-app:** tray icon, popover window, focus-to-close ([2fdc31b](https://github.com/stratif-io/tofa/commit/2fdc31b5606e8b49a7bf91019abcf223b65e1cbf))


### Bug Fixes

* **app:** make Quit menu item actually exit the process ([979c9c8](https://github.com/stratif-io/tofa/commit/979c9c8115c36bd96aec1f0ec960c76eb0dda71e))
* **app:** make Quit menu item actually exit the process ([9702a98](https://github.com/stratif-io/tofa/commit/9702a98796d1bdd10a417ad3f1ba7b18ecb77162))
* **app:** move popover to active Space instead of switching desktop ([9f8ad5e](https://github.com/stratif-io/tofa/commit/9f8ad5e69495ca343a56872871ef4c3c72dcadc4))
* **app:** move popover to active Space on click instead of switching desktop ([63ffcb3](https://github.com/stratif-io/tofa/commit/63ffcb3f56c8669b84666270af35f0b02cf75448))
* CI fixes, test fixes, project hygiene and guidelines ([ff87ab4](https://github.com/stratif-io/tofa/commit/ff87ab46329c54b48f6aa388664e96d43cb098df))
* **ci:** correct DMG path and bump app version to 0.2.0 ([57e976c](https://github.com/stratif-io/tofa/commit/57e976ca20ee0605f2b44c4a70725d02ba4cd20b))
* **ci:** correct DMG path and bump app version to 0.2.0 ([2a7427e](https://github.com/stratif-io/tofa/commit/2a7427e12bed3c6d6c719f757da2d676702a8dc9))
* **ci:** resolve merge conflict — use version 0.2.1 from main ([be001d6](https://github.com/stratif-io/tofa/commit/be001d60598077c46bc232500886c73c09f7f77d))
* proper 8-digit code formatting (XXXX XXXX) in TUI and Tauri app ([acf2582](https://github.com/stratif-io/tofa/commit/acf2582104b15e7cacaea5ec0b508264b0c11f71))
* remove unused is_locked method from PassphraseCache ([7b7257f](https://github.com/stratif-io/tofa/commit/7b7257f1a927448c08a30fff36a3200c21217806))
* **tauri:** update vault path in memory and lock on settings save ([36113f6](https://github.com/stratif-io/tofa/commit/36113f6d8d686aff247521b9ffe2894603b79963))
* **tofa-app:** CFBundleDisplayName=Tofa in Info.plist, fixed-width unlock form for centering ([9385899](https://github.com/stratif-io/tofa/commit/9385899e7fbb6f7ec9f123082ad1ca293cf3930e))
* **tofa-app:** distinct error messages for vault-not-found vs wrong passphrase ([28841da](https://github.com/stratif-io/tofa/commit/28841da4f80a09c69bba40e320a793aeb40e60f1))
* **tofa-app:** enable withGlobalTauri so window.__TAURI__ is available ([68e95ba](https://github.com/stratif-io/tofa/commit/68e95bafcd453ff5f5421ea1bac6a6323a859862))
* **tofa-app:** rename to Tofa, hide Dock icon via set_activation_policy, center unlock form ([369d844](https://github.com/stratif-io/tofa/commit/369d8445a784f26d9ebf225187f5aa68a26c4f91))
* **tofa-app:** space-format code in copy_code, guard slice indexing ([1162f55](https://github.com/stratif-io/tofa/commit/1162f5575a5e35b82b48d48fab17580144390793))
* **tofa-app:** use dirs::config_dir() for vault path to match CLI ([c5bc7f3](https://github.com/stratif-io/tofa/commit/c5bc7f3b46c09edaf8b443b7146e2d985b33eb0e))
* **tofa-app:** zeroize passphrase before re-unlock, self-enforce TTL expiry ([c1e6516](https://github.com/stratif-io/tofa/commit/c1e6516cfd8d9ca6b2b6321bad6cefdb5bbab6d4))

## [0.3.0](https://github.com/cabichahine/tofa/compare/Tofa macOS-v0.2.2...Tofa macOS-v0.3.0) (2026-05-04)


### Features

* **app:** black icon background, custom DMG with arrow and drag-to-install ([ac3a8fa](https://github.com/cabichahine/tofa/commit/ac3a8fa65cf13440374676c463a68b82288b83ec))
* **app:** black icon background, custom DMG with drag-to-install ([2ee3549](https://github.com/cabichahine/tofa/commit/2ee35490788ef413fb90945405413857f5dd8d56))
* **app:** dark/light mode, multi-format import, settings improvements ([494913b](https://github.com/cabichahine/tofa/commit/494913bed231e46fdff192078455e46033181370))
* smooth rAF progress bar, 10s API refresh, multi-QR screen scan, scan feedback ([f07fb1e](https://github.com/cabichahine/tofa/commit/f07fb1e64be9f638d82228309c9f6cc246661a6b))
* **tauri:** card click opens detail modal, clipboard icon per entry, no auto-hide ([557af15](https://github.com/cabichahine/tofa/commit/557af150c1bf3a30302237fa691342540e3ccbcc))
* **tauri:** delete button on hover ([df3aaf9](https://github.com/cabichahine/tofa/commit/df3aaf9d9076d4ce225ef7b7049b14f5ea62109e))
* **tauri:** display issuer and account separately in entry list ([0d664c7](https://github.com/cabichahine/tofa/commit/0d664c7f04d16c8a3bbf9325b714b3d4ebcd26d1))
* **tauri:** full Tauri app UI — loader, drag-drop, scan feedback, lock/unlock UX ([7d5fde0](https://github.com/cabichahine/tofa/commit/7d5fde0449aa31770a486d04083c095c7c105c40))
* **tauri:** hide Dock icon — menu bar only (activationPolicy accessory) ([a4e33a4](https://github.com/cabichahine/tofa/commit/a4e33a4c1da2e36737156709bc34bd13461c16d3))
* **tauri:** scan screen + camera QR, tray context menu ([69d336b](https://github.com/cabichahine/tofa/commit/69d336b00fe3946699589f4f5ddc0a2f44b6ca32))
* **tauri:** show OTP metadata tooltip on hover ([46878ad](https://github.com/cabichahine/tofa/commit/46878ad819efadc66be1fca762af708d5300d174))
* **tofa-app:** implement unlock, get_entries, copy_code, settings commands ([525ea07](https://github.com/cabichahine/tofa/commit/525ea070dc8d6a46bbf141cdc2ea106200cfead8))
* **tofa-app:** PassphraseCache with 10 min TTL, AppState, command stubs ([bab112d](https://github.com/cabichahine/tofa/commit/bab112dfc152b51e0722db62a24b27b2f81ad73c))
* **tofa-app:** regenerate all icons as Sir Wink from design system SVG ([feff4b4](https://github.com/cabichahine/tofa/commit/feff4b42356cc9e9add6f46fc612e3e1bedfff1e))
* **tofa-app:** scaffold Tauri v2 workspace crate ([274646a](https://github.com/cabichahine/tofa/commit/274646aa81cf13b37b6167f10d195ecdf518b337))
* **tofa-app:** tray icon, popover window, focus-to-close ([2fdc31b](https://github.com/cabichahine/tofa/commit/2fdc31b5606e8b49a7bf91019abcf223b65e1cbf))


### Bug Fixes

* CI fixes, test fixes, project hygiene and guidelines ([ff87ab4](https://github.com/cabichahine/tofa/commit/ff87ab46329c54b48f6aa388664e96d43cb098df))
* **ci:** correct DMG path and bump app version to 0.2.0 ([57e976c](https://github.com/cabichahine/tofa/commit/57e976ca20ee0605f2b44c4a70725d02ba4cd20b))
* **ci:** correct DMG path and bump app version to 0.2.0 ([2a7427e](https://github.com/cabichahine/tofa/commit/2a7427e12bed3c6d6c719f757da2d676702a8dc9))
* **ci:** resolve merge conflict — use version 0.2.1 from main ([be001d6](https://github.com/cabichahine/tofa/commit/be001d60598077c46bc232500886c73c09f7f77d))
* proper 8-digit code formatting (XXXX XXXX) in TUI and Tauri app ([acf2582](https://github.com/cabichahine/tofa/commit/acf2582104b15e7cacaea5ec0b508264b0c11f71))
* remove unused is_locked method from PassphraseCache ([7b7257f](https://github.com/cabichahine/tofa/commit/7b7257f1a927448c08a30fff36a3200c21217806))
* **tauri:** update vault path in memory and lock on settings save ([36113f6](https://github.com/cabichahine/tofa/commit/36113f6d8d686aff247521b9ffe2894603b79963))
* **tofa-app:** CFBundleDisplayName=Tofa in Info.plist, fixed-width unlock form for centering ([9385899](https://github.com/cabichahine/tofa/commit/9385899e7fbb6f7ec9f123082ad1ca293cf3930e))
* **tofa-app:** distinct error messages for vault-not-found vs wrong passphrase ([28841da](https://github.com/cabichahine/tofa/commit/28841da4f80a09c69bba40e320a793aeb40e60f1))
* **tofa-app:** enable withGlobalTauri so window.__TAURI__ is available ([68e95ba](https://github.com/cabichahine/tofa/commit/68e95bafcd453ff5f5421ea1bac6a6323a859862))
* **tofa-app:** rename to Tofa, hide Dock icon via set_activation_policy, center unlock form ([369d844](https://github.com/cabichahine/tofa/commit/369d8445a784f26d9ebf225187f5aa68a26c4f91))
* **tofa-app:** space-format code in copy_code, guard slice indexing ([1162f55](https://github.com/cabichahine/tofa/commit/1162f5575a5e35b82b48d48fab17580144390793))
* **tofa-app:** use dirs::config_dir() for vault path to match CLI ([c5bc7f3](https://github.com/cabichahine/tofa/commit/c5bc7f3b46c09edaf8b443b7146e2d985b33eb0e))
* **tofa-app:** zeroize passphrase before re-unlock, self-enforce TTL expiry ([c1e6516](https://github.com/cabichahine/tofa/commit/c1e6516cfd8d9ca6b2b6321bad6cefdb5bbab6d4))

## [0.2.2](https://github.com/cabichahine/tofa/compare/tofa-app-v0.2.1...tofa-app-v0.2.2) (2026-05-04)


### Bug Fixes

* **ci:** correct DMG path and bump app version to 0.2.0 ([57e976c](https://github.com/cabichahine/tofa/commit/57e976ca20ee0605f2b44c4a70725d02ba4cd20b))
* **ci:** correct DMG path and bump app version to 0.2.0 ([2a7427e](https://github.com/cabichahine/tofa/commit/2a7427e12bed3c6d6c719f757da2d676702a8dc9))
* **ci:** resolve merge conflict — use version 0.2.1 from main ([be001d6](https://github.com/cabichahine/tofa/commit/be001d60598077c46bc232500886c73c09f7f77d))

## [0.2.1](https://github.com/cabichahine/tofa/compare/tofa-app-v0.2.0...tofa-app-v0.2.1) (2026-05-04)


### Bug Fixes

* CI fixes, test fixes, project hygiene and guidelines ([ff87ab4](https://github.com/cabichahine/tofa/commit/ff87ab46329c54b48f6aa388664e96d43cb098df))

## [0.2.0](https://github.com/cabichahine/tofa/compare/tofa-app-v0.1.0...tofa-app-v0.2.0) (2026-05-04)


### Features

* **app:** dark/light mode, multi-format import, settings improvements ([494913b](https://github.com/cabichahine/tofa/commit/494913bed231e46fdff192078455e46033181370))
* smooth rAF progress bar, 10s API refresh, multi-QR screen scan, scan feedback ([f07fb1e](https://github.com/cabichahine/tofa/commit/f07fb1e64be9f638d82228309c9f6cc246661a6b))
* **tauri:** card click opens detail modal, clipboard icon per entry, no auto-hide ([557af15](https://github.com/cabichahine/tofa/commit/557af150c1bf3a30302237fa691342540e3ccbcc))
* **tauri:** delete button on hover ([df3aaf9](https://github.com/cabichahine/tofa/commit/df3aaf9d9076d4ce225ef7b7049b14f5ea62109e))
* **tauri:** display issuer and account separately in entry list ([0d664c7](https://github.com/cabichahine/tofa/commit/0d664c7f04d16c8a3bbf9325b714b3d4ebcd26d1))
* **tauri:** full Tauri app UI — loader, drag-drop, scan feedback, lock/unlock UX ([7d5fde0](https://github.com/cabichahine/tofa/commit/7d5fde0449aa31770a486d04083c095c7c105c40))
* **tauri:** hide Dock icon — menu bar only (activationPolicy accessory) ([a4e33a4](https://github.com/cabichahine/tofa/commit/a4e33a4c1da2e36737156709bc34bd13461c16d3))
* **tauri:** scan screen + camera QR, tray context menu ([69d336b](https://github.com/cabichahine/tofa/commit/69d336b00fe3946699589f4f5ddc0a2f44b6ca32))
* **tauri:** show OTP metadata tooltip on hover ([46878ad](https://github.com/cabichahine/tofa/commit/46878ad819efadc66be1fca762af708d5300d174))
* **tofa-app:** implement unlock, get_entries, copy_code, settings commands ([525ea07](https://github.com/cabichahine/tofa/commit/525ea070dc8d6a46bbf141cdc2ea106200cfead8))
* **tofa-app:** PassphraseCache with 10 min TTL, AppState, command stubs ([bab112d](https://github.com/cabichahine/tofa/commit/bab112dfc152b51e0722db62a24b27b2f81ad73c))
* **tofa-app:** regenerate all icons as Sir Wink from design system SVG ([feff4b4](https://github.com/cabichahine/tofa/commit/feff4b42356cc9e9add6f46fc612e3e1bedfff1e))
* **tofa-app:** scaffold Tauri v2 workspace crate ([274646a](https://github.com/cabichahine/tofa/commit/274646aa81cf13b37b6167f10d195ecdf518b337))
* **tofa-app:** tray icon, popover window, focus-to-close ([2fdc31b](https://github.com/cabichahine/tofa/commit/2fdc31b5606e8b49a7bf91019abcf223b65e1cbf))


### Bug Fixes

* proper 8-digit code formatting (XXXX XXXX) in TUI and Tauri app ([acf2582](https://github.com/cabichahine/tofa/commit/acf2582104b15e7cacaea5ec0b508264b0c11f71))
* remove unused is_locked method from PassphraseCache ([7b7257f](https://github.com/cabichahine/tofa/commit/7b7257f1a927448c08a30fff36a3200c21217806))
* **tauri:** update vault path in memory and lock on settings save ([36113f6](https://github.com/cabichahine/tofa/commit/36113f6d8d686aff247521b9ffe2894603b79963))
* **tofa-app:** CFBundleDisplayName=Tofa in Info.plist, fixed-width unlock form for centering ([9385899](https://github.com/cabichahine/tofa/commit/9385899e7fbb6f7ec9f123082ad1ca293cf3930e))
* **tofa-app:** distinct error messages for vault-not-found vs wrong passphrase ([28841da](https://github.com/cabichahine/tofa/commit/28841da4f80a09c69bba40e320a793aeb40e60f1))
* **tofa-app:** enable withGlobalTauri so window.__TAURI__ is available ([68e95ba](https://github.com/cabichahine/tofa/commit/68e95bafcd453ff5f5421ea1bac6a6323a859862))
* **tofa-app:** rename to Tofa, hide Dock icon via set_activation_policy, center unlock form ([369d844](https://github.com/cabichahine/tofa/commit/369d8445a784f26d9ebf225187f5aa68a26c4f91))
* **tofa-app:** space-format code in copy_code, guard slice indexing ([1162f55](https://github.com/cabichahine/tofa/commit/1162f5575a5e35b82b48d48fab17580144390793))
* **tofa-app:** use dirs::config_dir() for vault path to match CLI ([c5bc7f3](https://github.com/cabichahine/tofa/commit/c5bc7f3b46c09edaf8b443b7146e2d985b33eb0e))
* **tofa-app:** zeroize passphrase before re-unlock, self-enforce TTL expiry ([c1e6516](https://github.com/cabichahine/tofa/commit/c1e6516cfd8d9ca6b2b6321bad6cefdb5bbab6d4))
