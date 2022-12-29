# Variables

"Comprehensive Rust" Day 1: Afternoon - Section 8

https://google.github.io/comprehensive-rust/basic-syntax/variables.html

<!-- MarkdownTOC -->

- Examples
    - Section 8.1: Type Inference
    - Section 8.2: `static` & `const`
    - Section 8.3: Scopes & Shadowing

<!-- /MarkdownTOC -->

## Examples

### Section 8.1: Type Inference

```bash
cd type_inference;
cargo run;
```

Expected output:

```bash
❯ cargo run
   Compiling type_inference v0.1.0 (/Users/__USERNAME__/__PATHTO__/type_inference)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/type_inference`
u32: 10
i8: 20
```

### Section 8.2: `static` & `const`

```bash
cd static_const;
cargo run;
```

Expected output:

```bash
❯ cargo run
   Compiling static_const v0.1.0 (/Users/__USERNAME__/__PATHTO__/static_const)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/static_const`
Digest: [222, 254, 150]
Welcome to RustOS 3.14
```

### Section 8.3: Scopes & Shadowing

```bash
cd shadowing;
cargo run;
```

Expected output:

```bash
❯ cargo run
   Compiling shadowing v0.1.0 (/Users/__USERNAME__/__PATHTO__/shadowing)
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/shadowing`
before: 10
inner scope: hello
shadowed in inner scope: true
after: 10
```
