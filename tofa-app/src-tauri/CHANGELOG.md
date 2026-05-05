# Changelog

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
