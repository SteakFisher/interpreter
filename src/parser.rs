use crate::error::LoxError;
use crate::expr::{Binary, Expr, Grouping, Literal, Unary};
use crate::token::Token;
use crate::token_type::{LiteralValue, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    has_error: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,

            has_error: false
        }
    }

    pub fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.compare(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.compare(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term();

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.compare(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.compare(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary();

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.compare(&[TokenType::Bang, TokenType::Minus])  {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary(Unary {
                operator,
                right: Box::new(right),
            })
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.compare(&[TokenType::False]) { return Expr::Literal(Literal { value: LiteralValue::Bool(false) }) }
        if self.compare(&[TokenType::True]) { return Expr::Literal(Literal { value: LiteralValue::Bool(true) }) }
        if self.compare(&[TokenType::Nil]) { return Expr::Literal(Literal { value: LiteralValue::Nil }) }

        if self.compare(&[TokenType::Number, TokenType::String]) {
            return Expr::Literal(Literal { value: self.previous().literal.clone().unwrap() })
        }

        if self.compare(&[TokenType::LeftParan]) {
            let expr = self.expression();
            self.consume(TokenType::RightParan, "Expect ')' after expression.");
            Expr::Grouping(Grouping { expression: Box::new(expr) })
        } else {
            panic!("Recursive descent not exhaustive")
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.check(&token_type) {
            self.advance();
            return;
        }

        LoxError::syntax_error(1);
        self.has_error = true;
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn compare(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token: &TokenType) -> bool {
        if self.is_at_end() { false }
        else { self.peek().token_type == *token }
    }

    fn is_at_end(&self) -> bool {
        if self.peek().token_type == TokenType::EOF { true }
        else { false }
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() { self.current += 1 }
        self.previous()
    }

    pub fn has_error(&self) -> bool {
        self.has_error
    }
}