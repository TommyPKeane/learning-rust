# Machine Learning in Rust with `tch-rs` (`libtorch`)

[`tch-rs`](https://github.com/LaurentMazare/tch-rs) provides a Rust package for building against `libtorch` (the C++ basis for [PyTorch](https://pytorch.org/)) for Machine Learning algorithms and models with "tensors".

<!-- MarkdownTOC -->

- Prequisite Tools
- Development Setup
- Build and Run
    - `example-char-rnn`
    - `example-computationalgraph`
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

Note that these examples are setup using the following commands:

- `cargo new DIRECTORYNAME`
- `cargo add --features python,cpython tch`

### `example-char-rnn`

An example of a `char-rnn` (Character-based Recurrent Neural Network) for training a Natural Language Processing (NLP) Deep-Learning Model for generating sample text.

Code has come from these references and been updated for readability and testing purposes:

-

⚠️ Note that this runs the Training algorithm with a final output after `N` Epochs (iterations) of "Sample Text", so the runtime is largely dependent upon the size of the training dataset (the text) and the number of Epochs (`N`). Running on a standard laptop, this can take seconds, minutes, hours, days, or weeks, depending on the input data and number of Epochs (`N`).


```bash
cd example-char-rnn;
cargo run
```

The above will build, then run the code, which will read-in the `example-char-rnn/data/input.txt` plaintext file and use all the characters in this file as the training data for training a new Machine Learning Algorithm/"Model", based on the `nn::lstm`, `nn::linear`, and `nn::Adam` models from `torch` (`tch`).

Depending on the configuration you should see something like this as the output, and then a sample text will print when all Epochs (overall training iterations) have completed:

```bash
❯ cargo run
   Compiling example-char-rnn v0.1.0 (/Users/__USERNAME__/__PATHTO__/example-char-rnn)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/example-char-rnn`
Dataset loaded, 58 labels.
+ Processing Epoch 1 (BATCH_SIZE: 256):
[00:00:44] ########################################     101/
  > Epoch: 1, loss: 2.508

+ Processing Epoch 2 (BATCH_SIZE: 256):
[00:00:48] ########################################     101/101
  > Epoch: 2, loss: 1.378

  > Sample:
{{SAMPLE TEXT WILL BEEN PRINTED HERE}}
```

The goal of this ML Algorithm is to take the input data and train itself to recognize the characters and their relational structure to each other to recreate text that is similar to the `input.txt` file.

In a loosely generic sense, if we're using a sample text based-on the writings of William Shakespeare, then you can think of this ML Algorithm as being a character-based "NLP Model" that will generate sample text that looks like something that Shakespeare would've written.

Note: It should be made clear that this is not necessarily relevant to do anything, because it is very unlikely that the individual unicode characters and their relative layouts are themselves, alone, sufficiently indicative to recreate the writing style of William Shakespeare while also producing something readable, sensible, and acceptably similar. On the other hand, there is potentially an argument to be made that the painting style of an artist could potentially be recreated if the qualia of the brush strokes are "learned" and can be sampled in sensibly appropriate manner. This is the core complexity of "fitness" ("goodness" / "success") in Machine Learning.

### `example-computationalgraph`

This is an example based on the article and code from these links (minimal edits made):

- https://github.com/vegapit/tchtut/blob/master/examples/blackpricing.rs
- https://vegapit.com/article/how-to-use-torch-in-rust-with-tch-rs

```bash
cd example-computationalgraph;
cargo run
```

Expected Output:

```bash
❯ cargo run
   Compiling ... // REDACTED FOR BREVITY
   Compiling example-computationalgraph v0.1.0 (/Users/__USERNAME__/__PATHTO__/example-computationalgraph)
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/example-computationalgraph`
price: 11.8049
delta: 0.5540, vega: 39.0554
gamma: 0.0130, vanna: 0.1953, volga: -2.9292
```

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
   Compiling example-tensors v0.1.0 (/Users/__USERNAME__/__PATHTO__/example-tensors)
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
- https://doc.rust-lang.org/cargo/reference/environment-variables.html
