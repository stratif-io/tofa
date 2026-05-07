# docs/demo-cli.tape — VHS script for the README's TUI demo GIF.
#
# Usage:
#   vhs docs/demo-cli.tape
#
# Requires:
#   - vhs (`brew install vhs`)
#   - `tofa` on PATH (any 0.9+ build)
#   - A throwaway vault at /tmp/tofa-demo-vault.json with a few entries.
#     One-shot setup:
#       export TOFA_VAULT=/tmp/tofa-demo-vault.json TOFA_PASSPHRASE=demo
#       rm -f $TOFA_VAULT
#       tofa init
#       tofa add --name 'GitHub:carlo'      --secret JBSWY3DPEHPK3PXP
#       tofa add --name 'AWS:carlo@stratif' --secret KRSXG5DJNZSXEZJONFXG65DI
#       tofa add --name 'Vercel:stratif'    --secret MFRGGZDFM5UHIJBQGAYDQNZQ
#       tofa add --name 'Stripe:stratif'    --secret NBQXG3DTMVZWS5CHN52GS5DI
#       tofa add --name 'npm:stratif'       --secret ME2DSMRZGAYTKMRTGMZTKMBR

Output docs/demo-cli.gif

Set Theme "Catppuccin Mocha"
Set FontSize 14
Set Width 1100
Set Height 620
Set Padding 24
Set TypingSpeed 60ms

Env TOFA_VAULT      "/tmp/tofa-demo-vault.json"
Env TOFA_PASSPHRASE "demo"

Hide
Type "clear"
Enter
Sleep 400ms
Show

Type "tofa"
Sleep 200ms
Enter
Sleep 800ms

Type "demo"
Sleep 300ms
Enter
Sleep 2200ms

Down
Sleep 700ms
Down
Sleep 700ms

Type "y"
Sleep 1800ms

Type "u"
Sleep 1800ms

Type "q"
Sleep 500ms
