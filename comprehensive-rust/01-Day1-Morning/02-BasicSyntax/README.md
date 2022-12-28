# Basic Syntax

"Comprehensive Rust" Day 1: Morning - Section 6

https://google.github.io/comprehensive-rust/basic-syntax.html

<!-- MarkdownTOC -->

- Examples
   - `syntax`
   - `fizzbuzz`
   - `methods`
   - `template_overloading`

<!-- /MarkdownTOC -->

## Examples

These sections explain the cargo packages that are available here in this directory, and what the expected outputs are when running the code.

### `syntax`

```bash
cd syntax;
cargo run;
```

Expected output:

```bash
❯ cargo run
   Compiling syntax v0.1.0 (/Users/__USERNAME__/__PATHTO__/syntax)
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/syntax`
a: [42, 42, 42, 42, 42, 0, 42, 42, 42, 42]
1st index: 7
2nd index: true
x: 20
r: [10, 20, 30, 40, 50, 60]
s: [30, 40]
s1: Hello
s2: Hello
s2: Hello Hello
```

### `fizzbuzz`

```bash
cd fizzbuzz;
cargo run;
```

Expected output:

```bash
❯ cargo run
   Compiling fizzbuzz v0.1.0 (/Users/__USERNAME__/__PATHTO__/fizzbuzz)
    Finished dev [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/fizzbuzz`
1
2
fizz
4
buzz
fizz
7
8
fizz
buzz
11
fizz
13
14
fizzbuzz
16
17
fizz
19
buzz
```

### `methods`

```bash
cd methods;
cargo run;
```

Expected output:

```bash
❯ cargo run
   Compiling methods v0.1.0 (/Users/__USERNAME__/__PATHTO__/methods)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/methods`
old area: 50
new area: 75
```

### `template_overloading`

```bash
cd template_overloading;
cargo run;
```

Expected output:

```bash
❯ cargo run
   Compiling template_overlading v0.1.0 (/Users/__USERNAME__/__PATHTO__/template_overlading)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/template_overlading`
coin toss: heads
cash prize: 500
```

or

```bash
❯ cargo run
   Compiling template_overlading v0.1.0 (/Users/__USERNAME__/__PATHTO__/template_overlading)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/template_overlading`
coin toss: tails
cash prize: 1000
```
