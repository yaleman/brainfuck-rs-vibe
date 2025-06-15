default:
    just --list

sample:
    cargo build
    samply record ./target/debug/brainfuck-rs-vibe mandelbrot.bf

check:
    cargo clippy --all-targets
    cargo test
    cargo fmt --check