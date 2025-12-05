# 🎄 Advent of Code 2025 — Rust Solutions

My personal solutions for [Advent of Code 2025](https://adventofcode.com/2025), implemented in Rust with an interactive CLI.

## ✨ Features

- **Interactive CLI** — Select day and part at runtime
- **Modular architecture** — Each day is a self-contained module
- **Automatic input loading** — Input files are loaded from `data/` directory
- **Clean abstractions** — Shared utilities for common patterns

## 📁 Project Structure

```
advent_of_code/
├── Cargo.toml
├── data/
│   ├── day01.txt      # Puzzle inputs
│   ├── day02.txt
│   └── ...
└── src/
    ├── main.rs        # CLI entry point
    └── days/
        ├── mod.rs     # Day dispatcher & input loader
        ├── day01.rs
        ├── day02.rs
        └── ...
```

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2024 edition)

### Building

```bash
# Compile the project (debug mode)
cargo build

# Compile with optimizations (release mode)
cargo build --release
```

### Running Solutions

```bash
# Build and run
cargo run

# Then follow the prompts:
# Selecione o dia (1-25): 1
# Selecione a parte (1 ou 2): 1
```

### Running in Release Mode

For better performance on computationally intensive puzzles:

```bash
cargo run --release
```

## 📅 Progress

| Day | Part 1 | Part 2 |
|:---:|:------:|:------:|
|  1  |   ⭐   |   ⭐   |
|  2  |   ⭐   |   ⭐   |
|  3  |   ⭐   |   ⭐   |
|  4  |   ⭐   |   ⭐   |
|  5  |   ⭐   |   ⭐   |
|  6  |   ·    |   ·    |
|  7  |   ·    |   ·    |
|  8  |   ·    |   ·    |
|  9  |   ·    |   ·    |
|  10  |   ·    |   ·    |
|  11  |   ·    |   ·    |
|  12  |   ·    |   ·    |

## ➕ Adding a New Day

1. **Create input file**: `data/dayXX.txt`

2. **Create solution file**: `src/days/dayXX.rs`

```rust
use crate::days;
use std::io::{self, BufRead};

pub fn part1() -> io::Result<()> {
    let reader = days::load_input_reader(XX)?;
    
    for line in reader.lines() {
        let line = line?;
        // Your solution here
    }
    
    Ok(())
}

pub fn part2() -> io::Result<()> {
    // Part 2 solution
    Ok(())
}
```

3. **Register in `src/days/mod.rs`**:
   - Add `pub mod dayXX;`
   - Add match arms in the `run()` function

## 📜 License

This project is for educational purposes. Puzzle descriptions and inputs are property of [Advent of Code](https://adventofcode.com/).

---

*Happy coding! 🎅*
