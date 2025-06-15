use std::fs;
use std::process;

use brainfuck_rs_vibe::BrainfuckInterpreter;
use clap::Parser;

#[derive(Parser)]
#[command(name = "brainfuck-rs-vibe")]
#[command(about = "A Brainfuck interpreter written in Rust")]
struct Args {
    /// The Brainfuck source file to execute
    file: String,

    /// Ignore input commands (,) - they will be skipped
    #[arg(long)]
    ignore_input: bool,
}

fn main() {
    let args = Args::parse();

    #[cfg(any(test, debug_assertions))]
    eprintln!("Debug: Reading source file '{}'", args.file);

    let source = match fs::read_to_string(&args.file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", args.file, e);
            process::exit(1);
        }
    };

    #[cfg(any(test, debug_assertions))]
    {
        eprintln!("Debug: Read {} characters from file", source.len());
        eprintln!("Debug: Creating interpreter");
    }

    let mut interpreter = match BrainfuckInterpreter::new(source) {
        Ok(interpreter) => interpreter,
        Err(e) => {
            eprintln!("Error creating interpreter: {}", e);
            process::exit(1);
        }
    };

    #[cfg(any(test, debug_assertions))]
    eprintln!("Debug: Starting execution");

    if let Err(e) = interpreter.execute() {
        eprintln!("Runtime error: {}", e);
        process::exit(1);
    }

    #[cfg(any(test, debug_assertions))]
    eprintln!("Debug: Execution completed successfully");
}
