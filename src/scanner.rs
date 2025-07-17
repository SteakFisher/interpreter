use crate::error::LoxError;
use crate::token::Token;
use crate::token_type::{KeyWord, LiteralValue, TokenType};
use std::collections::HashMap;
use std::fmt::Display;
use crate::util::Utils;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,

    has_error: bool,

    keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            has_error: false,

            keywords: KeyWord::make_keywords(),
        }
    }

    pub fn get_tokens(self) -> Vec<Token> {
        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.char_indices().count()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.char_indices().nth(self.current);
        self.current += 1;
        match c {
            None => {
                None
            }
            Some(val) => {
                Some(val.1)
            }
        }
    }

    fn add_token(&mut self, token: TokenType, literal: Option<LiteralValue>) {
        let text = Utils::get_char_range(&self.source, self.start, self.current);
        self.tokens
            .push(Token::new(token, text, literal, self.line));
    }

    fn match_next(&mut self, expected: &char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != *expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return Some('\0');
        }
        Some(self.source.char_indices().nth(self.current).unwrap().1)
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 > self.source.len() {
            return Some('\0');
        }
        self.source.chars().nth(self.current + 1)
    }

    fn string(&mut self) {
        while self.peek().unwrap() != '"' && !self.is_at_end() {
            self.advance();
        }

        if self.is_at_end() {
            LoxError::unterminated_string(self.line);
            self.has_error = true;
            return;
        }

        self.advance();

        let value = Utils::get_char_range(&self.source, self.start + 1, self.current - 1);
        self.add_token(TokenType::String, Option::from(LiteralValue::String(value)));
    }

    fn number(&mut self) {
        while self.peek().unwrap().is_ascii_digit() {
            self.advance();
        }

        if self.peek().unwrap() == '.' && self.peek_next().unwrap().is_ascii_digit() {
            self.advance();

            while self.peek().unwrap().is_ascii_digit() {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current]
            .to_owned()
            .parse::<f64>()
            .unwrap();

        self.add_token(TokenType::Number, Some(LiteralValue::Number(value)));
    }

    fn identifier(&mut self) {
        while (self.peek().unwrap().is_ascii_alphanumeric() || (self.peek().unwrap() == '_'))
            && !self.is_at_end()
        {
            self.advance();
        }

        let value = self.source[self.start..self.current].to_owned();
        let token_type = self.keywords.get(value.as_str()).cloned();

        match token_type {
            Some(token_type) => {
                self.add_token(token_type, None);
            }
            None => {
                self.add_token(TokenType::Identifier, None);
            }
        }
    }

    fn scan_token(&mut self) {
        let c = match self.advance() {
            Some(c) => c,
            None => {
                println!("SCAN_TOKEN OUT OF BOUNDS");

                return;
            }
        };

        match c {
            '(' => self.add_token(TokenType::LeftParan, None),
            ')' => self.add_token(TokenType::RightParan, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),

            '+' => self.add_token(TokenType::Plus, None),
            '-' => self.add_token(TokenType::Minus, None),
            '*' => self.add_token(TokenType::Star, None),

            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            ';' => self.add_token(TokenType::Semicolon, None),

            '=' => {
                let is_equal = if self.match_next(&'=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(is_equal, None)
            }
            '!' => {
                let is_bang = if self.match_next(&'=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(is_bang, None)
            }
            '<' => {
                let is_less = if self.match_next(&'=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(is_less, None)
            }
            '>' => {
                let is_greater = if self.match_next(&'=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(is_greater, None)
            }

            '/' => {
                let is_slash = if self.match_next(&'/') {
                    while self.peek().unwrap_or('\0') != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    return;
                } else {
                    TokenType::Slash
                };
                self.add_token(is_slash, None)
            }

            '"' => self.string(),

            '0'..='9' => self.number(),

            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),

            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => {} // Ignore whitespace
            _ => {
                // Handle unexpected characters
                LoxError::unexpected_character(self.line, c);
                self.has_error = true;
            }
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), None, self.line));
    }

    pub fn has_error(&self) -> bool {
        self.has_error
    }
}

impl Display for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for token in self.tokens.iter() {
            let literal_str = match &token.literal {
                Some(literal) => format!("{}", literal),
                None => "null".to_string(),
            };

            writeln!(f, "{} {} {}", token.token_type, token.lexeme, literal_str)?;
        })
    }
}
