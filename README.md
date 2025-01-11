# AstragoDE's Rust Template

## About this template

This template contains a basic Rust project with the [tracing](https://github.com/tokio-rs/tracing) crate and a basic logger setup.
It is recommended that you use the [Cargo Generate](https://github.com/cargo-generate/cargo-generate) crate to initialize a new project with this template.

The [`just`](https://github.com/casey/just) command runner and [Nushell](https://www.nushell.sh/) are required for some features.

## Features

- A basic logger setup using [tracing](https://github.com/tokio-rs/tracing)
- A justfile configuration for using the logger
- A [Replit](https://replit.com) configuration

## Using this template

Install [Cargo Generate](https://github.com/cargo-generate/cargo-generate) by running `cargo install cargo-generate` in your terminal.

Then run

```bash
cargo generate --git https://github.com/AstragoDETechnologies/rust_template.git
```

### Adding the template as a favorite to Cargo Generate

To add this template as a favorite to [Cargo Generate](https://github.com/cargo-generate/cargo-generate) add the following snippet to your `$CARGO_HOME/cargo-generate.toml` file. [By default](https://doc.rust-lang.org/cargo/guide/cargo-home.html#cargo-home) this file is located at `$HOME/.cargo/cargo-generate.toml`.
If the file does not exist, create it.

```toml
[favorites.astra]
description = "AstragoDE's default Rust template."
git = "https://github.com/AstragoDETechnologies/rust_template.git"
branch = "main"
vcs = "Git"
```

You can then initialize a new project with this template by running `cargo generate astra`.

## Replit Compatibility

This template contains a basic setup for [Replit](https://replit.com/).

> [!IMPORTANT]  
> Trying to run the code immediately after importing it or after opening the repl might result in an error. 
> This is likely due to the rust-analyzer, which might block files necessary for rustup to update the toolchain. 
> If you encounter this error, please be patient and try running the code again after waiting some time. 