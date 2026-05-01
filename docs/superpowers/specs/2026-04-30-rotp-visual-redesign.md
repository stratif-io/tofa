# rotp Visual Redesign — Design Spec

## Goal

Replace the current monochrome green TUI with a Clean & Minimal design: near-black background, white/grey text hierarchy, single violet accent for OTP codes, and a dual timer signal (colour shift + progress bar).

## Palette

| Token       | Hex       | Usage                                      |
|-------------|-----------|--------------------------------------------|
| BG          | `#111111` | Full terminal background                   |
| Surface     | `#1C1C1C` | List item hover / modal body               |
| Selected    | `#1E1A2E` | Active list row background                 |
| Text        | `#E0E0E0` | Primary text (issuer, modal code)          |
| Dim         | `#555555` | Secondary text (account, help bar, labels) |
| Muted       | `#2A2A2A` | Non-selected codes when show_codes=false   |
| Accent      | `#A78BFA` | OTP code, borders, progress bar, cursor    |
| Warning     | `#FB923C` | Timer 5–10 s                               |
| Urgent      | `#F87171` | Timer < 5 s, error messages                |
| Border      | `#2A2A2A` | Modal borders, dividers                    |

All existing `theme::*` constants are replaced. No other colours are used anywhere.

## Timer colour rule

```
secs > 10  → Accent  (#A78BFA)
secs 5–10  → Warning (#FB923C)
secs < 5   → Urgent  (#F87171)
```

The progress bar and the code text both follow this rule simultaneously.

## List screen

**Layout:** full-area background BG, no header row, help bar 1 row at bottom.

**Each item (non-selected):**
- Row: `  {Issuer}` (Dim) on one line, `  {account}` (Muted, 9-10px equivalent = smaller span) below — two visual lines per account but rendered as a single `ListItem` with a `\n` or two-line `Text`.
- Code: right-aligned, Muted when `show_codes=false`, timer-coloured when visible.
- Progress bar: 1-cell-height `█` characters or a `Gauge` with `ratio = secs/30`, timer-coloured, rendered on the row immediately below the item — always visible (shows full bar when codes hidden, so user knows time is passing).

**Selected item:**
- Background: Selected (`#1E1A2E`), `BorderType::Plain` left border 1 char wide in Accent.
- Issuer: Text bold.
- Account: Dim.
- Code: timer-coloured, bold, always visible regardless of `show_codes`.
- Progress bar below: timer-coloured.

**Help bar:** single line, Dim. Adjusts dynamically: `h` label toggles between `show` / `hide`.

## Unlock screen

Same centred box layout as today. Changes:
- Border colour: Border (`#2A2A2A`) instead of green.
- Title `r o t p`: Text (`#E0E0E0`), bold, letter-spaced.
- Subtitle `OTP Manager`: Dim.
- Passphrase label: Dim.
- Input `••••▌`: Accent for the cursor `▌`, Dim for the dots.
- Error message: Urgent.
- Help text: Muted.

## Fullscreen modal (big code)

- Background: list rendered behind at full opacity, modal overlaid with `Clear`.
- Modal border: Border colour.
- Header: `{Issuer} · {account}` in Dim, centred.
- Big code: Accent → Warning → Urgent per timer rule, bold. `tui-big-text` unchanged.
- Progress bar: full-width, 1 row, timer-coloured.
- Help: `Esc back · y copy`, Dim.

## OTP Detail modal

- Same overlay pattern (list behind + Clear + modal).
- Border: Border colour.
- Fields: label in Dim, value in Text. Code value uses timer colour.
- Help: Dim.

## Export modal (checkbox list)

- Overlay over list.
- Border: Accent.
- Checkbox `[✓]` / `[ ]`: Accent when checked, Dim when unchecked.
- Selected row: Selected background, Accent text.
- Help: Dim.

## Export QR modal

- Border: Accent.
- Title `Scan with your authenticator app`: Text, bold.
- QR area: always `fg(Black) bg(White)`, unchanged (scanability requirement).
- "Too small" error modal: border Urgent, message Text + Dim.

## Scanning QR overlay

- Clear + modal, border Accent.
- Spinner char + message in Dim.

## File picker screen

- Full-screen, BG background.
- Query line: `/ {query}▌` — Accent for the cursor, Text for query.
- Directories: Dim with `▸` prefix.
- Files: Text.
- Selected row: Selected background, Accent left border.

## Add form / Add name screens

- Full-screen, BG background.
- Labels: Dim. Input values: Text. Parsed metadata: Dim.
- Status error: Urgent.

## Delete confirm modal

- Border: Urgent.
- Question: Text. Item name: bold Text.
- `[y]` confirm: Urgent. `[n]` cancel: Dim.

## Implementation scope

1. Replace `theme.rs` constants with the new palette.
2. Rewrite `screens/list.rs` — two-line items, progress bar row, Selected treatment.
3. Update `screens/unlock.rs` — colours only.
4. Update `screens/fullscreen.rs` — colours + progress bar.
5. Update `screens/otp_detail.rs` — colours only.
6. Update `screens/export.rs` — colours only.
7. Update `screens/export_qr.rs` — colours only (QR area unchanged).
8. Update `screens/scanning_qr.rs` — colours only.
9. Update `screens/file_picker.rs` — colours + Selected treatment.
10. Update `screens/add_form.rs`, `screens/add_name.rs`, `screens/delete_confirm.rs` — colours only.

No logic changes. No new dependencies. All ratatui rendering, no external crates needed.
