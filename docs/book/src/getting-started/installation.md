# Installation

## From source (recommended)

`tofa` is written in Rust. Install [rustup](https://rustup.rs) (1.78 or newer),
then:

```bash
git clone https://github.com/stratif-io/tofa
cd tofa
cargo install --path tofa
```

The binary `tofa` lands in `~/.cargo/bin`.

## From crates.io

```bash
cargo install tofa
```

## Verify the install

```bash
tofa --version
```

## What's next

Create your first vault: see **[Quick start](./quick-start.md)**.
