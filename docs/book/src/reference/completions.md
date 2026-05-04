# tofa completions

Print shell completions for `tofa`. Pipe the output into your shell's
completion directory.

<!-- BEGIN auto:help -->
**Synopsis**

```
tofa completions
```

<!-- END auto:help -->

## Examples

zsh:

```bash
tofa completions zsh > ~/.zsh/completions/_tofa
```

bash:

```bash
tofa completions bash > /usr/local/etc/bash_completion.d/tofa
```

fish:

```bash
tofa completions fish > ~/.config/fish/completions/tofa.fish
```

## Notes

- Supported shells: `bash`, `zsh`, `fish`, `powershell`, `elvish`.
- Output is plain text — safe to redirect to a file.

## See also

- **[Recipe: completions setup](../recipes/completions-setup.md)** — full
  install instructions per shell, including `fpath` setup for zsh.
