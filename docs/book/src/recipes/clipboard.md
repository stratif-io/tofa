# Copying codes to the clipboard

`tofa code` writes the 6-digit OTP to **stdout** and the countdown to stderr,
which makes it pipe-friendly. There's also a built-in `--copy` flag that
handles the platform clipboard for you.

## Built-in flag

```bash
tofa code GitHub:you --copy
```

Works on macOS, Linux (X11 and Wayland), and Windows.

## Manual piping

If you'd rather use your own tool — e.g., to integrate with a password
manager — combine `--raw` (no separator space) with the platform's
clipboard binary:

### macOS

```bash
tofa code GitHub:you --raw | pbcopy
```

### Linux (X11)

```bash
tofa code GitHub:you --raw | xclip -selection clipboard
```

### Linux (Wayland)

```bash
tofa code GitHub:you --raw | wl-copy
```

## Avoid leaving the code in the clipboard

The TOTP expires every 30 seconds anyway, but if you want to clear sooner:

```bash
tofa code GitHub:you --raw | pbcopy && (sleep 15; pbcopy < /dev/null)
```
