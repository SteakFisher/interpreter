use crate::expr::{Binary, Expr, Grouping, Literal, Unary, Variable};
use crate::stmt::{Expression, Print, Stmt, Var};
use crate::token::Token;
use crate::token_type::{LiteralValue, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts: Vec<Stmt> = Vec::new();

        while !self.is_at_end() {
            stmts.push(self.declaration()?);
        }

        Ok(stmts)
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.compare(&[TokenType::Var]) {
            return Ok(self.var_declaration()?);
        }

        Ok(self.statement()?)
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(TokenType::Identifier, "Expect variable name")?.clone();

        let mut initializer = Expr::Literal(Literal { value: LiteralValue::Nil });
        if self.compare(&[TokenType::Equal]) {
            initializer = self.expression()?;
        }

        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration")?;
        Ok(Stmt::Var(Var { name, initializer: Option::from(Box::new(initializer)) }))
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.compare(&[TokenType::Print]) {
            return self.print_statement()
        }

        self.expression_statement()
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';'");
        Ok(Stmt::Expression(Expression { expression: Box::new(expr) }))
    }

    fn print_statement(&mut self) -> Result<Stmt, String> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';'");

        Ok(Stmt::Print(Print {
            expression: Box::new(value),
        }))
    }

    pub fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.compare(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.compare(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.compare(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.compare(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.compare(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(Unary {
                operator,
                right: Box::new(right),
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.compare(&[TokenType::False]) {
            return Ok(Expr::Literal(Literal {
                value: LiteralValue::Bool(false),
            }))
        }
        if self.compare(&[TokenType::True]) {
            return Ok(Expr::Literal(Literal {
                value: LiteralValue::Bool(true),
            }))
        }
        if self.compare(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Literal {
                value: LiteralValue::Nil,
            }))
        }

        if self.compare(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(Literal {
                value: self.previous().literal.clone().unwrap(),
            }))
        }

        if self.compare(&[TokenType::LeftParan]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParan, "Expect ')' after expression.");
            return Ok(Expr::Grouping(Grouping {
                expression: Box::new(expr),
            }))
        }

        if self.compare(&[TokenType::Identifier]) {
            // println!("{} {}", self.previous(), self.peek());
            return Ok(Expr::Variable(Variable { name: self.previous().clone() }))
        }

        Err(self.error(self.peek(), "Expect expression.").to_string())
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, String> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek(), message))
    }

    fn error(&self, token: &Token, message: &str) -> String {
        if token.token_type == TokenType::EOF {
            format!("[line {}] Error at end: {}", token.line, message)
        } else {
            format!(
                "[line {}] Error at '{}': {}",
                token.line, token.lexeme, message
            )
        }
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
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == *token
        }
    }

    fn is_at_end(&self) -> bool {
        if self.peek().token_type == TokenType::EOF {
            true
        } else {
            false
        }
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1
        }
        self.previous()
    }

}
