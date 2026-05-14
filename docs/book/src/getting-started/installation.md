# Installation

## Shell installer (macOS or Linux)

The quickest way — no Rust required:

```bash
curl -fsSL https://tofa.stratif.io/install.sh | sh
```

Installs to `~/.local/bin/tofa`. To pin a specific version:

```bash
VERSION=0.11.0 curl -fsSL https://tofa.stratif.io/install.sh | sh
```

## Homebrew (macOS or Linux)

```bash
brew tap stratif-io/tofa
brew install tofa
```

## Cargo (from crates.io)

```bash
cargo install tofa
```

## From source

`tofa` is written in Rust. Install [rustup](https://rustup.rs) (1.78 or newer),
then:

```bash
git clone https://github.com/stratif-io/tofa
cd tofa
cargo install --path tofa
```

The binary `tofa` lands in `~/.cargo/bin`.

## Verify the install

```bash
tofa --version
```

## What's next

Create your first vault: see **[Quick start](./quick-start.md)**.
