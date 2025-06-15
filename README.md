# Brainfuck Interpreter in Rust

A fast and efficient Brainfuck interpreter implemented in Rust, following the specification at <https://brainfuck.org/brainfuck.html>.

## Features

- Complete implementation of all 8 Brainfuck commands
- Memory buffer starting at 30,000 cells with dynamic expansion
- Efficient bracket matching with pre-computed jump table
- Comprehensive error handling
- Command-line interface

## Usage

```bash
# Build the interpreter
cargo build --release

# Run a Brainfuck program
cargo run -- program.bf

# Run tests
cargo test
```

## Brainfuck Commands

- `+`: Increment cell value by 1
- `-`: Decrement cell value by 1
- `>`: Move pointer to next cell (right)
- `<`: Move pointer to previous cell (left)
- `[`: Begin loop (jump to matching `]` if cell is zero)
- `]`: End loop (jump back to matching `[` if cell is non-zero)
- `.`: Output cell value as byte
- `,`: Input one byte into current cell

## Examples

The repository includes several test programs:

- `hello.bf` - Classic "Hello World!" program
- `test_simple.bf` - Simple ABC output test
- `test_basic.bf` - Basic Hello World with comments

## Thanks

Test programs in this repository come from:

- `hello.bf` - Classic Brainfuck "Hello World!" program
- Additional test programs created for validation purposes

## License

This project is open source and available under standard licensing terms.
