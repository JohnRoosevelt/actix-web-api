## Starter
Look at the [Actix Web documentation](https://actix.rs/docs/getting-started) to learn more.

## Setup
Make sure to install dependencies:

#### install rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### check version
```bash
cargo -vV
```

#### install watchexec
```bash
cargo install watchexec-cli
```

## Development Server

Start the development server on `http://localhost:8080`:

#### watchexec
```bash
watchexec -e rs -r cargo r
```

## install dependencies crate
```bash
cargo add chrono --features serde,clock
```
