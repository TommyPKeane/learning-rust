# Exercises (Day 1: Morning)

"Comprehensive Rust" Day 1: Morning - Section 7

https://google.github.io/comprehensive-rust/exercises/day-1/morning.html

<!-- MarkdownTOC -->

- Exercises
   - 7.1: Implicit Conversions
   - 7.2: Arrays and `for` Loops

<!-- /MarkdownTOC -->

## Exercises

### 7.1: Implicit Conversions

```bash
cd conversions;
cargo run;
```

Expected Output:

```bash
❯ cargo run
   Compiling conversions v0.1.0 (/Users/__USERNAME__/__PATHTO__/conversions)
error[E0308]: mismatched types
 --> src/main.rs:9:41
  |
9 |     println!("{x} * {y} = {}", multiply(x, y));
  |                                         ^ expected `i16`, found `i8`
  |
help: you can convert an `i8` to an `i16`
  |
9 |     println!("{x} * {y} = {}", multiply(x.into(), y));
  |                                          +++++++

For more information about this error, try `rustc --explain E0308`.
error: could not compile `conversions` due to previous error
```

### 7.2: Arrays and `for` Loops

```bash
cd arrays_loops;
cargo run;
```

Expected Output:

```bash
❯ cargo run
   Compiling arrays_loops v0.1.0 (/Users/__USERNAME__/__PATHTO__/arrays_loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/arrays_loops`
matrix:
thread 'main' panicked at 'not implemented', src/main.rs:9:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
