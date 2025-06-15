use crate::BrainfuckInterpreter;

#[test]
fn test_simple_increment() {
    let mut interpreter = BrainfuckInterpreter::new("+++.".to_string(), false).unwrap();
    interpreter.execute().unwrap();
}

#[test]
fn test_memory_movement() {
    let mut interpreter = BrainfuckInterpreter::new("+++>++>+".to_string(), false).unwrap();
    interpreter.execute().unwrap();
}

#[test]
fn test_simple_loop() {
    let mut interpreter = BrainfuckInterpreter::new("+++[>+<-]".to_string(), false).unwrap();
    interpreter.execute().unwrap();
}

#[test]
fn test_bracket_matching() {
    assert!(BrainfuckInterpreter::new("[++".to_string(), false).is_err());
    assert!(BrainfuckInterpreter::new("++]".to_string(), false).is_err());
    assert!(BrainfuckInterpreter::new("[++]".to_string(), false).is_ok());
}

#[test]
fn test_memory_underflow() {
    let mut interpreter = BrainfuckInterpreter::new("<".to_string(), false).unwrap();
    assert!(interpreter.execute().is_err());
}

#[test]
fn test_nested_loops() {
    let mut interpreter = BrainfuckInterpreter::new("++[>+[>+<-]<-]".to_string(), false).unwrap();
    interpreter.execute().unwrap();
}

#[test]
fn test_wrapping() {
    let mut interpreter = BrainfuckInterpreter::new("+".repeat(256), false).unwrap();
    interpreter.execute().unwrap();

    let mut interpreter = BrainfuckInterpreter::new("-".to_string(), false).unwrap();
    interpreter.execute().unwrap();
}

#[test]
fn test_complex_program() {
    let source = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let mut interpreter = BrainfuckInterpreter::new(source.to_string(), false).unwrap();
    interpreter.execute().unwrap();
}
