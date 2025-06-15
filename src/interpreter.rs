use std::io::{self, Read};

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
    bracket_map: Vec<usize>,
}

impl BrainfuckInterpreter {
    pub fn new(source: String) -> Result<Self, BrainfuckError> {
        #[cfg(any(test, debug_assertions))]
        eprintln!("Debug: Converting source to bytes ({} chars)", source.len());
        let source_bytes = source.as_bytes().to_vec();

        #[cfg(any(test, debug_assertions))]
        eprintln!("Debug: Building bracket map");

        let bracket_map = Self::build_bracket_map(&source_bytes)?;

        #[cfg(any(test, debug_assertions))]
        {
            eprintln!("Debug: Found {} bracket pairs", bracket_map.len() / 2);
            eprintln!("Debug: Initializing memory with 30,000 cells");
        }

        Ok(BrainfuckInterpreter {
            memory: vec![0; 30000],
            data_pointer: 0,
            instruction_pointer: 0,
            source: source_bytes,
            bracket_map,
        })
    }

    fn build_bracket_map(source: &[u8]) -> Result<Vec<usize>, BrainfuckError> {
        let mut bracket_map = vec![0; source.len()];
        let mut stack = Vec::new();

        for (i, &ch) in source.iter().enumerate() {
            match ch {
                b'[' => stack.push(i),
                b']' => {
                    if let Some(start) = stack.pop() {
                        bracket_map[start] = i;
                        bracket_map[i] = start;
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
        #[cfg(any(test, debug_assertions))]
        eprintln!("Debug: Setting up buffered output");

        #[cfg(any(test, debug_assertions))]
        eprintln!(
            "Debug: Starting execution loop ({} instructions)",
            self.source.len()
        );

        let sourcelen = self.source.len();

        while self.instruction_pointer < sourcelen {
            match self.source[self.instruction_pointer] {
                b'+' => {
                    self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_add(1);
                }
                b'-' => {
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
                    self.data_pointer = self.data_pointer.saturating_sub(1);
                }
                b'[' => {
                    if self.memory[self.data_pointer] == 0 {
                        self.instruction_pointer = self.bracket_map[self.instruction_pointer];
                    }
                }
                b']' => {
                    if self.memory[self.data_pointer] != 0 {
                        self.instruction_pointer = self.bracket_map[self.instruction_pointer];
                    }
                }
                b'.' => {
                    print!("{}", self.memory[self.data_pointer] as char);
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
        #[cfg(any(test, debug_assertions))]
        eprintln!("Debug: Flushing output buffer");

        #[cfg(any(test, debug_assertions))]
        eprintln!(
            "Debug: Execution completed after {} instructions",
            self.instruction_pointer
        );

        Ok(())
    }
}
