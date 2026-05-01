# Redesign Page Principale TUI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Remplacer la page principale TUI de tofa par un design slate+bleu, 1 ligne par entrée, avec header contextuel et suppression des barres de progression.

**Architecture:** Deux fichiers uniquement — `theme.rs` (palette de couleurs) et `list.rs` (rendu). La logique de navigation et de génération TOTP ne change pas. Le rendu de la liste passe de 3 lignes/entrée à 1 ligne/entrée, avec un header fixe et un footer statique.

**Tech Stack:** Rust, Ratatui 0.28, `tofa-core::{Vault, generate_code_now, seconds_remaining_now}`

---

### Task 1 : Mettre à jour la palette de couleurs

**Files:**
- Modify: `tofa/src/tui/theme.rs`

- [ ] **Step 1 : Remplacer toutes les constantes de couleur**

Ouvrir `tofa/src/tui/theme.rs` et remplacer le contenu entier par :

```rust
use ratatui::style::Color;

pub const BG:          Color = Color::Rgb(13,  17,  23);   // #0d1117
pub const SURFACE:     Color = Color::Rgb(10,  14,  20);   // #0a0e14
pub const BORDER:      Color = Color::Rgb(33,  38,  45);   // #21262d
pub const TEXT:        Color = Color::Rgb(230, 237, 243);  // #e6edf3
pub const DIM:         Color = Color::Rgb(110, 118, 129);  // #6e7681
pub const MUTED:       Color = Color::Rgb(48,  54,  61);   // #30363d
pub const ACCENT:      Color = Color::Rgb(88,  166, 255);  // #58a6ff
pub const CODE:        Color = Color::Rgb(121, 192, 255);  // #79c0ff
pub const WARNING:     Color = Color::Rgb(210, 153, 34);   // #d29922
pub const URGENT:      Color = Color::Rgb(248, 81,  73);   // #f85149
pub const BADGE_BG:    Color = Color::Rgb(31,  48,  88);   // #1f3058

/// Couleur du timer et du code selon les secondes restantes dans la fenêtre TOTP.
/// > 10s → CODE (bleu), 5–10s → WARNING (orange), < 5s → URGENT (rouge)
pub fn timer_color(seconds: u64) -> Color {
    match seconds {
        s if s > 10 => CODE,
        s if s >= 5 => WARNING,
        _           => URGENT,
    }
}
```

- [ ] **Step 2 : Vérifier que le projet compile**

```bash
cargo build -p tofa 2>&1 | head -30
```

Résultat attendu : warnings possibles sur des constantes non-utilisées (`SELECTED` disparaît), mais pas d'erreur de compilation. Si erreur sur `SELECTED` ou `ACCENT` utilisé ailleurs, on le corrige à l'étape suivante.

- [ ] **Step 3 : Identifier les usages de l'ancienne palette dans les autres screens**

```bash
grep -rn "theme::" tofa/src/tui/screens/ | grep -v "list.rs"
```

Pour chaque occurrence de `theme::ACCENT` dans les autres screens, le laisser tel quel — `ACCENT` existe toujours avec une nouvelle valeur (bleu au lieu de violet). Aucune autre constante supprimée ne devrait poser problème car `SELECTED` n'était utilisée que dans `list.rs`.

- [ ] **Step 4 : Commit**

```bash
git add tofa/src/tui/theme.rs
git commit -m "feat(tui): nouvelle palette slate+bleu"
```

---

### Task 2 : Réécrire le rendu de la liste

**Files:**
- Modify: `tofa/src/tui/screens/list.rs`

- [ ] **Step 1 : Remplacer le contenu entier de `list.rs`**

Remplacer `tofa/src/tui/screens/list.rs` par :

```rust
use crate::tui::{state::AppState, theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph},
    Frame,
};
use tofa_core::{
    store::Vault,
    totp::{generate_code_now, seconds_remaining_now},
};

pub fn render(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault) {
    // Fond global
    f.render_widget(Block::default().style(Style::default().bg(theme::BG)), area);

    // Layout : header(1) + liste(min) + footer(1)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // header + séparateur
            Constraint::Min(1),    // liste
            Constraint::Length(2), // séparateur + footer
        ])
        .split(area);

    render_header(f, chunks[0], vault);
    render_list(f, chunks[1], state, vault);
    render_footer(f, chunks[2]);
    render_toast(f, area, state);
}

fn render_header(f: &mut Frame, area: Rect, vault: &Vault) {
    let secs = seconds_remaining_now();
    let timer_col = theme::timer_color(secs);
    let count = vault.entries().len();

    // Ligne du header : "tofa [N]" à gauche, "Xs" à droite
    let left = Line::from(vec![
        Span::styled("tofa", Style::default().fg(theme::ACCENT).add_modifier(Modifier::BOLD)),
        Span::raw(" "),
        Span::styled(
            format!("[{}]", count),
            Style::default().fg(theme::ACCENT).bg(theme::BADGE_BG),
        ),
    ]);
    let right = Line::from(vec![
        Span::styled(format!("{}s", secs), Style::default().fg(timer_col)),
    ]);

    // Render gauche
    f.render_widget(
        Paragraph::new(left).style(Style::default().bg(theme::SURFACE)),
        Rect { x: area.x, y: area.y, width: area.width / 2, height: 1 },
    );
    // Render droite (aligné à droite)
    f.render_widget(
        Paragraph::new(right)
            .alignment(Alignment::Right)
            .style(Style::default().bg(theme::SURFACE)),
        Rect { x: area.x + area.width / 2, y: area.y, width: area.width - area.width / 2, height: 1 },
    );
    // Séparateur
    f.render_widget(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(theme::BORDER))
            .style(Style::default().bg(theme::SURFACE)),
        Rect { x: area.x, y: area.y, width: area.width, height: 2 },
    );
}

fn render_list(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault) {
    let secs = seconds_remaining_now();
    let timer_col = theme::timer_color(secs);
    let entries = vault.entries();
    let width = area.width as usize;

    let items: Vec<ListItem> = entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let selected = i == state.selected_index;

            // Construire le label "issuer · account" ou juste "issuer"
            let label = if let Some(pos) = entry.name.find(':') {
                let issuer  = &entry.name[..pos];
                let account = &entry.name[pos + 1..];
                if account.is_empty() {
                    entry.name.clone()
                } else {
                    format!("{} · {}", issuer, account)
                }
            } else {
                entry.name.clone()
            };

            // Code ou masque
            let show = state.show_codes || selected;
            let code_str = if show {
                let code = generate_code_now(&entry.secret)
                    .unwrap_or_else(|_| "000000".to_string());
                format!("{} {}", &code[..3], &code[3..])
            } else {
                "••• •••".to_string()
            };

            // Couleurs
            let (cursor, label_col, label_mod, code_col) = if selected {
                (
                    "› ",
                    theme::TEXT,
                    Modifier::BOLD,
                    timer_col,
                )
            } else {
                (
                    "  ",
                    theme::DIM,
                    Modifier::empty(),
                    if state.show_codes { theme::DIM } else { theme::MUTED },
                )
            };

            // Padding pour aligner le code à droite
            let label_display_len = 2 + label.chars().count(); // cursor + label
            let code_display_len  = code_str.chars().count();
            let pad = width
                .saturating_sub(label_display_len)
                .saturating_sub(code_display_len);

            let line = Line::from(vec![
                Span::styled(cursor, Style::default().fg(theme::ACCENT)),
                Span::styled(label, Style::default().fg(label_col).add_modifier(label_mod)),
                Span::raw(" ".repeat(pad)),
                Span::styled(code_str, Style::default().fg(code_col)),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().style(Style::default().bg(theme::BG)));

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_index));
    f.render_stateful_widget(list, area, &mut list_state);
}

fn render_footer(f: &mut Frame, area: Rect) {
    // Séparateur + ligne de commandes
    f.render_widget(
        Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(theme::BORDER))
            .style(Style::default().bg(theme::BG)),
        area,
    );
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "↑↓ nav  ⏎ détail  h codes  a add  d del  y copy  q quit",
            Style::default().fg(theme::MUTED),
        )))
        .style(Style::default().bg(theme::BG)),
        Rect { x: area.x, y: area.y + 1, width: area.width, height: 1 },
    );
}

fn render_toast(f: &mut Frame, area: Rect, state: &AppState) {
    let Some(msg) = &state.status_message else { return };

    let is_copy = msg.contains("Copied") || msg.contains("copied");
    let (border_col, text_col) = if is_copy {
        (theme::ACCENT, theme::ACCENT)
    } else {
        (theme::URGENT, theme::URGENT)
    };
    let label = if is_copy {
        format!("  ✓  {}  ", msg)
    } else {
        format!("  {}  ", msg)
    };
    let toast_w = (label.chars().count() as u16 + 2).min(area.width);
    let toast_h = 3u16;
    let toast = Rect {
        x: area.x + (area.width.saturating_sub(toast_w)) / 2,
        y: area.y + (area.height.saturating_sub(toast_h)) / 2,
        width: toast_w,
        height: toast_h,
    };
    f.render_widget(Clear, toast);
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_col))
            .style(Style::default().bg(theme::BG)),
        toast,
    );
    let inner = Rect {
        x: toast.x + 1,
        y: toast.y + 1,
        width: toast.width.saturating_sub(2),
        height: 1,
    };
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            label,
            Style::default().fg(text_col).add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center),
        inner,
    );
}
```

- [ ] **Step 2 : Compiler et vérifier qu'il n'y a pas d'erreur**

```bash
cargo build -p tofa 2>&1
```

Résultat attendu : compilation réussie, éventuellement des warnings sur imports inutilisés (ex. `Direction` si le layout change) — corriger les imports en tête de fichier si nécessaire en retirant les identifiants non utilisés.

- [ ] **Step 3 : Lancer l'app manuellement et vérifier visuellement**

```bash
TOFA_VAULT=~/.config/tofa/vault.enc cargo run -p tofa
```

Vérifier :
- Header : `tofa [N]` à gauche, timer en secondes à droite, couleur selon urgence
- Liste : 1 ligne par entrée, code visible uniquement pour la sélection
- Touche `h` : toggle affichage de tous les codes
- Footer : une seule ligne statique de commandes
- Aucune barre de progression `█░` visible

- [ ] **Step 4 : Lancer les tests automatisés**

```bash
cargo test -p tofa 2>&1
```

Résultat attendu : tous les tests passent (les tests CLI ne testent pas le rendu TUI, ils ne sont pas impactés).

- [ ] **Step 5 : Commit**

```bash
git add tofa/src/tui/screens/list.rs
git commit -m "feat(tui): redesign page principale — 1 ligne/entrée, header contextuel, palette slate+bleu"
```
