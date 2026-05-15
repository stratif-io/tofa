# Changelog

## [0.11.3](https://github.com/stratif-io/tofa/compare/tofa-core-v0.11.2...tofa-core-v0.11.3) (2026-05-15)


### Bug Fixes

* **core:** use DEFAULT_PERIOD/DIGITS/ALGORITHM constants in VaultEntry::new ([896e0ef](https://github.com/stratif-io/tofa/commit/896e0efb473d776c1cd2d08798c92de6866c37ad))
* **core:** use DEFAULT_PERIOD/DIGITS/ALGORITHM constants in VaultEntry::new ([1c781a9](https://github.com/stratif-io/tofa/commit/1c781a9ce467d6df83fbe362e8fc9ab9dd60bc64))
* store default constants, must_use annotations, release cp collision ([#121](https://github.com/stratif-io/tofa/issues/121)) ([201f24e](https://github.com/stratif-io/tofa/commit/201f24ef9069144596b9e7397c697d9a67c27bd8))

## [0.11.2](https://github.com/stratif-io/tofa/compare/tofa-core-v0.11.1...tofa-core-v0.11.2) (2026-05-14)


### Bug Fixes

* **landing:** use node:22-alpine in Dockerfile for Astro v6 ([1ed6256](https://github.com/stratif-io/tofa/commit/1ed6256a0bb426b35f01ee56155f663c97ae4fd1))

## [0.11.1](https://github.com/stratif-io/tofa/compare/tofa-core-v0.11.0...tofa-core-v0.11.1) (2026-05-14)


### Bug Fixes

* **app:** vendor jsQR locally to remove CDN dependency ([5f9fc36](https://github.com/stratif-io/tofa/commit/5f9fc367247f596cece37e7cceaf7d06dbf0123d))
* **security:** vendor jsQR + self-host fonts; add AUDIT.md ([7a4ca32](https://github.com/stratif-io/tofa/commit/7a4ca32a4d1f606319dca441c705b51eba50f9f0))

## [0.11.0](https://github.com/stratif-io/tofa/compare/tofa-core-v0.10.0...tofa-core-v0.11.0) (2026-05-11)


### Features

* **app:** About window + silent auto-update via tauri-plugin-updater ([fad93aa](https://github.com/stratif-io/tofa/commit/fad93aa0be8c5140990a490bf3e544de125f4e4d))
* **ci:** add crates.io publish workflow and complete crate metadata ([3428908](https://github.com/stratif-io/tofa/commit/34289083e9c917eab298df202d467b1fd74afbed))
* **ci:** crates.io publish workflow and complete crate metadata ([62a26a0](https://github.com/stratif-io/tofa/commit/62a26a0c177fe812a2e702abb2caf84d0d9b9845))
* **cli,tui,app:** route every import surface through the unified dispatcher ([5591dcb](https://github.com/stratif-io/tofa/commit/5591dcbfeb7e80d0b425ef8c045c0b9afa480e1c))
* **core,cli:** expose per-pass scan progress and surface it in the spinner ([08da8af](https://github.com/stratif-io/tofa/commit/08da8af76513c7df8ce05dc95dbcdf02e81e6703))
* **core:** add build_otpauth_uri helper for single-entry exports ([ae6a79b](https://github.com/stratif-io/tofa/commit/ae6a79b8b7aa37b7b169a279cde3fd56dcf58ae4))
* **core:** add entries_to_uri_list for plain-text URI exports ([f326953](https://github.com/stratif-io/tofa/commit/f326953530f1bcc98097f473e32fadf78d8cb88b))
* **core:** add Google Authenticator migration URI import ([22d6ad7](https://github.com/stratif-io/tofa/commit/22d6ad7ca4513f760bff15e13fdcd4d79b6ab287))
* **core:** add import parsers for 2FAS, Raivo, Bitwarden, FreeOTP+, Ente Auth, KeePassXC ([d931a08](https://github.com/stratif-io/tofa/commit/d931a08d754b4a901f599afd4fb6a2c0a1fd30ca))
* **core:** add unified import::parse_file(path) dispatcher ([26c159d](https://github.com/stratif-io/tofa/commit/26c159d0129b59e48ad017e288c34daef9ec9ccc))
* **core:** build_selection_uri dispatches by selection shape ([6695d10](https://github.com/stratif-io/tofa/commit/6695d106b119d36fe3811034ce05569bc457c6f5))
* **core:** expose VERSION constant for downstream consumers ([2e93ff5](https://github.com/stratif-io/tofa/commit/2e93ff55ae953c1bd6fe52202b0b40a659f58bb0))
* **fixtures:** generate all 12 TOTP combinations (3 algo × 2 digits × 2 periods) ([2cfba10](https://github.com/stratif-io/tofa/commit/2cfba1015d597ac2cbb5239284e70e40f2caa92d))
* full-fidelity QR export rules across CLI, TUI, and Tauri app ([b0cb6de](https://github.com/stratif-io/tofa/commit/b0cb6decb0820186368619ff9f09d420c94b64ba))
* import dispatcher, URI export, dedup, and architectural audit pass ([9d8f2d4](https://github.com/stratif-io/tofa/commit/9d8f2d4432b59c2b43016e67792013a9c1be3cfa))
* multi-format import, UI redesign, Homebrew tap, TOFA branding ([30b7fd9](https://github.com/stratif-io/tofa/commit/30b7fd99276ead2450ee02e59304598b67fd1bfb))
* smooth rAF progress bar, 10s API refresh, multi-QR screen scan, scan feedback ([f07fb1e](https://github.com/stratif-io/tofa/commit/f07fb1e64be9f638d82228309c9f6cc246661a6b))
* **tauri:** full Tauri app UI — loader, drag-drop, scan feedback, lock/unlock UX ([7d5fde0](https://github.com/stratif-io/tofa/commit/7d5fde0449aa31770a486d04083c095c7c105c40))
* **tauri:** scan screen + camera QR, tray context menu ([69d336b](https://github.com/stratif-io/tofa/commit/69d336b00fe3946699589f4f5ddc0a2f44b6ca32))


### Bug Fixes

* **core,app:** centralise dedup in Vault::add_entry_if_unique ([7b97628](https://github.com/stratif-io/tofa/commit/7b97628395be94064ffb12aafd719561b1dbd52b))
* **core:** detect native tofa export format before andOTP parser ([4ed0473](https://github.com/stratif-io/tofa/commit/4ed0473ab7ac7fd3ff53c06cdad41669671bb0ed))
* **core:** preserve per-account algorithm and digits in migration export ([49641a8](https://github.com/stratif-io/tofa/commit/49641a8dcd0ca1885db7d9c56f471301c6b2a7ff))
* **core:** scan native resolution before downscaled rescales ([5ea3fde](https://github.com/stratif-io/tofa/commit/5ea3fde8fdec87420b0211d917a29355b987b26d))
* **core:** scan native resolution before downscaled rescales ([b658501](https://github.com/stratif-io/tofa/commit/b6585014cc6af52361ce534ec7846cce56772759))
* **fixtures:** replace invalid base32 secrets with known-good JBSWY3DPEHPK3PXP ([6c3fa00](https://github.com/stratif-io/tofa/commit/6c3fa00b2658b304feb788c0ed936d2fd208cc09))
* proper 8-digit code formatting (XXXX XXXX) in TUI and Tauri app ([acf2582](https://github.com/stratif-io/tofa/commit/acf2582104b15e7cacaea5ec0b508264b0c11f71))
* **release:** bump tofa-core to 0.4.0 and tofa to 0.5.0 ([ee895da](https://github.com/stratif-io/tofa/commit/ee895da58bc6d81bd52f4d670a002ea7bfc45f88))
* repair broken 0.4.0 publish, harden release pipeline, preserve OTP params in migration export ([654a6a9](https://github.com/stratif-io/tofa/commit/654a6a910f1bb954ebb2030b4aac29e19bb74ca0))
* resolve all clippy warnings ([5a3f218](https://github.com/stratif-io/tofa/commit/5a3f21828887f1578c70f583704530bb401c593c))
* **tests:** add missing id field in VaultEntry initializers and fix list output ([4cd0c7e](https://github.com/stratif-io/tofa/commit/4cd0c7ee5b7b61df80884c47600a8c6d11710343))


### Performance Improvements

* **core:** add Triangle filter pass at 1920px for marginal-QR recall ([9e36e44](https://github.com/stratif-io/tofa/commit/9e36e4462e41a5b126ec35a6d1f04c90e588ccac))
* **core:** cap effective native scan at 3840px ([f39c548](https://github.com/stratif-io/tofa/commit/f39c548c9bcfbb373095ad38e166ca6c479f1350))
* **core:** trim scan rescale ladder + early termination ([e5cc986](https://github.com/stratif-io/tofa/commit/e5cc9867d40e82755718969701426db43259396f))
* **core:** trim scan rescale ladder + early termination ([445f2fd](https://github.com/stratif-io/tofa/commit/445f2fd356dcd93cc7fbc8fc64adf8498e9f2c2d))

## [0.10.0](https://github.com/stratif-io/tofa/compare/tofa-core-v0.9.0...tofa-core-v0.10.0) (2026-05-11)


### Features

* expose VERSION constant for downstream consumers ([#76](https://github.com/stratif-io/tofa/pull/76))

## [0.9.0](https://github.com/stratif-io/tofa/compare/tofa-core-v0.8.0...tofa-core-v0.9.0) (2026-05-07)


### Features

* **ci:** add crates.io publish workflow and complete crate metadata ([3428908](https://github.com/stratif-io/tofa/commit/34289083e9c917eab298df202d467b1fd74afbed))
* **ci:** crates.io publish workflow and complete crate metadata ([62a26a0](https://github.com/stratif-io/tofa/commit/62a26a0c177fe812a2e702abb2caf84d0d9b9845))
* **cli,tui,app:** route every import surface through the unified dispatcher ([5591dcb](https://github.com/stratif-io/tofa/commit/5591dcbfeb7e80d0b425ef8c045c0b9afa480e1c))
* **core,cli:** expose per-pass scan progress and surface it in the spinner ([08da8af](https://github.com/stratif-io/tofa/commit/08da8af76513c7df8ce05dc95dbcdf02e81e6703))
* **core:** add build_otpauth_uri helper for single-entry exports ([ae6a79b](https://github.com/stratif-io/tofa/commit/ae6a79b8b7aa37b7b169a279cde3fd56dcf58ae4))
* **core:** add entries_to_uri_list for plain-text URI exports ([f326953](https://github.com/stratif-io/tofa/commit/f326953530f1bcc98097f473e32fadf78d8cb88b))
* **core:** add Google Authenticator migration URI import ([22d6ad7](https://github.com/stratif-io/tofa/commit/22d6ad7ca4513f760bff15e13fdcd4d79b6ab287))
* **core:** add import parsers for 2FAS, Raivo, Bitwarden, FreeOTP+, Ente Auth, KeePassXC ([d931a08](https://github.com/stratif-io/tofa/commit/d931a08d754b4a901f599afd4fb6a2c0a1fd30ca))
* **core:** add unified import::parse_file(path) dispatcher ([26c159d](https://github.com/stratif-io/tofa/commit/26c159d0129b59e48ad017e288c34daef9ec9ccc))
* **core:** build_selection_uri dispatches by selection shape ([6695d10](https://github.com/stratif-io/tofa/commit/6695d106b119d36fe3811034ce05569bc457c6f5))
* **fixtures:** generate all 12 TOTP combinations (3 algo × 2 digits × 2 periods) ([2cfba10](https://github.com/stratif-io/tofa/commit/2cfba1015d597ac2cbb5239284e70e40f2caa92d))
* full-fidelity QR export rules across CLI, TUI, and Tauri app ([b0cb6de](https://github.com/stratif-io/tofa/commit/b0cb6decb0820186368619ff9f09d420c94b64ba))
* import dispatcher, URI export, dedup, and architectural audit pass ([9d8f2d4](https://github.com/stratif-io/tofa/commit/9d8f2d4432b59c2b43016e67792013a9c1be3cfa))
* multi-format import, UI redesign, Homebrew tap, TOFA branding ([30b7fd9](https://github.com/stratif-io/tofa/commit/30b7fd99276ead2450ee02e59304598b67fd1bfb))
* smooth rAF progress bar, 10s API refresh, multi-QR screen scan, scan feedback ([f07fb1e](https://github.com/stratif-io/tofa/commit/f07fb1e64be9f638d82228309c9f6cc246661a6b))
* **tauri:** full Tauri app UI — loader, drag-drop, scan feedback, lock/unlock UX ([7d5fde0](https://github.com/stratif-io/tofa/commit/7d5fde0449aa31770a486d04083c095c7c105c40))
* **tauri:** scan screen + camera QR, tray context menu ([69d336b](https://github.com/stratif-io/tofa/commit/69d336b00fe3946699589f4f5ddc0a2f44b6ca32))


### Bug Fixes

* **core,app:** centralise dedup in Vault::add_entry_if_unique ([7b97628](https://github.com/stratif-io/tofa/commit/7b97628395be94064ffb12aafd719561b1dbd52b))
* **core:** detect native tofa export format before andOTP parser ([4ed0473](https://github.com/stratif-io/tofa/commit/4ed0473ab7ac7fd3ff53c06cdad41669671bb0ed))
* **core:** preserve per-account algorithm and digits in migration export ([49641a8](https://github.com/stratif-io/tofa/commit/49641a8dcd0ca1885db7d9c56f471301c6b2a7ff))
* **core:** scan native resolution before downscaled rescales ([5ea3fde](https://github.com/stratif-io/tofa/commit/5ea3fde8fdec87420b0211d917a29355b987b26d))
* **core:** scan native resolution before downscaled rescales ([b658501](https://github.com/stratif-io/tofa/commit/b6585014cc6af52361ce534ec7846cce56772759))
* **fixtures:** replace invalid base32 secrets with known-good JBSWY3DPEHPK3PXP ([6c3fa00](https://github.com/stratif-io/tofa/commit/6c3fa00b2658b304feb788c0ed936d2fd208cc09))
* proper 8-digit code formatting (XXXX XXXX) in TUI and Tauri app ([acf2582](https://github.com/stratif-io/tofa/commit/acf2582104b15e7cacaea5ec0b508264b0c11f71))
* **release:** bump tofa-core to 0.4.0 and tofa to 0.5.0 ([ee895da](https://github.com/stratif-io/tofa/commit/ee895da58bc6d81bd52f4d670a002ea7bfc45f88))
* repair broken 0.4.0 publish, harden release pipeline, preserve OTP params in migration export ([654a6a9](https://github.com/stratif-io/tofa/commit/654a6a910f1bb954ebb2030b4aac29e19bb74ca0))
* resolve all clippy warnings ([5a3f218](https://github.com/stratif-io/tofa/commit/5a3f21828887f1578c70f583704530bb401c593c))
* **tests:** add missing id field in VaultEntry initializers and fix list output ([4cd0c7e](https://github.com/stratif-io/tofa/commit/4cd0c7ee5b7b61df80884c47600a8c6d11710343))


### Performance Improvements

* **core:** add Triangle filter pass at 1920px for marginal-QR recall ([9e36e44](https://github.com/stratif-io/tofa/commit/9e36e4462e41a5b126ec35a6d1f04c90e588ccac))
* **core:** cap effective native scan at 3840px ([f39c548](https://github.com/stratif-io/tofa/commit/f39c548c9bcfbb373095ad38e166ca6c479f1350))
* **core:** trim scan rescale ladder + early termination ([e5cc986](https://github.com/stratif-io/tofa/commit/e5cc9867d40e82755718969701426db43259396f))
* **core:** trim scan rescale ladder + early termination ([445f2fd](https://github.com/stratif-io/tofa/commit/445f2fd356dcd93cc7fbc8fc64adf8498e9f2c2d))

## [0.8.0](https://github.com/stratif-io/tofa/compare/tofa-core-v0.6.0...tofa-core-v0.8.0) (2026-05-07)


### Features

* **cli,tui,app:** route every import surface through the unified dispatcher ([5591dcb](https://github.com/stratif-io/tofa/commit/5591dcbfeb7e80d0b425ef8c045c0b9afa480e1c))
* **core,cli:** expose per-pass scan progress and surface it in the spinner ([08da8af](https://github.com/stratif-io/tofa/commit/08da8af76513c7df8ce05dc95dbcdf02e81e6703))
* **core:** add entries_to_uri_list for plain-text URI exports ([f326953](https://github.com/stratif-io/tofa/commit/f326953530f1bcc98097f473e32fadf78d8cb88b))
* **core:** add unified import::parse_file(path) dispatcher ([26c159d](https://github.com/stratif-io/tofa/commit/26c159d0129b59e48ad017e288c34daef9ec9ccc))
* import dispatcher, URI export, dedup, and architectural audit pass ([9d8f2d4](https://github.com/stratif-io/tofa/commit/9d8f2d4432b59c2b43016e67792013a9c1be3cfa))


### Bug Fixes

* **core,app:** centralise dedup in Vault::add_entry_if_unique ([7b97628](https://github.com/stratif-io/tofa/commit/7b97628395be94064ffb12aafd719561b1dbd52b))
* **core:** scan native resolution before downscaled rescales ([5ea3fde](https://github.com/stratif-io/tofa/commit/5ea3fde8fdec87420b0211d917a29355b987b26d))
* **core:** scan native resolution before downscaled rescales ([b658501](https://github.com/stratif-io/tofa/commit/b6585014cc6af52361ce534ec7846cce56772759))


### Performance Improvements

* **core:** add Triangle filter pass at 1920px for marginal-QR recall ([9e36e44](https://github.com/stratif-io/tofa/commit/9e36e4462e41a5b126ec35a6d1f04c90e588ccac))
* **core:** cap effective native scan at 3840px ([f39c548](https://github.com/stratif-io/tofa/commit/f39c548c9bcfbb373095ad38e166ca6c479f1350))
* **core:** trim scan rescale ladder + early termination ([e5cc986](https://github.com/stratif-io/tofa/commit/e5cc9867d40e82755718969701426db43259396f))
* **core:** trim scan rescale ladder + early termination ([445f2fd](https://github.com/stratif-io/tofa/commit/445f2fd356dcd93cc7fbc8fc64adf8498e9f2c2d))

## [0.6.0](https://github.com/stratif-io/tofa/compare/tofa-core-v0.5.0...tofa-core-v0.6.0) (2026-05-06)


### Features

* **core:** add build_otpauth_uri helper for single-entry exports ([ae6a79b](https://github.com/stratif-io/tofa/commit/ae6a79b8b7aa37b7b169a279cde3fd56dcf58ae4))
* **core:** build_selection_uri dispatches by selection shape ([6695d10](https://github.com/stratif-io/tofa/commit/6695d106b119d36fe3811034ce05569bc457c6f5))
* full-fidelity QR export rules across CLI, TUI, and Tauri app ([b0cb6de](https://github.com/stratif-io/tofa/commit/b0cb6decb0820186368619ff9f09d420c94b64ba))

## [0.5.0](https://github.com/stratif-io/tofa/compare/tofa-core-v0.4.0...tofa-core-v0.5.0) (2026-05-06)


### Features

* **ci:** add crates.io publish workflow and complete crate metadata ([3428908](https://github.com/stratif-io/tofa/commit/34289083e9c917eab298df202d467b1fd74afbed))
* **ci:** crates.io publish workflow and complete crate metadata ([62a26a0](https://github.com/stratif-io/tofa/commit/62a26a0c177fe812a2e702abb2caf84d0d9b9845))
* **core:** add Google Authenticator migration URI import ([22d6ad7](https://github.com/stratif-io/tofa/commit/22d6ad7ca4513f760bff15e13fdcd4d79b6ab287))
* **core:** add import parsers for 2FAS, Raivo, Bitwarden, FreeOTP+, Ente Auth, KeePassXC ([d931a08](https://github.com/stratif-io/tofa/commit/d931a08d754b4a901f599afd4fb6a2c0a1fd30ca))
* **fixtures:** generate all 12 TOTP combinations (3 algo × 2 digits × 2 periods) ([2cfba10](https://github.com/stratif-io/tofa/commit/2cfba1015d597ac2cbb5239284e70e40f2caa92d))
* multi-format import, UI redesign, Homebrew tap, TOFA branding ([30b7fd9](https://github.com/stratif-io/tofa/commit/30b7fd99276ead2450ee02e59304598b67fd1bfb))
* smooth rAF progress bar, 10s API refresh, multi-QR screen scan, scan feedback ([f07fb1e](https://github.com/stratif-io/tofa/commit/f07fb1e64be9f638d82228309c9f6cc246661a6b))
* **tauri:** full Tauri app UI — loader, drag-drop, scan feedback, lock/unlock UX ([7d5fde0](https://github.com/stratif-io/tofa/commit/7d5fde0449aa31770a486d04083c095c7c105c40))
* **tauri:** scan screen + camera QR, tray context menu ([69d336b](https://github.com/stratif-io/tofa/commit/69d336b00fe3946699589f4f5ddc0a2f44b6ca32))


### Bug Fixes

* **core:** detect native tofa export format before andOTP parser ([4ed0473](https://github.com/stratif-io/tofa/commit/4ed0473ab7ac7fd3ff53c06cdad41669671bb0ed))
* **core:** preserve per-account algorithm and digits in migration export ([49641a8](https://github.com/stratif-io/tofa/commit/49641a8dcd0ca1885db7d9c56f471301c6b2a7ff))
* **fixtures:** replace invalid base32 secrets with known-good JBSWY3DPEHPK3PXP ([6c3fa00](https://github.com/stratif-io/tofa/commit/6c3fa00b2658b304feb788c0ed936d2fd208cc09))
* proper 8-digit code formatting (XXXX XXXX) in TUI and Tauri app ([acf2582](https://github.com/stratif-io/tofa/commit/acf2582104b15e7cacaea5ec0b508264b0c11f71))
* **release:** bump tofa-core to 0.4.0 and tofa to 0.5.0 ([ee895da](https://github.com/stratif-io/tofa/commit/ee895da58bc6d81bd52f4d670a002ea7bfc45f88))
* repair broken 0.4.0 publish, harden release pipeline, preserve OTP params in migration export ([654a6a9](https://github.com/stratif-io/tofa/commit/654a6a910f1bb954ebb2030b4aac29e19bb74ca0))
* resolve all clippy warnings ([5a3f218](https://github.com/stratif-io/tofa/commit/5a3f21828887f1578c70f583704530bb401c593c))
* **tests:** add missing id field in VaultEntry initializers and fix list output ([4cd0c7e](https://github.com/stratif-io/tofa/commit/4cd0c7ee5b7b61df80884c47600a8c6d11710343))
