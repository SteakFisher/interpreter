use crate::expr::{Binary, Expr, Grouping, Literal, Unary, Visitor};
use crate::token_type::LiteralValue;

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
        todo!()
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