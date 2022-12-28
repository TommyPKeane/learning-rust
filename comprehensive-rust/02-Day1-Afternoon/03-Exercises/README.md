# Exercises

"Comprehensive Rust" Day 1: Afternoon - Section 11

https://google.github.io/comprehensive-rust/exercises/day-1/afternoon.html

<!-- MarkdownTOC -->

- Exercises
    - 11.1: Designing a Library
    - 11.2: Iterators and Ownership

<!-- /MarkdownTOC -->

## Exercises

### 11.1: Designing a Library

```bash
cd library;
cargo run;
```

Original Errored Output:

```bash
```

Expected Output (fixed):

```bash
```

### 11.2: Iterators and Ownership

```bash
cd ownership;
cargo run;
```

Original Errored Output:

```bash
❯ cargo run
   Compiling ownership v0.1.0 (/Users/__USERNAME__/__PATHTO__/ownership)
error: invalid const generic expression
  --> src/main.rs:22:20
   |
22 |     let v0: Option<..> = iter.next();
   |                    ^^
   |
help: expressions must be enclosed in braces to be used as const generic arguments
   |
22 |     let v0: Option<{ .. }> = iter.next();
   |                    +    +

error: invalid const generic expression
  --> src/main.rs:29:20
   |
29 |     let w0: Option<..> = iter.next();
   |                    ^^
   |
help: expressions must be enclosed in braces to be used as const generic arguments
   |
29 |     let w0: Option<{ .. }> = iter.next();
   |                    +    +

error[E0747]: constant provided when a type was expected
  --> src/main.rs:22:20
   |
22 |     let v0: Option<..> = iter.next();
   |                    ^^

error[E0747]: constant provided when a type was expected
  --> src/main.rs:29:20
   |
29 |     let w0: Option<..> = iter.next();
   |                    ^^

For more information about this error, try `rustc --explain E0747`.
error: could not compile `ownership` due to 4 previous errors
```

Expected Output (fixed):

```bash
❯ cargo run
   Compiling ownership v0.1.0 (/Users/__USERNAME__/__PATHTO__/ownership)
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/ownership`
v[0]: Some(10)
v[1]: Some(20)
v[2]: Some(30)
No more items: None
v0: None
w0: Some("foo")
word: foo
word: bar
word: foo
word: bar
```
