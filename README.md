# Advent of Code 2021

Giving [Advent of Code 2021 🎄](https://adventofcode.com/2021) a go to with [Rust 🦀](https://www.rust-lang.org/learn/get-started). 

## Cargo

[Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) is Rust’s build system and package manager.

### Building and Running a Cargo Project
Run `cargo build` to compile the project.
The executable will be created in `target/debug/advent-of-code-2021`.

To compile and execute simply run `cargo run`.

### Building for Release
Run `cargo build --release` to compile the project with optimizations. 
The executable will be created in `target/release/advent-of-code-2021`.

## Benchmarking
Rust bench is unstable & seems the community uses [Criterion.rs](https://github.com/bheisler/criterion.rs).
### Benchmarking the release executable
[hyperfine](https://github.com/sharkdp/hyperfine) can be used to benchmark arbitrary command line tool:

`hyperfine './target/release/advent-of-code-2021'`