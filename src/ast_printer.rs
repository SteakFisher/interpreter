use std::ptr::null;
use crate::expr::{Binary, Expr, Grouping, Literal, Unary, Visitor};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(&self, expr: Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: String, exprs: &[&Box<Expr>]) -> String {
        let mut final_string = String::new();

        final_string.push_str("(");
        final_string.push_str(&name);
        for expr in exprs {
            final_string.push_str(" ");
            final_string.push_str(&expr.accept(self));
            final_string.push_str(" ");
        }
        final_string.push_str(")");
        final_string
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &Binary) -> String {
        // println!("Binary expr: {}", expr.operator);
        self.parenthesize(expr.operator.lexeme.to_string(), &[&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> String {
        self.parenthesize("group".to_string(), &[&expr.expression])
    }

    fn visit_literal_expr(&self, expr: &Literal) -> String {
        expr.value.to_string()
    }

    fn visit_unary_expr(&self, expr: &Unary) -> String {
        self.parenthesize(expr.operator.lexeme.to_string(), &[&expr.right])
    }
}