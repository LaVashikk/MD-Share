<div align="center">
<img src="branding\logo.png" alt="Logo" >

<h3 align="center">
Md-Share is a super minimalist and lightweight web service for sharing Markdown snippets. It's built with Rust and designed to be fast, simple, and self-contained. Paste your text, get a link, and share it.
</h3>
</div>

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Lightweight & Fast:** Built with performance in mind using Axum and Tokio.
- **Markdown Rendering:** Pastes are rendered as clean HTML.
- **Expiration Control:** Set an expiration time for your pastes (e.g., 10 minutes, 1 hour, 1 day).
- **View Limiting:** Set a maximum number of views before a paste is automatically deleted.
- **"Copy Markdown" Button:** Easily copy the raw Markdown source of any paste.
- **Configurable:** Set the server port, enable verbose logging, or reset the database via command-line arguments.
- **Self-Contained:** Uses a simple SQLite database file (`pastes.db`) for storage.

## Usage

You can run the application directly with Cargo or by executing the compiled binary.

```sh
cargo build --release # or cargo run --release

# Run the optimized release binary
./target/release/mdshare
```

The server will start on `http://127.0.0.1:3000` by default.

### Command-Line Options

```
A minimalist markdown pastebin.

Usage: mdshare [OPTIONS]

Options:
  -p, --port <PORT>
          Port to listen on

          [default: 3000]

      --recreate-db
          Recreate the database on startup

          [default: false]

  -v, --verbose
          Enable verbose logging (debug level)

          [default: false]

  -h, --help
          Print help

  -V, --version
          Print version
```

## License

This project is licensed under the MIT License. See the `LICENSE` file for details (or check `Cargo.toml`).
