use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct KeyWord {}

impl KeyWord {
    pub fn make_keywords() -> HashMap<&'static str, TokenType> {
        use TokenType::*;

        let mut keywords = HashMap::new();
        keywords.insert("and", And);
        keywords.insert("class", Class);
        keywords.insert("else", Else);
        keywords.insert("false", False);
        keywords.insert("for", For);
        keywords.insert("fun", Fun);
        keywords.insert("if", If);
        keywords.insert("nil", Nil);
        keywords.insert("or", Or);
        keywords.insert("print", Print);
        keywords.insert("return", Return);
        keywords.insert("super", Super);
        keywords.insert("this", This);
        keywords.insert("true", True);
        keywords.insert("var", Var);
        keywords.insert("while", While);

        keywords
    }

}

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
    Number,
    Identifier,

    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF
}

pub enum Literal {
    String(String),
    Number(f64),
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(
            match self {
                Literal::String(string) => {
                    write!(f, "{}", string)?;
                }
                Literal::Number(num) => {
                    if num.fract() == 0.0 {
                        write!(f, "{:.1}", num)?;
                    } else {
                        write!(f, "{}", num)?;
                    }
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
            TokenType::Number => "NUMBER",
            TokenType::Identifier => "IDENTIFIER",

            TokenType::And => "AND",
            TokenType::Class => "CLASS",
            TokenType::Else => "ELSE",
            TokenType::False => "FALSE",
            TokenType::For => "FOR",
            TokenType::Fun => "FUN",
            TokenType::If => "IF",
            TokenType::Nil => "NIL",
            TokenType::Or => "OR",
            TokenType::Print => "PRINT",
            TokenType::Return => "RETURN",
            TokenType::Super => "SUPER",
            TokenType::This => "THIS",
            TokenType::True => "TRUE",
            TokenType::Var => "VAR",
            TokenType::While => "WHILE",

            TokenType::EOF => "EOF"
        })
    }
}

