use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub enum TokenType {
    LeftParan,
    RightParan,
    LeftBrace,
    RightBrace,

    EOF
}

#[derive(Debug)]
pub struct Literal {
    smth: String,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TokenType::LeftParan => "LEFT_PAREN",
            TokenType::RightParan => "RIGHT_PAREN",
            TokenType::LeftBrace => "LEFT_BRACE",
            TokenType::RightBrace => "RIGHT_BRACE",
            TokenType::EOF => "EOF"
        })
    }
}

