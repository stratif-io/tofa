# Moving an account to another authenticator

Sometimes you need to move one account out of TOFA without exporting
the whole vault — for example, sharing a service account with a
teammate, or migrating a single login to your phone. The
`otpauth://` URI is the lingua franca: every authenticator app can
import it, and TOFA can both emit and accept it.

## Single account

Copy the URI to your clipboard:

```bash
tofa code GitHub:you --uri --copy
```

The clipboard now holds an `otpauth://totp/...` URI carrying the
secret, period, digits, and algorithm. Paste it into the receiving
app's *add account* flow.

In the **TUI**, select the entry and press `u`. In the **menu bar
app**, open the entry's detail view and click the **URI** button.

## Many accounts as a list

Export the selected entries (or the whole vault) as a plain-text URI
list:

```bash
tofa export --format uris --output accounts.txt
```

The file has one `otpauth://` per line. Most authenticators will
accept it directly; otherwise re-import into another `tofa` install
with:

```bash
tofa import accounts.txt
```

In the **TUI**, open the export modal (`e` from the list) and press
`u` to write a date-stamped `.txt` to your home directory. In the
**menu bar app**, open *Export QR* and click **Save URI list (.txt)**.

## Caveats

- The URI **contains the secret** — treat the clipboard, file, or
  pasted message as you would a password. Clear the clipboard after
  use; delete the file.
- The receiving authenticator gets an exact copy of the entry, so
  both apps will produce the same codes from that point on. Decide
  ahead of time whether you want to remove the entry from TOFA after
  the move.
