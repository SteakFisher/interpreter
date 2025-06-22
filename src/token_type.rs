use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub enum TokenType {
    LeftParan,
    RightParan,
    LeftBrace,
    RightBrace,

    Plus,
    Minus,
    Star,
    Slash,

    Comma,
    Dot,
    Semicolon,

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
            TokenType::Plus => "PLUS",
            TokenType::Minus => "MINUS",
            TokenType::Star => "STAR",
            TokenType::Slash => "SLASH",
            TokenType::Comma => "COMMA",
            TokenType::Dot => "DOT",
            TokenType::Semicolon => "SEMICOLON",
            TokenType::EOF => "EOF"
        })
    }
}

