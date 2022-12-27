# "Comprehensive Rust": Setup for macOS

<!-- MarkdownTOC -->

- Homebrew \(brew.sh\)
- Install `rust` and `rustup`
- Check for Binaries
- Test with `cargo run`

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

## Test with `cargo run`

The code in `hello/src/main.rs` was created with `cargo new hello`, and it should be able to run with the following commands by `cd`-ing into the package directory and then calling `cargo run`:

```bash
❯ cd ./hello/

❯ cargo run
   Compiling hello v0.1.0 (/Users/__USERNAME__/__PATHTO__/learning-rust/comprehensive-rust/00-Setup-macOS/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.44s
     Running `target/debug/hello`
Hello, World!
```
