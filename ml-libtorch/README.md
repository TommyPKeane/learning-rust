# Machine Learning in Rust with `tch-rs` (`libtorch`)

[`tch-rs`](https://github.com/LaurentMazare/tch-rs) provides a Rust package for building against `libtorch` (the C++ basis for [PyTorch](https://pytorch.org/)) for Machine Learning algorithms and models with "tensors".

<!-- MarkdownTOC -->

- Prequisite Tools
- Development Setup
- Build and Run
    - `example-tensors`
- References

<!-- /MarkdownTOC -->

## Prequisite Tools

Required Tooling:

- `direnv`: https://direnv.net/
    - `brew install direnv`: https://formulae.brew.sh/formula/direnv
- `pyenv`: https://github.com/pyenv/pyenv
    - `brew install pyenv`: https://formulae.brew.sh/formula/pyenv
- `rust` (`cargo`): https://www.rust-lang.org/
    - `brew install rust`: https://formulae.brew.sh/formula/rust
    - `brew install rustup`: https://formulae.brew.sh/formula/rustup-init

All of the above can be installed on macOS by using [`brew`](https://brew.sh/) (Homebrew), which will manage updates and linkages for your system, to make your macOS system more like a Linux environment.

You should also be able to install the same tools and run the same code on most Linux variants, and likely in Docker (with a Linux image) as well.

## Development Setup

Before the examples can be run, you'll need to make sure that you've got all the prerequisite tools, and then you can run the following commands to make sure that all the rust examples will work.

1. `pyenv install` (If you already have the relevant version from the `.python-version` file, then you don't need to do this step)
1. `direnv allow` -- This reads the `.envrc` file and pre-emptively sets Environment Variables, even if the paths aren't valid until you finish running all the commands here
1. `pip install --upgrade pip`
1. `pip install` -- This installs `pytorch` based on the `poetry.lock` file which is managed _via_ the `pyproject.toml` file.

Once you've gotten all the Python dependencies installed, you can see that the `.envrc` file has set some Environment Variables which will allow the Rust package `tch` (`tch-rs`) be able to build and link against the Rust source code provided in the examples here.

At this point, you can now go through and build/run any of the examples as documented below.

(_Note: any system you want to build or run these examples on will need to have the requisite depdencies, unless you build a static application; though note that software licensing considerations may be adjusted if you statically, instead of dynamically, link/bundle any of the depdencies referenced here._)

## Build and Run

The subsections here provide details on each `crate` (Rust package) setup in this directory of examples related to `libtorch` by using `tch-rs` (`cargo add tch`) for Rust programming of Machine Learning algorithms.

### `example-tensors`

This is a simple example executable from the `tch-rs` repo's README which shows how to import `tch` and use the `libtorch` `Tensor` objects for some mathematical computations.

```bash
cd example-tensors;
cargo run
```

Expected Output:

```bash
❯ cargo run
   Compiling ... // REDACTED FOR BREVITY
   Compiling example-tensors v0.1.0 (/Users/tommy/dev/tommypkeane/learning-rust/ml-libtorch/example-tensors)
    Finished dev [unoptimized + debuginfo] target(s) in 16.79s
     Running `target/debug/example-tensors`
  6
  2
  8
  2
 10
[ CPUIntType{5} ]
```

## References

- https://github.com/LaurentMazare/tch-rs
- https://pytorch.org/
- http://vegapit.com/article/how-to-use-torch-in-rust-with-tch-rs
- https://github.com/metobom/tchrs-opencv-webcam-inference
- https://github.com/LaurentMazare/tch-rs/tree/main/examples/mnist
