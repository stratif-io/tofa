# rotp Visual Redesign Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the monochrome green TUI with a Clean & Minimal palette — near-black background, grey/white text hierarchy, single violet accent (`#A78BFA`) for OTP codes, dual timer signal (colour + progress bar).

**Architecture:** All changes are purely visual — no logic, no new dependencies, no state changes. `theme.rs` is the single source of truth for all colours; every screen imports from it. The list screen is the most structurally changed: each entry becomes a multi-line `ListItem` (issuer row + account row + bar row) rendered inside the existing `List` widget.

**Tech Stack:** Rust stable, ratatui 0.29, crossterm, tui-big-text 0.6. No new crates.

---

## File map

| File | Change |
|---|---|
| `rotp/src/tui/theme.rs` | Full rewrite — new palette + updated `timer_color` thresholds |
| `rotp/src/tui/screens/list.rs` | Major rewrite — two-line items, bar row, selected treatment |
| `rotp/src/tui/screens/fullscreen.rs` | Colour updates + border colour |
| `rotp/src/tui/screens/unlock.rs` | Colour updates only |
| `rotp/src/tui/screens/otp_detail.rs` | Colour updates only |
| `rotp/src/tui/screens/export.rs` | Colour updates only |
| `rotp/src/tui/screens/export_qr.rs` | Colour updates only (QR area untouched) |
| `rotp/src/tui/screens/scanning_qr.rs` | Colour updates only |
| `rotp/src/tui/screens/file_picker.rs` | Colour updates + selected treatment |
| `rotp/src/tui/screens/add_form.rs` | Colour updates only |
| `rotp/src/tui/screens/add_name.rs` | Colour updates only |
| `rotp/src/tui/screens/delete_confirm.rs` | Colour updates only |

---

## Task 1: Replace theme.rs

**Files:**
- Modify: `rotp/src/tui/theme.rs`

- [ ] **Step 1: Replace the entire file**

```rust
use ratatui::style::Color;

pub const BG:       Color = Color::Rgb(17,  17,  17);   // #111111
pub const SURFACE:  Color = Color::Rgb(28,  28,  28);   // #1C1C1C
pub const SELECTED: Color = Color::Rgb(30,  26,  46);   // #1E1A2E
pub const TEXT:     Color = Color::Rgb(224, 224, 224);  // #E0E0E0
pub const DIM:      Color = Color::Rgb(85,  85,  85);   // #555555
pub const MUTED:    Color = Color::Rgb(42,  42,  42);   // #2A2A2A
pub const ACCENT:   Color = Color::Rgb(167, 139, 250);  // #A78BFA
pub const WARNING:  Color = Color::Rgb(251, 146, 60);   // #FB923C
pub const URGENT:   Color = Color::Rgb(248, 113, 113);  // #F87171
pub const BORDER:   Color = Color::Rgb(42,  42,  42);   // #2A2A2A

/// Returns the timer colour based on seconds remaining in the 30s TOTP window.
/// > 10s → ACCENT, 5–10s → WARNING, < 5s → URGENT
pub fn timer_color(seconds: u64) -> Color {
    match seconds {
        s if s > 10 => ACCENT,
        s if s >= 5 => WARNING,
        _           => URGENT,
    }
}
```

- [ ] **Step 2: Build to check all `theme::` references compile**

```bash
cargo build -p rotp 2>&1 | grep "^error"
```

Expected: errors about `GREEN`, `DIM_GREEN`, `RED`, `WHITE`, `ORANGE`, `SELECTED_BG` — these are removed constants, still referenced by screens. That's expected — we'll fix them screen by screen.

- [ ] **Step 3: Commit**

```bash
git add rotp/src/tui/theme.rs
git commit -m "style: replace green palette with clean minimal + violet accent"
```

---

## Task 2: Rewrite list.rs

**Files:**
- Modify: `rotp/src/tui/screens/list.rs`

The list now renders each vault entry as a **3-line `ListItem`**:
- Line 0: `▌ {Issuer}` (selected) or `  {Issuer}` (dim) + code right-aligned
- Line 1: `  {account}` in Muted
- Line 2: progress bar as `█░` characters, timer-coloured

Right-aligning the code: compute padding = `item_width - left_prefix(2) - issuer_len - code_len`.

The `List` widget's `highlight_style` sets the background for all lines of the selected item.

- [ ] **Step 1: Write the new list.rs**

```rust
use crate::tui::{state::AppState, theme};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, ListState, Paragraph},
    Frame,
};
use rotp_core::{
    store::Vault,
    totp::{generate_code_now, seconds_remaining_now},
};

pub fn render(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let secs = seconds_remaining_now();
    let timer_col = theme::timer_color(secs);
    let entries = vault.entries();
    let item_w = chunks[0].width as usize;

    let items: Vec<ListItem> = entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let selected = i == state.selected_index;

            // ── split name into issuer / account ─────────────────────────
            let (issuer, account) = if let Some(pos) = entry.name.find(':') {
                (&entry.name[..pos], &entry.name[pos + 1..])
            } else {
                (entry.name.as_str(), "")
            };

            // ── code string ───────────────────────────────────────────────
            let show = state.show_codes || selected;
            let code_str = if show {
                let code = generate_code_now(&entry.secret)
                    .unwrap_or_else(|_| "000000".to_string());
                format!("{} {}", &code[..3], &code[3..])
            } else {
                "••• •••".to_string()
            };
            let code_col = if selected {
                timer_col
            } else if show {
                theme::MUTED
            } else {
                theme::MUTED
            };

            // ── line 0: border-char + issuer + padding + code ─────────────
            let border_char = if selected { "▌ " } else { "  " };
            let border_col  = if selected { theme::ACCENT } else { theme::BG };

            // right-align code: pad = width - border(2) - issuer - code(7) - gap(2)
            let pad_len = item_w
                .saturating_sub(2)                 // border char
                .saturating_sub(issuer.len())
                .saturating_sub(code_str.len())
                .saturating_sub(2);                // gap before code
            let padding = " ".repeat(pad_len);

            let issuer_col  = if selected { theme::TEXT } else { theme::DIM };
            let issuer_mod  = if selected { Modifier::BOLD } else { Modifier::empty() };

            let line0 = Line::from(vec![
                Span::styled(border_char, Style::default().fg(border_col)),
                Span::styled(issuer, Style::default().fg(issuer_col).add_modifier(issuer_mod)),
                Span::raw(padding),
                Span::raw("  "),
                Span::styled(
                    code_str,
                    Style::default().fg(code_col).add_modifier(
                        if selected { Modifier::BOLD } else { Modifier::empty() }
                    ),
                ),
            ]);

            // ── line 1: account ───────────────────────────────────────────
            let line1 = if account.is_empty() {
                Line::from(Span::raw(""))
            } else {
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled(account, Style::default().fg(theme::MUTED)),
                ])
            };

            // ── line 2: progress bar ──────────────────────────────────────
            let bar_w = item_w.saturating_sub(2);
            let filled = ((secs as usize * bar_w) / 30).min(bar_w);
            let bar = format!(
                "  {}{}",
                "█".repeat(filled),
                "░".repeat(bar_w.saturating_sub(filled))
            );
            let bar_col = if selected { timer_col } else { theme::MUTED };
            let line2 = Line::from(Span::styled(bar, Style::default().fg(bar_col)));

            ListItem::new(Text::from(vec![line0, line1, line2]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().style(Style::default().bg(theme::BG)))
        .highlight_style(Style::default().bg(theme::SELECTED));

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_index));
    f.render_stateful_widget(list, chunks[0], &mut list_state);

    // ── help / status bar ─────────────────────────────────────────────────
    let bar = if let Some(msg) = &state.status_message {
        Span::styled(msg.as_str(), Style::default().fg(theme::URGENT))
    } else {
        let toggle = if state.show_codes { "hide" } else { "show" };
        Span::styled(
            format!(
                "↑↓ navigate · Enter fullscreen · h {toggle} · a add · i detail · e export · d delete · y copy · q quit"
            ),
            Style::default().fg(theme::MUTED),
        )
    };
    f.render_widget(
        Paragraph::new(Line::from(bar)).style(Style::default().bg(theme::BG)),
        chunks[1],
    );
}
```

- [ ] **Step 2: Build**

```bash
cargo build -p rotp 2>&1 | grep "^error"
```

Expected: no errors from list.rs. Other screens may still have errors (removed theme constants).

- [ ] **Step 3: Commit**

```bash
git add rotp/src/tui/screens/list.rs
git commit -m "style: rewrite list — two-line items, progress bar, violet selection"
```

---

## Task 3: Update unlock.rs

**Files:**
- Modify: `rotp/src/tui/screens/unlock.rs`

Colour-only changes: border → `theme::BORDER`, title → `theme::TEXT`, subtitle/label/help → `theme::DIM`, input dots → `theme::DIM` with `▌` cursor in `theme::ACCENT`, error → `theme::URGENT`.

- [ ] **Step 1: Write the updated unlock.rs**

```rust
use crate::tui::{state::AppState, theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    let box_height = 10u16;
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(box_height),
            Constraint::Fill(1),
        ])
        .split(area);

    let box_width = area.width.min(52);
    let pad = (area.width.saturating_sub(box_width)) / 2;
    let outer = Rect {
        x: area.x + pad,
        y: vert[1].y,
        width: box_width,
        height: box_height,
    };

    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::BORDER))
            .style(Style::default().bg(theme::BG)),
        outer,
    );

    let inner = Rect {
        x: outer.x + 1,
        y: outer.y + 1,
        width: outer.width.saturating_sub(2),
        height: outer.height.saturating_sub(2),
    };

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // title
            Constraint::Length(1), // subtitle
            Constraint::Length(1), // gap
            Constraint::Length(1), // label
            Constraint::Length(1), // input
            Constraint::Length(1), // error
            Constraint::Length(1), // gap
            Constraint::Length(1), // help
        ])
        .split(inner);

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "r o t p",
            Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center),
        rows[0],
    );

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "OTP Manager",
            Style::default().fg(theme::DIM),
        )))
        .alignment(Alignment::Center),
        rows[1],
    );

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "Passphrase",
            Style::default().fg(theme::DIM),
        )))
        .alignment(Alignment::Left),
        rows[3],
    );

    let masked = format!(
        "{}{}",
        "•".repeat(state.passphrase_input.len()),
        Span::styled("▌", Style::default().fg(theme::ACCENT)).content
    );
    // Render dots in DIM, cursor in ACCENT
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                "•".repeat(state.passphrase_input.len()),
                Style::default().fg(theme::DIM),
            ),
            Span::styled("▌", Style::default().fg(theme::ACCENT)),
        ]))
        .alignment(Alignment::Left),
        rows[4],
    );
    drop(masked); // suppress unused warning

    if state.unlock_error {
        f.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "Wrong passphrase. Try again.",
                Style::default().fg(theme::URGENT),
            )))
            .alignment(Alignment::Center),
            rows[5],
        );
    }

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "[ Enter ] unlock   [ Ctrl+C ] quit",
            Style::default().fg(theme::MUTED),
        )))
        .alignment(Alignment::Center),
        rows[7],
    );
}
```

- [ ] **Step 2: Build**

```bash
cargo build -p rotp 2>&1 | grep "^error"
```

- [ ] **Step 3: Commit**

```bash
git add rotp/src/tui/screens/unlock.rs
git commit -m "style: unlock screen — minimal palette"
```

---

## Task 4: Update fullscreen.rs

**Files:**
- Modify: `rotp/src/tui/screens/fullscreen.rs`

Changes: border → `theme::BORDER`, name label → `theme::DIM`, big code colour → `timer_color`, gauge style → timer colour on `theme::BG`, timer text → timer colour, help → `theme::DIM`. Remove the now-unused `theme::GREEN` references.

- [ ] **Step 1: Write the updated fullscreen.rs**

```rust
use crate::tui::{state::AppState, theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Gauge, Paragraph},
    Frame,
};
use rotp_core::{
    store::Vault,
    totp::{generate_code_now, seconds_remaining_now},
};
use tui_big_text::{BigText, PixelSize};

pub fn render(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault) {
    let entry = match vault.entries().get(state.selected_index) {
        Some(e) => e,
        None => return,
    };

    let code = generate_code_now(&entry.secret).unwrap_or_else(|_| "------".to_string());
    let secs = seconds_remaining_now();
    let timer_col = theme::timer_color(secs);

    let use_half_height = area.width >= 68;
    let (modal_w, modal_h) = if use_half_height {
        (area.width.min(70), 13u16)
    } else {
        (area.width.min(38).max(28), 13u16)
    };
    let modal_x = area.x + (area.width.saturating_sub(modal_w)) / 2;
    let modal_y = area.y + (area.height.saturating_sub(modal_h)) / 2;
    let modal = Rect { x: modal_x, y: modal_y, width: modal_w, height: modal_h };

    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);
    f.render_widget(Clear, modal);
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::BORDER))
            .style(Style::default().bg(theme::BG)),
        modal,
    );

    let inner = Rect {
        x: modal.x + 1,
        y: modal.y + 1,
        width: modal.width.saturating_sub(2),
        height: modal.height.saturating_sub(2),
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // name
            Constraint::Length(1), // gap
            Constraint::Length(4), // big text
            Constraint::Length(1), // gap
            Constraint::Length(1), // gauge
            Constraint::Length(1), // timer text
            Constraint::Fill(1),   // spacer
            Constraint::Length(1), // help
        ])
        .split(inner);

    // Split entry.name into "Issuer · account"
    let header = if let Some(pos) = entry.name.find(':') {
        format!("{} · {}", &entry.name[..pos], &entry.name[pos + 1..])
    } else {
        entry.name.clone()
    };

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            header,
            Style::default().fg(theme::DIM),
        )))
        .alignment(Alignment::Center),
        chunks[0],
    );

    let spaced = format!("{} {}", &code[..3], &code[3..]);
    let pixel_size = if use_half_height { PixelSize::HalfHeight } else { PixelSize::Quadrant };
    let big_text = BigText::builder()
        .pixel_size(pixel_size)
        .style(Style::default().fg(timer_col).add_modifier(Modifier::BOLD))
        .lines(vec![Line::from(spaced.as_str())])
        .centered()
        .build();
    f.render_widget(big_text, chunks[2]);

    let ratio = secs as f64 / 30.0;
    let gauge_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(chunks[4]);
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::NONE))
        .gauge_style(Style::default().fg(timer_col).bg(theme::BG))
        .label("")
        .ratio(ratio);
    f.render_widget(gauge, gauge_area[1]);

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            format!("expires in {}s", secs),
            Style::default().fg(timer_col),
        )))
        .alignment(Alignment::Center),
        chunks[5],
    );

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "[ y ] copy   [ Esc ] back",
            Style::default().fg(theme::DIM),
        )))
        .alignment(Alignment::Center),
        chunks[7],
    );
}
```

- [ ] **Step 2: Build**

```bash
cargo build -p rotp 2>&1 | grep "^error"
```

- [ ] **Step 3: Commit**

```bash
git add rotp/src/tui/screens/fullscreen.rs
git commit -m "style: fullscreen modal — minimal palette"
```

---

## Task 5: Update otp_detail.rs

**Files:**
- Modify: `rotp/src/tui/screens/otp_detail.rs`

Changes: border → `theme::BORDER`, title → `theme::TEXT` bold, label column → `theme::DIM`, value column → `theme::TEXT`, code value → timer colour bold, help → `theme::DIM`.

- [ ] **Step 1: Update colours in otp_detail.rs**

Replace all colour references:
- `theme::GREEN` (border) → `theme::BORDER`
- `theme::GREEN` (title) → `theme::TEXT` + `Modifier::BOLD`
- `theme::DIM` (labels) → `theme::DIM` ✓ unchanged
- `theme::GREEN` (values) → `theme::TEXT`
- Code value: `timer_color` ✓ unchanged (already uses it)
- `theme::DIM` (help) → `theme::DIM` ✓ unchanged

Full file:

```rust
use crate::tui::{state::AppState, theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rotp_core::{
    store::Vault,
    totp::{generate_code_now, seconds_remaining_now},
};

pub fn render(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault) {
    let entry = match vault.entries().get(state.selected_index) {
        Some(e) => e,
        None => return,
    };

    let code = generate_code_now(&entry.secret).unwrap_or_else(|_| "------".to_string());
    let secs = seconds_remaining_now();
    let timer_col = theme::timer_color(secs);

    let rows: Vec<(&str, String, bool)> = vec![
        ("Name",    entry.name.clone(),    false),
        ("Secret",  entry.secret.clone(),  false),
        ("Code",    format!("{} {}  {}s", &code[..3], &code[3..], secs), true),
        ("Created", entry.created_at.clone(), false),
    ];

    let box_h = (rows.len() as u16 + 6).min(area.height.saturating_sub(4));
    let box_w = area.width.min(62);
    let pad_x = (area.width.saturating_sub(box_w)) / 2;
    let pad_y = (area.height.saturating_sub(box_h)) / 2;
    let modal = Rect { x: area.x + pad_x, y: area.y + pad_y, width: box_w, height: box_h };

    f.render_widget(Clear, modal);
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::BORDER))
            .style(Style::default().bg(theme::BG)),
        modal,
    );

    let inner = Rect {
        x: modal.x + 1,
        y: modal.y + 1,
        width: modal.width.saturating_sub(2),
        height: modal.height.saturating_sub(2),
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(inner);

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "OTP Details",
            Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD),
        ))),
        chunks[0],
    );

    let mut content: Vec<Line> = vec![Line::from("")];
    for (label, value, is_code) in &rows {
        content.push(Line::from(vec![
            Span::styled(format!("{label:<10}"), Style::default().fg(theme::DIM)),
            Span::styled(
                value.clone(),
                Style::default()
                    .fg(if *is_code { timer_col } else { theme::TEXT })
                    .add_modifier(if *is_code { Modifier::BOLD } else { Modifier::empty() }),
            ),
        ]));
    }
    f.render_widget(Paragraph::new(content), chunks[2]);

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "[ y ] copy code   [ Esc ] back",
            Style::default().fg(theme::DIM),
        )))
        .alignment(Alignment::Center),
        chunks[3],
    );
}
```

- [ ] **Step 2: Build**

```bash
cargo build -p rotp 2>&1 | grep "^error"
```

- [ ] **Step 3: Commit**

```bash
git add rotp/src/tui/screens/otp_detail.rs
git commit -m "style: otp detail modal — minimal palette"
```

---

## Task 6: Update export.rs

**Files:**
- Modify: `rotp/src/tui/screens/export.rs`

Changes: border → `theme::ACCENT`, title → `theme::TEXT` bold, checked checkbox → `theme::ACCENT`, unchecked → `theme::DIM`, selected row bg → `theme::SELECTED` with `theme::ACCENT` text, help → `theme::DIM`.

- [ ] **Step 1: Update colours in export.rs**

Full file:

```rust
use crate::tui::{state::AppState, theme};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph},
    Frame,
};
use rotp_core::store::Vault;

pub fn render(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault) {
    let box_h = area.height.saturating_sub(4).max(8);
    let box_w = area.width.min(60);
    let pad_x = (area.width.saturating_sub(box_w)) / 2;
    let pad_y = (area.height.saturating_sub(box_h)) / 2;
    let modal = Rect { x: area.x + pad_x, y: area.y + pad_y, width: box_w, height: box_h };

    f.render_widget(Clear, modal);
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::ACCENT))
            .style(Style::default().bg(theme::BG)),
        modal,
    );

    let inner = Rect {
        x: modal.x + 1,
        y: modal.y + 1,
        width: modal.width.saturating_sub(2),
        height: modal.height.saturating_sub(2),
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(inner);

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "Export OTPs",
            Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD),
        ))),
        chunks[0],
    );

    let entries = vault.entries();
    let items: Vec<ListItem> = entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let checked  = state.export_checked.get(i).copied().unwrap_or(true);
            let selected = i == state.export_selected;
            let checkbox = if checked { "[✓]" } else { "[ ]" };
            let style = if selected {
                Style::default().fg(theme::ACCENT).bg(theme::SELECTED).add_modifier(Modifier::BOLD)
            } else if checked {
                Style::default().fg(theme::TEXT)
            } else {
                Style::default().fg(theme::DIM)
            };
            let cb_style = if checked {
                style.fg(theme::ACCENT)
            } else {
                style
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{checkbox} "), cb_style),
                Span::styled(entry.name.clone(), style),
            ]))
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(state.export_selected));
    f.render_stateful_widget(
        List::new(items).block(Block::default().style(Style::default().bg(theme::BG))),
        chunks[2],
        &mut list_state,
    );

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "[ Space ] toggle  [ Enter ] generate QR  [ Esc ] back",
            Style::default().fg(theme::DIM),
        ))),
        chunks[3],
    );
}
```

- [ ] **Step 2: Build**

```bash
cargo build -p rotp 2>&1 | grep "^error"
```

- [ ] **Step 3: Commit**

```bash
git add rotp/src/tui/screens/export.rs
git commit -m "style: export modal — minimal palette"
```

---

## Task 7: Update export_qr.rs, scanning_qr.rs

**Files:**
- Modify: `rotp/src/tui/screens/export_qr.rs`
- Modify: `rotp/src/tui/screens/scanning_qr.rs`

**export_qr.rs** — border → `theme::ACCENT`, title → `theme::TEXT` bold, "too small" border → `theme::URGENT`, help → `theme::DIM`. QR area stays `fg(Black) bg(White)`.

**scanning_qr.rs** — border → `theme::ACCENT`, message → `theme::DIM`.

- [ ] **Step 1: Update export_qr.rs colours**

In `export_qr.rs`, change:
- Line `border_style(Style::default().fg(theme::GREEN))` → `fg(theme::ACCENT)`
- Line title `fg(theme::GREEN).add_modifier(Modifier::BOLD)` → `fg(theme::TEXT).add_modifier(Modifier::BOLD)`
- Help `fg(theme::DIM)` → unchanged ✓
- `render_too_small`: border `fg(theme::RED)` → `fg(theme::URGENT)`, "Terminal too small" `fg(theme::RED)` → `fg(theme::URGENT)`, detail text `fg(theme::DIM)` → unchanged ✓, help `fg(theme::DIM)` → unchanged ✓

- [ ] **Step 2: Update scanning_qr.rs colours**

In `scanning_qr.rs`, change:
- `border_style(Style::default().fg(theme::GREEN))` → `fg(theme::ACCENT)`
- Spinner `fg(theme::GREEN)` → `fg(theme::ACCENT)`
- Message `fg(theme::DIM)` → unchanged ✓

- [ ] **Step 3: Build**

```bash
cargo build -p rotp 2>&1 | grep "^error"
```

- [ ] **Step 4: Commit**

```bash
git add rotp/src/tui/screens/export_qr.rs rotp/src/tui/screens/scanning_qr.rs
git commit -m "style: export QR and scanning overlay — minimal palette"
```

---

## Task 8: Update file_picker.rs

**Files:**
- Modify: `rotp/src/tui/screens/file_picker.rs`

Changes: border → `theme::BORDER`, path text → `theme::DIM` ✓, search bar `/ query▌` → query in `theme::TEXT`, cursor `▌` in `theme::ACCENT`, directories → `theme::DIM` ✓, files → `theme::TEXT`, selected row → `theme::SELECTED` bg + `theme::ACCENT` left-border char `▌`.

- [ ] **Step 1: Update file_picker.rs**

Full file:

```rust
use crate::tui::{state::AppState, theme};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub fn filtered<'a>(entries: &'a [(String, bool)], query: &str) -> Vec<&'a (String, bool)> {
    let q = query.to_lowercase();
    entries
        .iter()
        .filter(|(name, _)| q.is_empty() || name.to_lowercase().contains(&q))
        .collect()
}

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    let box_h = area.height.saturating_sub(4).max(8);
    let box_w = area.width.min(72);
    let pad_x = (area.width.saturating_sub(box_w)) / 2;
    let pad_y = (area.height.saturating_sub(box_h)) / 2;
    let modal = Rect { x: area.x + pad_x, y: area.y + pad_y, width: box_w, height: box_h };

    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::BORDER))
            .style(Style::default().bg(theme::BG)),
        modal,
    );

    let inner = Rect {
        x: modal.x + 1,
        y: modal.y + 1,
        width: modal.width.saturating_sub(2),
        height: modal.height.saturating_sub(2),
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // path
            Constraint::Length(1), // search
            Constraint::Length(1), // gap
            Constraint::Min(1),    // list
            Constraint::Length(1), // help
        ])
        .split(inner);

    // Current path
    let path_str = state.fp_path.to_string_lossy();
    let max_w = chunks[0].width as usize;
    let truncated = if path_str.len() > max_w {
        format!("…{}", &path_str[path_str.len() - max_w + 1..])
    } else {
        path_str.to_string()
    };
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(truncated, Style::default().fg(theme::DIM)))),
        chunks[0],
    );

    // Search bar: "/ {query}▌"
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("/ ", Style::default().fg(theme::DIM)),
            Span::styled(state.fp_query.clone(), Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD)),
            Span::styled("▌", Style::default().fg(theme::ACCENT)),
        ])),
        chunks[1],
    );

    // File list
    let visible: Vec<&(String, bool)> = filtered(&state.fp_entries, &state.fp_query);
    let items: Vec<ListItem> = visible
        .iter()
        .enumerate()
        .map(|(i, (name, is_dir))| {
            let selected = i == state.fp_selected;
            let (border, icon, text_col) = if selected {
                ("▌ ", if *is_dir { "▸ " } else { "  " }, if *is_dir { theme::DIM } else { theme::TEXT })
            } else {
                ("  ", if *is_dir { "▸ " } else { "  " }, if *is_dir { theme::DIM } else { theme::TEXT })
            };
            let border_col = if selected { theme::ACCENT } else { theme::BG };
            ListItem::new(Line::from(vec![
                Span::styled(border, Style::default().fg(border_col)),
                Span::styled(icon, Style::default().fg(text_col)),
                Span::styled(name.clone(), Style::default().fg(text_col)),
            ]))
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(state.fp_selected));
    f.render_stateful_widget(
        List::new(items)
            .block(Block::default().style(Style::default().bg(theme::BG)))
            .highlight_style(Style::default().bg(theme::SELECTED)),
        chunks[3],
        &mut list_state,
    );

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "[ type ] filter  [ ↑↓ ] navigate  [ Enter ] open  [ Esc ] back",
            Style::default().fg(theme::DIM),
        ))),
        chunks[4],
    );
}
```

- [ ] **Step 2: Build**

```bash
cargo build -p rotp 2>&1 | grep "^error"
```

- [ ] **Step 3: Commit**

```bash
git add rotp/src/tui/screens/file_picker.rs
git commit -m "style: file picker — minimal palette + violet selection"
```

---

## Task 9: Update add_form.rs, add_name.rs, delete_confirm.rs

**Files:**
- Modify: `rotp/src/tui/screens/add_form.rs`
- Modify: `rotp/src/tui/screens/add_name.rs`
- Modify: `rotp/src/tui/screens/delete_confirm.rs`

These are colour-only changes in all three files.

**add_form.rs:** border `theme::DIM_GREEN` → `theme::BORDER`, title `theme::GREEN` → `theme::TEXT`, input `theme::GREEN` → `theme::TEXT`, label `theme::DIM` ✓, error `theme::RED` → `theme::URGENT`, help `theme::DIM` ✓.

**add_name.rs:** border `theme::GREEN` → `theme::BORDER`, title `theme::GREEN` → `theme::TEXT`, code `theme::GREEN` → `theme::ACCENT`, timer colour unchanged ✓, meta labels `theme::DIM` ✓, meta values `theme::GREEN` → `theme::TEXT`, name label `theme::DIM` ✓, name input `theme::GREEN` → `theme::TEXT`, error `theme::RED` → `theme::URGENT`, help `theme::DIM` ✓.

**delete_confirm.rs:** border `theme::RED` → `theme::URGENT`, title `theme::RED` → `theme::URGENT`, body text `theme::DIM` ✓, `[y]` `theme::RED` → `theme::URGENT`, `[n]` `theme::DIM` ✓.

- [ ] **Step 1: Update add_form.rs**

```rust
use crate::tui::{state::AppState, theme};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    let error_line = if let Some(msg) = &state.status_message {
        Line::from(Span::styled(msg.as_str(), Style::default().fg(theme::URGENT)))
    } else {
        Line::from("")
    };

    let box_height = 10u16;
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(box_height),
            Constraint::Fill(1),
        ])
        .split(area);

    let box_width = area.width.min(70);
    let pad = (area.width.saturating_sub(box_width)) / 2;
    let inner = Rect {
        x: area.x + pad,
        y: vert[1].y,
        width: box_width,
        height: box_height,
    };

    let content = vec![
        Line::from(Span::styled(
            "Add OTP",
            Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "QR code path, otpauth:// URI, or Base32 secret:",
            Style::default().fg(theme::DIM),
        )),
        Line::from(Span::styled(
            format!("{}_", state.add_secret_input),
            Style::default().fg(theme::TEXT),
        )),
        Line::from(""),
        error_line,
        Line::from(""),
        Line::from(Span::styled(
            "[ Enter ] next   [ Tab ] browse   [ Esc ] cancel",
            Style::default().fg(theme::DIM),
        )),
    ];

    f.render_widget(
        Paragraph::new(content).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme::BORDER))
                .style(Style::default().bg(theme::BG)),
        ),
        inner,
    );
}
```

- [ ] **Step 2: Update add_name.rs**

```rust
use crate::tui::{state::AppState, theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use rotp_core::totp::{generate_code_now, seconds_remaining_now};

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    let code = generate_code_now(&state.add_parsed_secret)
        .unwrap_or_else(|_| "------".to_string());
    let secs = seconds_remaining_now();
    let timer_col = theme::timer_color(secs);

    let error_line = if let Some(msg) = &state.status_message {
        Line::from(Span::styled(msg.as_str(), Style::default().fg(theme::URGENT)))
    } else {
        Line::from("")
    };

    let mut meta_lines: Vec<Line> = Vec::new();
    if let Some(meta) = &state.add_meta {
        if let Some(issuer) = &meta.issuer {
            meta_lines.push(Line::from(vec![
                Span::styled("Issuer:    ", Style::default().fg(theme::DIM)),
                Span::styled(issuer.clone(), Style::default().fg(theme::TEXT)),
            ]));
        }
        if let Some(account) = &meta.account {
            meta_lines.push(Line::from(vec![
                Span::styled("Account:   ", Style::default().fg(theme::DIM)),
                Span::styled(account.clone(), Style::default().fg(theme::TEXT)),
            ]));
        }
        let algo   = meta.algorithm.as_deref().unwrap_or("SHA1");
        let digits = meta.digits.unwrap_or(6);
        let period = meta.period.unwrap_or(30);
        meta_lines.push(Line::from(vec![
            Span::styled("Algorithm: ", Style::default().fg(theme::DIM)),
            Span::styled(
                format!("{algo}  {digits} digits  {period}s"),
                Style::default().fg(theme::DIM),
            ),
        ]));
    }

    let box_height = (10 + meta_lines.len()).min(20) as u16;
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(box_height),
            Constraint::Fill(1),
        ])
        .split(area);

    let box_width = area.width.min(60);
    let pad = (area.width.saturating_sub(box_width)) / 2;
    let inner = Rect {
        x: area.x + pad,
        y: vert[1].y,
        width: box_width,
        height: box_height,
    };

    let mut content = vec![
        Line::from(Span::styled(
            "Add OTP",
            Style::default().fg(theme::TEXT).add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::styled(
                format!("{}  {}", &code[..3], &code[3..]),
                Style::default().fg(theme::ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(format!("{}s", secs), Style::default().fg(timer_col)),
        ]),
        Line::from(""),
    ];

    content.extend(meta_lines);
    if state.add_meta.is_some() { content.push(Line::from("")); }
    content.push(Line::from(Span::styled("Name for this account:", Style::default().fg(theme::DIM))));
    content.push(Line::from(Span::styled(format!("{}_", state.add_name), Style::default().fg(theme::TEXT))));
    content.push(Line::from(""));
    content.push(error_line);
    content.push(Line::from(""));
    content.push(Line::from(Span::styled("[ Enter ] save   [ Esc ] back", Style::default().fg(theme::DIM))));

    f.render_widget(
        Paragraph::new(content)
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(theme::BORDER))
                    .style(Style::default().bg(theme::BG)),
            ),
        inner,
    );
}
```

- [ ] **Step 3: Update delete_confirm.rs**

```rust
use crate::tui::{state::AppState, theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use rotp_core::store::Vault;

pub fn render(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault) {
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    let entry_name = vault
        .entries()
        .get(state.selected_index)
        .map(|e| e.name.as_str())
        .unwrap_or("this entry");

    let box_height = 9u16;
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(box_height),
            Constraint::Fill(1),
        ])
        .split(area);

    let box_width = area.width.min(48);
    let pad = (area.width.saturating_sub(box_width)) / 2;
    let inner = Rect {
        x: area.x + pad,
        y: vert[1].y,
        width: box_width,
        height: box_height,
    };

    let content = vec![
        Line::from(Span::styled(
            "Delete this account?",
            Style::default().fg(theme::URGENT).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("{} will be permanently", entry_name),
            Style::default().fg(theme::DIM),
        )),
        Line::from(Span::styled(
            "removed from the vault.",
            Style::default().fg(theme::DIM),
        )),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled("[ y ] Yes   ", Style::default().fg(theme::URGENT).add_modifier(Modifier::BOLD)),
            Span::styled("[ n ] No", Style::default().fg(theme::DIM)),
        ]),
    ];

    f.render_widget(
        Paragraph::new(content)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(theme::URGENT))
                    .style(Style::default().bg(theme::BG)),
            ),
        inner,
    );
}
```

- [ ] **Step 4: Build — expect zero errors**

```bash
cargo build -p rotp 2>&1 | grep "^error"
```

Expected: no output.

- [ ] **Step 5: Commit**

```bash
git add rotp/src/tui/screens/add_form.rs rotp/src/tui/screens/add_name.rs rotp/src/tui/screens/delete_confirm.rs
git commit -m "style: add form, add name, delete confirm — minimal palette"
```

---

## Task 10: Final verification

- [ ] **Step 1: Full build and test suite**

```bash
cargo build -p rotp 2>&1 | grep "^error"
cargo test -p rotp-core 2>&1 | grep -E "FAILED|error\["
```

Expected: no errors, no failures.

- [ ] **Step 2: Manual smoke test**

Run the app and visually verify each screen:

```bash
cargo run -p rotp
```

Checklist:
- [ ] Unlock: dark bg, grey border, violet cursor `▌`, red error on wrong passphrase
- [ ] List: issuer bold + account below, violet code, progress bar per item, violet left border on selected, bar + code turn orange/red when timer low
- [ ] Fullscreen: dark modal, grey border, violet → orange → red big code, progress gauge
- [ ] `i` OTP Detail: grey border, violet code value
- [ ] `e` Export: violet border, violet checkmarks, violet left on selected
- [ ] Export QR: violet border, QR still black-on-white
- [ ] `a` Add form: grey border, white input text
- [ ] `d` Delete: red border, red confirm button
- [ ] Tab File picker: grey border, violet cursor, violet left border on selected

- [ ] **Step 3: Final commit**

```bash
git add -A
git commit -m "style: visual redesign complete — clean minimal + violet accent"
```
