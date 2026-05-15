# Grep-Lite
A Rust-based grep tool built as a side learning project.
---
# Day 1

## What I Built
- A basic grep-like CLI tool
- Reads file path and query from command line arguments
- Searches for matching lines inside a file
- Prints matching lines to stdout

## What I Learned
- Borrowing references using `&`
- Using lifetimes with structs
- Storing references inside structs
- Passing slices like `&[String]`
- Difference between `&String` and `&str`
- Reading files using `fs`
- Iterating over lines using `.lines()`
- Searching substrings using `.contains()`

## New Functions / Methods
### `env::args()`
Used to get command line arguments.

### `env::args().collect()`
Used to collect iterator values into a vector.

### `fs::read_to_string()`
Reads entire file into a String.

## Libraries Used

### `use std::{env, fs};`
* env → command line arguments
* fs → file handling

