# Starter

Look at the [Actix Web documentation](https://actix.rs/docs/getting-started) to learn more.

## Setup

Make sure to install dependencies:

```bash
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# check version
rustc --version

# install watchexec
cargo install watchexec-cli
```

## Development Server

Start the development server on `http://localhost:8080`:

```bash
# watchexec
watchexec -e rs -r cargo r
```