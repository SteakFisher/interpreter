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

    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    String,

    EOF
}

pub enum Literal {
    String(String),
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(
            match self {
                Literal::String(string) => {
                    write!(f, "{}", string)?;
                }
            }
        )
    }
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

            TokenType::Equal => "EQUAL",
            TokenType::EqualEqual => "EQUAL_EQUAL",
            TokenType::Bang => "BANG",
            TokenType::BangEqual => "BANG_EQUAL",
            TokenType::Less => "LESS",
            TokenType::LessEqual => "LESS_EQUAL",
            TokenType::Greater => "GREATER",
            TokenType::GreaterEqual => "GREATER_EQUAL",

            TokenType::String => "STRING",

            TokenType::EOF => "EOF"
        })
    }
}

