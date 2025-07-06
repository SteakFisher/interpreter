use crate::expr::{Binary, Expr, Grouping, Literal, Unary, Visitor};
use crate::token_type::{LiteralValue, TokenType};

pub struct Interpreter {}

impl Visitor<LiteralValue> for Interpreter {
    fn visit_binary_expr(&self, expr: &Binary) -> LiteralValue {
        todo!()
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> LiteralValue {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&self, expr: &Literal) -> LiteralValue {
        expr.clone().value
    }

    fn visit_unary_expr(&self, expr: &Unary) -> LiteralValue {
        let right = self.evaluate(&expr.right);

        match expr.operator.token_type {
            TokenType::Minus => {
                match right {
                    LiteralValue::Number(n) => LiteralValue::Number(-n),
                    _ => panic!("Tried negating a non number")
                }
            }
            TokenType::Bang => {
                match right {
                    LiteralValue::Number(n) => {
                        if n == 0.0 {
                            LiteralValue::Bool(true)
                        } else {
                            LiteralValue::Bool(false)
                        }
                    },
                    LiteralValue::Bool(b) => LiteralValue::Bool(!b),
                    LiteralValue::Nil => { LiteralValue::Bool(true) }
                    LiteralValue::String(_) => {
                        panic!("Tried banging a string lmao")
                    }
                }
            }
            _ => {
                panic!("Tried to evaluate a non-unary operator in the unary Visitor");
            }
        }
    }
}

impl Interpreter {
    pub fn interpret(&self, expr: &Box<Expr>) -> LiteralValue {
        self.evaluate(expr)
    }

    fn evaluate(&self, expr: &Box<Expr>) -> LiteralValue {
        expr.accept(self)
    }
}