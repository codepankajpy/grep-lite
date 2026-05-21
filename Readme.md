# Grep-Lite

A lightweight grep-like CLI tool written in Rust as a systems programming learning project.

---

## Table of contents

- Features
- Example
- Project structure
- What I learned
- Crates used
- Installation

---

## Features

- Search for matching lines inside files
- Case-insensitive search
- Line-numbered output
- Highlight matched query in terminal output
- Simple CLI interface
- Modular project structure

---

## Example

Command:

cargo run -- test.txt Rust

Output:

1 : Rust gives memory safety without garbage collection.
2 : The cargo tool manages Rust projects and dependencies.

---

## Project structure

src/
├── main.rs
├── search.rs
├── print.rs
└── config.rs

---

## What I learned

### Rust concepts

- Ownership and borrowing
- Lifetimes with structs
- Borrowed data inside structs
- &str vs &String
- Passing slices using &[String]
- Modularization using separate files/modules
- Iterators and closures
- Working with Vec<T>
- String manipulation and formatting

### File handling

- Reading files using fs::read_to_string
- Iterating using .lines()

### Searching logic

- Using .contains()
- Case-insensitive matching using .to_lowercase()
- Filtering iterators with .filter()

### CLI development

- Reading command line arguments with env::args()
- Building configuration structs from arguments

### Terminal output

- Colored terminal output using the colored crate
- Highlighting matched substrings

---

## Crates used

- colored = "3" — used for colored terminal output

---

## Installation

Run locally:

cargo run -- test.txt Rust

Build release binary:

cargo build --release

Install globally:

cargo install --path .

Then run:

grep-lite test.txt Rust