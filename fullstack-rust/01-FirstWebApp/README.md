# First Web App (Example)

<!-- MarkdownTOC -->

- Package Setup
- Run

<!-- /MarkdownTOC -->

## Package Setup

These steps were already done, so you don't need to do them, but if you want to learn about how this was done, these are the steps that were followed to setup this example:

1. `cargo new messages-actix`
1. `cd messages-actix/`
1. `cargo add actix-web`
1. `cargo add env_logger`
1. `cargo add --features derive serde`
1. `cargo add serde_json`

## Run

```bash
cd messages-actix;
cargo run;
```
