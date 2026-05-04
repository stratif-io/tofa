# Scripting with `TOFA_PASSPHRASE`

For non-interactive use (CI, scripts, password manager hooks), set
`TOFA_PASSPHRASE` so `tofa` does not prompt:

```bash
export TOFA_PASSPHRASE="$(security find-generic-password -s tofa -w)"
tofa code GitHub:you
```

`tofa` prints a warning to stderr when it reads this variable. **Never bake
your passphrase into a shell history or a committed file.** Use a system
secret store instead:

- macOS Keychain: `security find-generic-password -s tofa -w`
- Linux Secret Service: `secret-tool lookup service tofa`
- 1Password CLI: `op read "op://Personal/tofa/passphrase"`

## A login-time script

```bash
#!/usr/bin/env bash
# ~/bin/otp
set -eu
export TOFA_PASSPHRASE="$(security find-generic-password -s tofa -w)"
tofa code "$1" --raw | tr -d '\n' | pbcopy
echo "copied OTP for $1"
```

Make it executable, then `otp GitHub:you` copies the current code.

## Pairing with watchdog tools

Because `tofa code --raw` exits cleanly on success and non-zero on missing
account or bad passphrase, it composes well with `set -e` scripts and CI
guards.
