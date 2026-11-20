# Rust practice

This repository contains small Rust exercises covering ownership, error
handling, enums, file I/O, references, collections, and simple data
structures.

## Exercises

| Directory | Topic |
|---|---|
| `hello`, `hello1` | Basic Rust programs |
| `bank` | Structs and application-style data handling |
| `char_frequency` | Text processing and collections |
| `enum` | Enum modeling |
| `error` | Error handling |
| `file_finder` | File-system traversal |
| `other` | Additional language experiments |
| `refr` | References and borrowing |
| `safe_split_str` | Safe string slicing |
| `simple_stack` | Stack data structure |
| `iterators` | Iterator adapters and collection |
| `option_result` | Parsing and recoverable errors |
| `generics` | Generic functions and bounds |
| `threads` | Thread creation and joining |
| `channels` | Message passing between threads |
| `lifetimes` | Explicit lifetime annotations |
| `traits` | Trait definitions and generic interfaces |

## Running an exercise

Each directory is kept as an independent practice example. Enter a directory
and use Cargo when that directory contains a `Cargo.toml`:

```bash
cd simple_stack
cargo run
cargo test
```

For source-only examples, compile with `rustc` using the relevant `.rs` file.
The repository intentionally preserves the original exercise layout rather
than combining the examples into one workspace.

## Recommended checks

From an exercise directory:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy -- -D warnings
```

These commands are guidance for each local exercise; availability depends on
whether that directory defines a Cargo package.

The collection now also includes focused exercises for collections, ownership, smart pointers, concurrency, file input, pattern matching, and small data structures.
