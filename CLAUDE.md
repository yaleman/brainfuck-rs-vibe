# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

**IMPORTANT**: Any time the implementation plan changes, this file should be updated to reflect the current state and approach.

## Project Overview

This is a Rust project for implementing a Brainfuck interpreter based on the specification at <https://brainfuck.org/brainfuck.html>. The interpreter accepts a filename on the command line and executes Brainfuck programs.

## Development Commands

- **Build**: `cargo build`
- **Run with file**: `cargo run -- program.bf`
- **Test**: `cargo test`
- **Check**: `cargo check`
- **Format**: `cargo fmt`
- **Lint**: `cargo clippy --all-targets`

**IMPORTANT**: When completing tasks, always run these Development Commands in order to ensure quality:

1. **Format**: `cargo fmt`
2. **Lint**: `cargo clippy --all-targets`
3. **Test**: `cargo test`

## Implementation Plan

### Core Architecture

- **BrainfuckInterpreter** struct containing:
  - Memory buffer (`Vec<u8>`) starting at 30,000 cells
  - Data pointer (usize) for current memory position
  - Source code (`Vec<u8>`) for the program
  - Bracket map for efficient loop jumping

### Key Requirements

1. **Memory Management**: Initialize with 30,000 cells, expand by 1,000 cells when needed
2. **Command Processing**: Implement all 8 Brainfuck commands (`+`, `-`, `>`, `<`, `[`, `]`, `.`, `,`)
3. **Bracket Matching**: Pre-compute bracket pairs for O(1) loop jumps
4. **Error Handling**: File I/O errors, unmatched brackets, memory bounds
5. **I/O**: Byte-based input/output with proper EOF handling
6. **CLI**: Accept filename as command line argument

### Brainfuck Commands

- `+`: Increment cell value by 1
- `-`: Decrement cell value by 1
- `>`: Move pointer to next cell (right)
- `<`: Move pointer to previous cell (left)
- `[`: Begin loop (jump to matching `]` if cell is zero)
- `]`: End loop (jump back to matching `[` if cell is non-zero)
- `.`: Output cell value as byte
- `,`: Input one byte into current cell

### Project Structure

- `src/main.rs` - CLI argument parsing and program entry
- `src/interpreter.rs` - Core BrainfuckInterpreter implementation
- `src/lib.rs` - Library exports and common functionality
