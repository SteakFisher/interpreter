use std::fmt::Display;
use crate::token::Token;
use crate::token_type::{Literal, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.current);
        self.current += 1;
        c
    }

    fn add_token(&mut self, token: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start..self.current].to_owned();
        self.tokens.push(Token::new(token, text, literal, self.line));
    }

    fn scan_token(&mut self) {
        let c = match self.advance() {
            Some(c) => c,
            None => {
                println!("SCAN_TOKEN OUT OF BOUNDS");
                return
            },
        };

        // eprintln!("SCAN_TOKEN GOT {:?}", c);

        match c {
            '(' => self.add_token(TokenType::LeftParan, None),
            ')' => self.add_token(TokenType::RightParan, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),

            '+' => self.add_token(TokenType::Plus, None),
            '-' => self.add_token(TokenType::Minus, None),
            // '/' => self.add_token(TokenType::Slash, None),
            '*' => self.add_token(TokenType::Star, None),

            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            ';' => self.add_token(TokenType::Semicolon, None),

            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => {}, // Ignore whitespace
            _ => {
                // Handle unexpected characters
                println!("[line {}] Error: Unexpected character: {}", self.line, c);
            }
        }
    }

    pub fn scan_tokens(&mut self) {
        while(!self.is_at_end()) {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, "".to_string(), None, self.line));
    }
}

impl Display for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for token in self.tokens.iter() {
            let literal_str = match &token.literal {
                Some(literal) => format!("{:?}", literal),
                None => "null".to_string(),
            };

            writeln!(f, "{} {} {}", token.token_type, token.lexeme, literal_str)?;
        })
    }
}