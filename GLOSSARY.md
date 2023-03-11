# Terminology of Rust

Rust has some unique terminology related to it, which may not be immediately familiar even if you've developed in C, C++, Python, JavaScript, or other contemporary programming languages, so this section provides some explanations for Rust-specific terminology.

<!-- MarkdownTOC -->

- [Cargo](#cargo)
- [Crate](#crate)
- [Package](#package)
- [Target](#target)

<!-- /MarkdownTOC -->


<a id="cargo"></a>
## Cargo

[`cargo`](https://doc.rust-lang.org/cargo/appendix/glossary.html#cargo) is the Rust Package Manager

<a id="crate"></a>
## Crate

There are `library crates` and `binary crates`, which are either code libraries (like "packages") that are importable, or are executable standalone binary applications. [`crates`](https://doc.rust-lang.org/cargo/appendix/glossary.html#crate) are created and managed by `cargo` and the Rust compiler as the subset of artifacts of a `cargo` managed `package`.

<a id="package"></a>
## Package

A [`package`](https://doc.rust-lang.org/cargo/appendix/glossary.html#package) in Rust is made-up of `crates` from Rust source code, to have a locally used package, or an importable package as installed _via_ `cargo`.

<a id="target"></a>
## Target

A [`target`](https://doc.rust-lang.org/cargo/appendix/glossary.html#target) is a concept related to Rust `packages` managed by `cargo`, typically, but can also have the generic meaning like a "target architecture" or "target directory".
