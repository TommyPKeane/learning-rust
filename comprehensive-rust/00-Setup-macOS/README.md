# "Comprehensive Rust": Setup for macOS

<!-- MarkdownTOC -->

- Homebrew \(brew.sh\)
- Install `rust` and `rustup`
- Check for Binaries

<!-- /MarkdownTOC -->

## Homebrew (brew.sh)

Install [`brew`](https://brew.sh/) on your macOS System:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

## Install `rust` and `rustup`

```bash
brew install rust
```

```bash
brew install rustup
```

## Check for Binaries

```bash
❯ which rustup
/Users/USERNAME/.cargo/bin/rustup
```

```bash
❯ which rustc
/Users/USERNAME/.cargo/bin/rustc
```

```bash
❯ which cargo
/Users/USERNAME/.cargo/bin/cargo
```
