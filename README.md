# Learning Rust (tommypkeane)

This repository provides some examples and personal efforts in learning how to develop and build applications with the Rust programming language.

<!-- MarkdownTOC -->

- Repo Structure
    - "Comprehensive Rust" \(`google-comprehensive-rust`\)
- Terminology
    - Cargo
    - Crate
    - Package
    - Target
- References

<!-- /MarkdownTOC -->

## Repo Structure

These sections go through explanations and reference links related to each directory, to summarize the contents of this repository.

### "Comprehensive Rust" (`google-comprehensive-rust`)

This directory contains examples and tutorial code from going through Google's "Comprehensive Rust" online book.

See the local [README](./comprehensive-rust/README.md) for more details.

- https://github.com/google/comprehensive-rust/
- https://google.github.io/comprehensive-rust/welcome.html

## Terminology

Rust has some unique terminology related to it, which may not be immediately familiar even if you've developed in C, C++, Python, JavaScript, or other contemporary programming languages, so this section provides some explanations for Rust-specific terminology.

### Cargo

[`cargo`](https://doc.rust-lang.org/cargo/appendix/glossary.html#cargo) is the Rust Package Manager

### Crate

There are `library crates` and `binary crates`, which are either code libraries (like "packages") that are importable, or are executable standalone binary applications. [`crates`](https://doc.rust-lang.org/cargo/appendix/glossary.html#crate) are created and managed by `cargo` and the Rust compiler as the subset of artifacts of a `cargo` managed `package`.

### Package

A [`package`](https://doc.rust-lang.org/cargo/appendix/glossary.html#package) in Rust is made-up of `crates` from Rust source code, to have a locally used package, or an importable package as installed _via_ `cargo`.

### Target

A [`target`](https://doc.rust-lang.org/cargo/appendix/glossary.html#target) is a concept related to Rust `packages` managed by `cargo`, typically, but can also have the generic meaning like a "target architecture" or "target directory".

## References

- [Documentation for the `std` Crate in Rust](https://doc.rust-lang.org/std/index.html)
- [Rust `cargo` Book](https://doc.rust-lang.org/cargo/index.html)
- ["Comprehensive Rust" Book](https://google.github.io/comprehensive-rust/welcome.html)
