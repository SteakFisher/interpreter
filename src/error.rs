pub struct LoxError {}

impl LoxError {
    pub fn unexpected_character(line: usize, c: char) {
        eprintln!("[line {}] Error: Unexpected character: {}", line, c)
    }

    pub fn unterminated_string(line: usize) {
        eprintln!("[line {}] Error: Unterminated string.", line)
    }
}
