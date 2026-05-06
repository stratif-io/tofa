# Changelog

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
