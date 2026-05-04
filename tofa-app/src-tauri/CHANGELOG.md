# Changelog

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
