use std::collections::HashMap;
use std::io::{self, BufWriter, Read, Write};

#[derive(Debug)]
pub enum BrainfuckError {
    InvalidCommand(char),
    UnmatchedBracket(usize),
    MemoryUnderflow,
    IoError(io::Error),
}

impl std::fmt::Display for BrainfuckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BrainfuckError::InvalidCommand(c) => write!(f, "Invalid command: {}", c),
            BrainfuckError::UnmatchedBracket(pos) => {
                write!(f, "Unmatched bracket at position {}", pos)
            }
            BrainfuckError::MemoryUnderflow => write!(f, "Memory pointer underflow"),
            BrainfuckError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for BrainfuckError {}

pub struct BrainfuckInterpreter {
    memory: Vec<u8>,
    data_pointer: usize,
    instruction_pointer: usize,
    source: Vec<u8>,
    bracket_map: HashMap<usize, usize>,
    debug: bool,
}

impl BrainfuckInterpreter {
    pub fn new(source: String, debug: bool) -> Result<Self, BrainfuckError> {
        if debug {
            eprintln!("Debug: Converting source to bytes ({} chars)", source.len());
        }
        let source_bytes = source.as_bytes().to_vec();

        if debug {
            eprintln!("Debug: Building bracket map");
        }
        let bracket_map = Self::build_bracket_map(&source_bytes)?;

        if debug {
            eprintln!("Debug: Found {} bracket pairs", bracket_map.len() / 2);
            eprintln!("Debug: Initializing memory with 30,000 cells");
        }

        Ok(BrainfuckInterpreter {
            memory: vec![0; 30000],
            data_pointer: 0,
            instruction_pointer: 0,
            source: source_bytes,
            bracket_map,
            debug,
        })
    }

    fn build_bracket_map(source: &[u8]) -> Result<HashMap<usize, usize>, BrainfuckError> {
        let mut bracket_map = HashMap::new();
        let mut stack = Vec::new();

        for (i, &ch) in source.iter().enumerate() {
            match ch {
                b'[' => stack.push(i),
                b']' => {
                    if let Some(start) = stack.pop() {
                        bracket_map.insert(start, i);
                        bracket_map.insert(i, start);
                    } else {
                        return Err(BrainfuckError::UnmatchedBracket(i));
                    }
                }
                _ => {}
            }
        }

        if !stack.is_empty() {
            return Err(BrainfuckError::UnmatchedBracket(stack[0]));
        }

        Ok(bracket_map)
    }

    fn ensure_memory_capacity(&mut self) {
        if self.data_pointer >= self.memory.len() {
            self.memory.resize(self.memory.len() + 1000, 0);
        }
    }

    pub fn execute(&mut self) -> Result<(), BrainfuckError> {
        if self.debug {
            eprintln!("Debug: Setting up buffered output");
        }
        let mut output = BufWriter::new(io::stdout());

        if self.debug {
            eprintln!(
                "Debug: Starting execution loop ({} instructions)",
                self.source.len()
            );
        }

        while self.instruction_pointer < self.source.len() {
            let command = self.source[self.instruction_pointer];

            match command {
                b'+' => {
                    self.ensure_memory_capacity();
                    self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_add(1);
                }
                b'-' => {
                    self.ensure_memory_capacity();
                    self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_sub(1);
                }
                b'>' => {
                    self.data_pointer += 1;
                    self.ensure_memory_capacity();
                }
                b'<' => {
                    if self.data_pointer == 0 {
                        return Err(BrainfuckError::MemoryUnderflow);
                    }
                    self.data_pointer -= 1;
                }
                b'[' => {
                    if self.memory[self.data_pointer] == 0 {
                        self.instruction_pointer = self.bracket_map[&self.instruction_pointer];
                    }
                }
                b']' => {
                    if self.memory[self.data_pointer] != 0 {
                        self.instruction_pointer = self.bracket_map[&self.instruction_pointer];
                    }
                }
                b'.' => {
                    write!(output, "{}", self.memory[self.data_pointer] as char)
                        .map_err(BrainfuckError::IoError)?;
                    output.flush().map_err(BrainfuckError::IoError)?;
                }
                b',' => {
                    let mut buffer = [0; 1];

                    match io::stdin().read_exact(&mut buffer) {
                        Ok(_) => self.memory[self.data_pointer] = buffer[0],
                        Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => {
                            // EOF - leave cell unchanged as per spec
                        }
                        Err(err) => return Err(BrainfuckError::IoError(err)),
                    }
                }
                _ => {
                    // Ignore non-command characters
                }
            }

            self.instruction_pointer += 1;
        }

        if self.debug {
            eprintln!("Debug: Flushing output buffer");
        }
        output.flush().map_err(BrainfuckError::IoError)?;

        if self.debug {
            eprintln!(
                "Debug: Execution completed after {} instructions",
                self.instruction_pointer
            );
        }
        Ok(())
    }
}
