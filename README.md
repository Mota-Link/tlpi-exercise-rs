# The Linux Programming Interface Solutions in Rust

This repository contains Rust solutions for the exercises from "The Linux Programming Interface" book, organized by chapters and exercise names. Each solution is a separate binary project within a Rust workspace for easier management and compilation.

## About

"The Linux Programming Interface" by Michael Kerrisk is an authoritative guide to the Linux and UNIX system programming. This repository aims to provide practical examples and solutions to the exercises in the book, implemented in Rust.

## Repository Structure

The repository uses a workspace setup to manage multiple projects (exercise solutions) under a single Cargo workspace. Each exercise is named with a prefix indicating its chapter and a short, descriptive name.

```text
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── ch4_1_tee
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── ch4_2_cp
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── ...
```

## Building and Running Solutions

To build and run the solutions within the workspace, follow these steps:

1. Ensure you have Rust's package manager and build tool, `cargo`, installed.
2. Clone this repository to your local machine.
3. Navigate to the root directory of the repository.
4. Use `cargo build` to compile all solutions in the workspace.
5. Use `cargo run --bin <exercise-name>` to execute a specific compiled program, where `<exercise-name>` is the name of the exercise binary.
