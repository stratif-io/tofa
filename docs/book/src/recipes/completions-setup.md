# Shell completions

`tofa completions <shell>` prints completions to stdout. Install them once
and forget.

## zsh

```bash
mkdir -p ~/.zsh/completions
tofa completions zsh > ~/.zsh/completions/_tofa
```

Add to `~/.zshrc` (if not already there):

```bash
fpath=(~/.zsh/completions $fpath)
autoload -Uz compinit && compinit
```

## bash

```bash
tofa completions bash > /usr/local/etc/bash_completion.d/tofa
```

## fish

```bash
tofa completions fish > ~/.config/fish/completions/tofa.fish
```

## powershell

```powershell
tofa completions powershell | Out-String | Invoke-Expression
```

(Add the same line to your PowerShell profile to persist.)

## See also

[`tofa completions`](../reference/completions.md) — flag reference.
