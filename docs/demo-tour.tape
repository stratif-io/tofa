# docs/demo-tour.tape — extended feature tour for the README's <details> section.
#
# Renders a longer GIF showcasing: unlock, copy, hide-codes, fullscreen detail,
# secret reveal, QR display, file-picker import, QR export, and the CLI's
# `list`, `list --codes`, and `qr <name>` commands.
#
# Usage:
#   vhs docs/demo-tour.tape
#
# Requires:
#   - vhs (`brew install vhs`)
#   - `tofa` 0.9+ on PATH
#   - Setup the throwaway demo dir + vault first:
#       rm -rf /tmp/tofa-demo && mkdir /tmp/tofa-demo
#       # Stage 1: temp vault to mint a QR PNG for an entry NOT in the demo vault.
#       TOFA_VAULT=/tmp/_t.json TOFA_PASSPHRASE=t tofa init
#       TOFA_VAULT=/tmp/_t.json TOFA_PASSPHRASE=t tofa add --name 'Linear:carlo' --secret HELLOEXAMPLEYESEXAMPLEEXAM
#       TOFA_VAULT=/tmp/_t.json TOFA_PASSPHRASE=t tofa qr 'Linear:carlo' --output /tmp/tofa-demo/linear.png
#       rm -f /tmp/_t.json
#       # Stage 2: demo vault with 5 entries, no Linear.
#       export TOFA_VAULT=/tmp/tofa-demo-vault.json TOFA_PASSPHRASE=demo
#       rm -f $TOFA_VAULT
#       tofa init
#       tofa add --name 'GitHub:carlo'      --secret JBSWY3DPEHPK3PXP
#       tofa add --name 'AWS:carlo@stratif' --secret KRSXG5DJNZSXEZJONFXG65DI
#       tofa add --name 'Vercel:stratif'    --secret MFRGGZDFM5UHIJBQGAYDQNZQ
#       tofa add --name 'Stripe:stratif'    --secret NBQXG3DTMVZWS5CHN52GS5DI
#       tofa add --name 'npm:stratif'       --secret ME2DSMRZGAYTKMRTGMZTKMBR

Output docs/demo-tour.gif
Output docs/demo-tour.mp4

Set Theme "Catppuccin Mocha"
Set FontSize 13
Set Width 1150
Set Height 720
Set Padding 22
Set TypingSpeed 55ms

Env TOFA_VAULT      "/tmp/tofa-demo-vault.json"
Env TOFA_PASSPHRASE "demo"

Hide
Type "cd /tmp/tofa-demo && clear"
Enter
Sleep 400ms
Show

# ── Open TUI + unlock ────────────────────────────────────────────────
Type "tofa"
Sleep 200ms
Enter
Sleep 700ms
Type "demo"
Sleep 200ms
Enter
Sleep 1700ms

# ── Navigate, copy code ──────────────────────────────────────────────
Down
Sleep 350ms
Down
Sleep 400ms
Type "y"
Sleep 1400ms

# ── Hide codes (privacy / screen-share) ──────────────────────────────
Type "h"
Sleep 1500ms
Type "h"
Sleep 600ms

# ── Open fullscreen detail ───────────────────────────────────────────
Space
Sleep 1500ms

# ── Reveal secret ────────────────────────────────────────────────────
Type "s"
Sleep 600ms
Type "demo"
Sleep 200ms
Enter
Sleep 2000ms

# ── Show QR for the entry ────────────────────────────────────────────
Type "r"
Sleep 2400ms

# Back to fullscreen
Escape
Sleep 500ms
# Back to list
Escape
Sleep 500ms

# ── Add via PNG file picker ─────────────────────────────────────────
Type "a"
Sleep 800ms
Tab
Sleep 1100ms
Down
Sleep 500ms
Space
Sleep 600ms
Enter
Sleep 1800ms

# ── QR export view ──────────────────────────────────────────────────
Type "e"
Sleep 1800ms
Escape
Sleep 500ms

# ── Quit TUI ─────────────────────────────────────────────────────────
Type "q"
Sleep 600ms

# ── CLI: tofa list ───────────────────────────────────────────────────
Type "tofa list"
Sleep 200ms
Enter
Sleep 1300ms

# ── CLI: tofa list --codes ──────────────────────────────────────────
Type "tofa list --codes"
Sleep 200ms
Enter
Sleep 2000ms

# ── CLI: tofa qr <name> in terminal ─────────────────────────────────
Type "tofa qr GitHub:carlo"
Sleep 200ms
Enter
Sleep 2800ms
