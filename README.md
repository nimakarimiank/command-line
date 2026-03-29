# herogrep (Mini-Grep in Rust)

A small command-line search tool inspired by the Rust Book project:
"An I/O Project: Building a Command Line Program".

This repository is focused on learning Rust by building a practical CLI application that reads command-line arguments, opens files, and searches for matching lines.

## Why This Project

This project is based on the classic `grep` workflow:

1. Accept a search query and file path from command-line arguments.
2. Read the file contents.
3. Find matching lines.
4. Print results to the terminal.

Along the way, it helps practice:

- Module organization
- Ownership and borrowing with strings
- Error handling using `Result`
- Testing with `cargo test`
- Environment-variable-driven behavior (case-insensitive mode)

## Project Status

Current state:

- Basic CLI argument reading is implemented in [src/main.rs](src/main.rs).
- The binary prints what query and file were provided.

Planned next steps (from the Rust Book chapter flow):

- Move argument parsing into a `Config` struct in a library module.
- Read file contents and search for matching lines.
- Add `IGNORE_CASE` environment variable support.
- Route errors to stderr.
- Add unit tests for search behavior.

## Tech Stack

- Rust (Edition 2024)
- Cargo for build/test/run

## Getting Started

### Prerequisites

Install Rust and Cargo:

- <https://www.rust-lang.org/tools/install>

Check your installation:

```bash
rustc --version
cargo --version
```

### Clone

```bash
git clone https://github.com/<your-username>/herogrep.git
cd herogrep
```

## Run

Because the current CLI expects two positional arguments (`query` and `file`), run it like this:

```bash
cargo run -- to test.txt
```

Example output (current implementation):

```text
Searching for to
In file test.txt
```

## Build

```bash
cargo build
```

## Test

```bash
cargo test
```

Note: tests will become more meaningful as search logic is added to `src/lib.rs`.

## Planned Usage (Target Behavior)

Once the full mini-grep behavior is implemented, usage will look like:

```bash
cargo run -- <query> <file>
```

Case-insensitive mode will be enabled with an environment variable:

```bash
IGNORE_CASE=1 cargo run -- <query> <file>
```

## Suggested Repository Structure

```text
.
├── Cargo.toml
├── README.md
├── src
│   ├── main.rs
│   └── lib.rs
└── test.txt
```

## Publish to GitHub

If your repository is not yet linked to GitHub:

```bash
git init
git add .
git commit -m "Initial mini-grep project"
git branch -M main
git remote add origin https://github.com/<your-username>/herogrep.git
git push -u origin main
```

If your remote already exists:

```bash
git add .
git commit -m "Update README"
git push
```

## Learning Reference

- Rust Book, Chapter 12: “An I/O Project: Building a Command Line Program”
  - <https://doc.rust-lang.org/book/ch12-00-an-io-project.html>

## License

Choose one and add a LICENSE file (recommended: MIT or Apache-2.0).
