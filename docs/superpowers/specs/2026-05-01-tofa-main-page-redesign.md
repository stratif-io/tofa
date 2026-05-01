# Spec: Redesign page principale TUI — tofa

**Date:** 2026-05-01
**Statut:** Approuvé

## Contexte

La page principale actuelle (`tofa/src/tui/screens/list.rs`) souffre de trois problèmes :

1. **Trop dense** — 3 lignes par entrée (issuer+code / account / barre de progression), ce qui surcharge l'écran et cache peu d'entrées simultanément.
2. **Pas de contexte** — aucun header indiquant le nom de l'app, le nombre de comptes ou le temps restant avant rotation TOTP.
3. **Ambiance générale** — le fond `#111111` avec accent violet `#A78BFA` ne convient pas ; l'utilisateur veut quelque chose de plus propre et familier.

## Design retenu

### Palette — Slate + Bleu

| Token | Hex | Usage |
|-------|-----|-------|
| `BG` | `#0d1117` | Fond principal |
| `SURFACE` | `#0a0e14` | Header, zones secondaires |
| `BORDER` | `#21262d` | Séparateurs |
| `TEXT` | `#e6edf3` | Entrée sélectionnée |
| `DIM` | `#6e7681` | Entrées non-sélectionnées, labels |
| `MUTED` | `#30363d` | Codes masqués, footer |
| `ACCENT` | `#58a6ff` | Nom de l'app, badge |
| `CODE` | `#79c0ff` | Code TOTP sélectionné (> 10s) |
| `WARNING` | `#d29922` | Timer et code entre 5–10s |
| `URGENT` | `#f85149` | Timer et code < 5s |
| `SELECTED_BG` | `#1f3058` | Badge count |

### Structure de la liste — 1 ligne par entrée

```
tofa [4]                                        18s
› GitHub · carlo                            482 901
  Notion · work                             ••• •••
  AWS · root                                ••• •••
  Stripe · api                              ••• •••
─────────────────────────────────────────────────
↑↓ nav  ⏎ détail  h codes  a add  d del  y copy  q quit
```

**Règles :**
- Chaque entrée occupe **exactement 1 ligne** : `issuer · account` à gauche, code à droite.
- Si `account` est vide, afficher seulement `issuer`.
- Entrée sélectionnée : `›` cursor bleu + texte `TEXT` + code coloré selon timer.
- Entrées non-sélectionnées : indent 2 espaces + texte `DIM` + code masqué `••• •••` en `MUTED`.
- **Supprimer** la barre de progression `█░` par entrée — elle n'existe plus.

### Header

Une ligne fixe en haut, séparée par une bordure `BORDER` :

```
tofa [N]                                        Xs
```

- Gauche : `tofa` en `ACCENT` bold + badge `[N]` en `SELECTED_BG` / `ACCENT`.
- Droite : secondes restantes dans la fenêtre TOTP de 30s, colorées selon urgence :
  - > 10s → `CODE` (bleu)
  - 5–10s → `WARNING` (orange)
  - < 5s → `URGENT` (rouge)

### Toggle codes (`h`)

- Par défaut : seul le code de l'entrée **sélectionnée** est visible.
- Après `h` : tous les codes sont visibles, mais ceux des entrées non-sélectionnées sont affichés avec opacité réduite (couleur `DIM` au lieu de `CODE`).
- Nouvelle frappe `h` : retour à l'état masqué.

### Footer (statusline)

1 ligne fixe en bas, séparée par une bordure `BORDER` :
```
↑↓ nav  ⏎ détail  h codes  a add  d del  y copy  q quit
```
Texte en `MUTED`. Pas de changement dynamique selon le contexte.

## Fichiers impactés

| Fichier | Changement |
|---------|-----------|
| `tofa/src/tui/theme.rs` | Remplacer toute la palette de couleurs |
| `tofa/src/tui/screens/list.rs` | Réécrire le rendu : 1 ligne par entrée, header, suppression barre |

## Ce qui ne change pas

- La logique de navigation (`↑↓`, `Enter`, `a`, `d`, `y`, `q`, `h`) reste identique.
- Les autres écrans TUI (unlock, add, detail, export…) ne sont pas touchés par ce spec.
- La génération des codes TOTP (`generate_code_now`, `seconds_remaining_now`) reste inchangée.
