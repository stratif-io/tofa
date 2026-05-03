# Design System Application — TOFA CLI · TUI · App

**Date:** 2026-05-03  
**Status:** Approved  
**Scope:** Apply the TOFA design system across all three surfaces simultaneously via a shared `tofa-theme` crate.

---

## 1. Overview

The TOFA design system (defined in `docs/design/tofa-design-system.html`) is currently HTML/CSS only. This spec describes how it is applied to:

- **CLI** (`tofa` crate) — ANSI output, brand voice, table formatting
- **TUI** (`tofa` crate, ratatui) — themed widgets, Sir Wink Unicode art, dark/light auto-detect
- **App** (`tofa-app` crate, Tauri webview) — full HTML/CSS rewrite from design system tokens

The approach is **Approach C: shared `tofa-theme` crate** — a new workspace crate that exports color constants, theme detection, Sir Wink art, and reusable ratatui widgets. The Tauri app consumes the CSS assets directly (no Rust sharing needed).

---

## 2. New Crate: `tofa-theme`

### Workspace addition

```toml
# Cargo.toml (workspace root)
members = ["tofa-core", "tofa-theme", "tofa", "tofa-app"]
```

`tofa` depends on `tofa-theme`. `tofa-app` does not (CSS only).

### Module structure

```
tofa-theme/src/
  lib.rs          — re-exports palette, theme, wink, widgets
  palette.rs      — Color constants (ratatui::style::Color)
  theme.rs        — ThemeMode enum + detect()
  wink.rs         — Sir Wink Unicode art constants
  widgets/
    mod.rs
    otp_display.rs
    account_list.rs
    badge.rs
    toast.rs
    unlock_prompt.rs
    search_bar.rs
```

---

## 3. Color Tokens (`palette.rs`)

All values map 1:1 from the CSS design system tokens.

```rust
use ratatui::style::Color;

// Semantic — dark mode (default)
pub const BRAND:      Color = Color::Rgb(184, 158, 255); // --purple-300
pub const SUCCESS:    Color = Color::Rgb( 74, 222, 128); // --green
pub const WARNING:    Color = Color::Rgb(251, 191,  36); // --amber
pub const DANGER:     Color = Color::Rgb(248, 113, 113); // --red

pub const BG:         Color = Color::Rgb( 10,  10,  18); // --ink-900
pub const SURFACE:    Color = Color::Rgb( 20,  19,  31); // --ink-800
pub const BORDER:     Color = Color::Rgb( 42,  42,  58); // --border
pub const TEXT:       Color = Color::Rgb(232, 230, 240); // --ink-100
pub const TEXT_MUTED: Color = Color::Rgb(122, 118, 144); // --ink-400

// Light mode overrides — accessed via ThemeMode::resolve()
pub const BRAND_LIGHT: Color = Color::Rgb(117,  89, 184); // --purple-600
pub const BG_LIGHT:    Color = Color::Rgb(244, 243, 248); // --ink-50
pub const TEXT_LIGHT:  Color = Color::Rgb( 10,  10,  18); // --ink-900
```

---

## 4. Theme Detection (`theme.rs`)

```rust
pub enum ThemeMode { Dark, Light }

impl ThemeMode {
    pub fn detect() -> Self {
        // Priority order:
        // 1. $TOFA_THEME=dark|light  (explicit user override)
        // 2. $TERM_BACKGROUND=dark|light  (iTerm2, kitty)
        // 3. $COLORSCHEME=dark|light  (WezTerm, foot)
        // 4. OSC 11 background query (ANSI escape fallback)
        // 5. Dark by default
    }

    pub fn brand(&self) -> Color { ... }
    pub fn bg(&self) -> Color { ... }
    pub fn text(&self) -> Color { ... }
    // etc. for all semantic tokens
}
```

`ThemeMode::detect()` is called once at startup and passed into all widgets. No global state.

---

## 5. Sir Wink Unicode Art (`wink.rs`)

Two sizes, both as `&'static str` constants. Final glyphs are refined at implementation time.

```rust
/// Large — splash screen, unlock screen. ~10×8 terminal chars.
pub const WINK_LARGE: &str = "
⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⠛⠛⣿⠻⠿⠿⣿⣿
⣿⣿⠃⣿⡟⣠⣤⠈⣿⣿
⣿⣿⠛⠛⣿⠛⠛⠛⣿⣿
⣿⡇⠀⣿⣿⡆⠀⠀⢸⣿
⣿⣿⣄⣀⣀⣀⣀⣄⣿⣿
⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛";

/// Small — TUI header, inline with title. ~5×4 terminal chars.
pub const WINK_SMALL: &str = "▄▄▄▄▄\n█▀▄▀█\n▀▀▀▀▀";
```

**Usage in TUI:**
- `WINK_LARGE` — centered on `UnlockPrompt` screen, colored `theme.brand()`, subtle pulse animation via color cycling
- `WINK_SMALL` — inline in the `Block::default().title()` of the main TUI header

---

## 6. Ratatui Widgets (`widgets/`)

All widgets implement `ratatui::Widget`. All accept a `ThemeMode` via builder pattern.

### Common API pattern

```rust
OtpDisplay::new(&account, &code)
    .theme(theme_mode)
    .focused(true)
    .render(area, buf);
```

### Widget catalogue

#### `OtpDisplay`
Displays a single TOTP account with code, countdown, and progress bar.

Layout:
```
┌─────────────────────────────────────┐
│ [icon]  Issuer · account            │
│         847 392                     │
│                         19s remaining│
│ ████████████░░░░░░░░░░░░░░░░░░░░░░  │
└─────────────────────────────────────┘
```

Countdown color logic:
- `>= 10s` → `BRAND`
- `< 10s`  → `WARNING` (amber)
- `< 5s`   → `DANGER` (red)

Both the seconds label and the progress bar change color together.

#### `AccountList`
Scrollable list of accounts. Selected item has brand left-border accent and brand-bg background. Unselected items show code as `● ● ● ● ● ●` (masked). Selected item reveals the code.

#### `Badge`
Inline status pill. Variants: `Success`, `Warning`, `Danger`, `Brand`. Dot + text.

#### `Toast`
Temporary overlay anchored to the bottom of the screen. Auto-dismisses after 2 seconds. Variants: `Success`, `Error`. Managed via `ToastState` pushed from any screen with `Toast::success("msg").push(&mut state.toasts)`.

#### `UnlockPrompt`
Full-screen centered layout: `WINK_LARGE` in brand color, "VAULT LOCKED" label in `TEXT_MUTED`, masked password input with brand border-focus.

#### `SearchBar`
Single-line input with `/` prefix. Filters `AccountList` in real time via fuzzy match. Matched characters highlighted in `BRAND`. Activated with `/` or `⌘K`.

---

## 7. CLI Output Styling

### Colors (ANSI via `ratatui` or direct `crossterm`)

| Element              | Color         |
|----------------------|---------------|
| OTP code             | `BRAND`       |
| Account name (highlight) | `BRAND`   |
| Success message      | `SUCCESS`     |
| Warning              | `WARNING`     |
| Error                | `DANGER`      |
| Secondary output     | `TEXT_MUTED`  |
| Box-drawing chars    | `--term-box` (`#7559b8`) |

### Brand voice constants

```rust
// Used with format!() — e.g. format!(voice::COPIED, account = name)
pub mod voice {
    pub const ADDED_OK:  &str = "Sir Wink's got it. 😉";
    pub const COPIED:    &str = "Copied · {account}";
    pub const NOT_FOUND: &str = "no account named \"{name}\" — did you mean {suggestion}?";
    pub const NO_MATCH:  &str = "no accounts match \"{query}\"";
}
```

### Table format (`tofa list`)

```
┌──────────────┬─────────────────┬──────────┬─────────┐
│ issuer       │ account         │ code     │ expires │
├──────────────┼─────────────────┼──────────┼─────────┤
│ github       │ user@ex.com     │ 847 392  │ 19s     │
│ aws          │ root            │ ● ● ● ●  │ 24s     │
└──────────────┴─────────────────┴──────────┴─────────┘
```

The `expires` column applies the same brand→amber→red color logic as `OtpDisplay`.

---

## 8. Tauri App Rewrite

### File structure

```
tofa-app/src/
  assets/
    css/
      tokens.css    ← copied from docs/design/assets/css/tokens.css
      styles.css    ← copied from docs/design/assets/css/styles.css
      app.css       ← Tauri-specific: scrollbar, window chrome, drag regions
    svg/
      sprite.svg    ← copied from docs/design/assets/svg/sprite.svg
  js/
    app.js          ← Tauri invoke/listen, view routing
    otp.js          ← countdown timer, clipboard, toast
  index.html        ← single entry point, all views as hidden sections
```

### Four views

| View | Trigger | Key elements |
|------|---------|--------------|
| **Locked** | Vault encrypted at startup | `WINK_LARGE` (inline SVG), password input, unlock button |
| **List** | After unlock | Header with Sir Wink + badge, search bar, progress bar, account list, countdown per item |
| **Detail** | Click account | Full `OtpDisplay` layout, "Xs remaining" + progress, Copy / QR / Delete actions |
| **Add** | ⌘N or + button | Drag-drop QR zone, screen-scan button, manual secret input |

### Theme

Same `data-theme="light"` toggle on `<html>` as the design system HTML. The toggle reads system preference via `prefers-color-scheme` on first load, then persists to `localStorage`.

### Countdown JS

```js
// Same brand→amber→red logic as TUI
function updateCountdown(el, seconds) {
  const color = seconds >= 10 ? 'var(--brand)'
              : seconds >= 5  ? 'var(--warning)'
              :                  'var(--danger)';
  el.style.color = color;
  el.style.setProperty('--progress', `${(seconds / 30) * 100}%`);
}
```

---

## 9. Error Handling

- `ThemeMode::detect()` never panics — always falls back to `Dark`
- Widget rendering is infallible (ratatui contract)
- `Toast` queue is bounded to 3 items; oldest is dropped if exceeded
- CSS assets (`tokens.css`, `styles.css`, `sprite.svg`) are copied manually from `docs/design/assets/` into `tofa-app/src/assets/` and committed. They are not auto-synced — changes to the design system must be propagated manually. A `build.rs` sync script can be added later if drift becomes a problem.

---

## 10. Out of Scope

- i18n / localization
- Accessibility audit (separate task)
- Animations beyond color cycling for Sir Wink pulse
- New CLI commands (this spec is styling only)
