# Advent of Code 2024

My solutions to the [Advent of Code 2024](https://adventofcode.com/2024) puzzles, all written in Rust.

## Structure

Each day is its own standalone Cargo project:

```
day_1/
├── Cargo.toml
├── input.txt
└── src/
    └── main.rs
```

Some days also include a `test_input.txt` with the example from the puzzle description.

## Running a solution

You'll need [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

Each solution reads `input.txt` from the current directory, so run it from inside that day's folder:

```sh
cd day_1
cargo run --release
```

This prints the answers for part 1 and part 2.

> **Note:** Part 2 of days 24 and 25 is not completed, so those days only print the answer for part 1.

To use your own puzzle input, replace `input.txt` in that day's folder with your input.
