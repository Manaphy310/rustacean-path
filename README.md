# Rustacean Path

A learning program for Rust beginners. Learn basic syntax in a structured, modular way step by step.

## Learning Contents

This project covers the following basic Rust concepts:

1. **Hello World** (`src/basics/hello.rs`)
   - Using the `println!` macro
   - Formatted output
   - Debug output

2. **Variables and Mutability** (`src/basics/variables.rs`)
   - Immutable and mutable variables (`mut`)
   - Shadowing
   - Constants (`const`)
   - Scope

3. **Data Types** (`src/basics/data_types.rs`)
   - Integer types, floating-point types
   - Boolean type, character type
   - Tuple type, array type
   - String slices

4. **Control Flow** (`src/basics/control_flow.rs`)
   - `if` expressions
   - `loop`, `while`, `for` loops
   - `match` expressions (pattern matching)
   - `break`, `continue`

5. **Functions** (`src/basics/functions.rs`)
   - Function definition and calls
   - Parameters and return values
   - Difference between expressions and statements

## How to Run

### Prerequisites

Rust must be installed. If not installed, use the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Running the Program

```bash
# Build and run
cargo run

# Run with release build (optimized)
cargo run --release

# Build only
cargo build
```

## Project Structure

```
rustacean-path/
├── Cargo.toml              # Project configuration file
├── README.md               # This file
└── src/
    ├── main.rs             # Entry point
    └── basics/             # Basic syntax module
        ├── mod.rs          # Module definition
        ├── hello.rs        # Hello World
        ├── variables.rs    # Variables and mutability
        ├── data_types.rs   # Data types
        ├── control_flow.rs # Control flow
        └── functions.rs    # Functions
```

## How to Learn

1. First, run `cargo run` to see the output of each section
2. Read the source code in each module (files in `src/basics/`) and understand the comments and implementation
3. Modify the code yourself and experiment with the behavior
4. Don't be afraid of error messages - try different things

## Next Steps

After mastering the basics, we recommend exploring these topics:

- **Ownership** - The most important concept in Rust
- **Structs and Enums** - Defining custom data types
- **Error Handling** - `Result` and `Option` types
- **Collections** - `Vec`, `HashMap`, `String`, etc.
- **Module System** - Structuring larger programs

## References

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Official Documentation](https://www.rust-lang.org/learn)

Happy Coding!
