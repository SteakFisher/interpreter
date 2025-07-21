use crate::expr::{Binary, Expr, Grouping, Literal, Unary, Variable, Visitor};
use std::ptr::null;

pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(&mut self, expr: Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: String, exprs: &[&Box<Expr>]) -> String {
        let mut final_string = String::new();

        final_string.push_str("(");
        final_string.push_str(&name);
        for expr in exprs {
            final_string.push_str(" ");
            final_string.push_str(&expr.accept(self));
        }
        final_string.push_str(")");
        final_string
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &Binary) -> String {
        // println!("Binary expr: {}", expr.operator);
        self.parenthesize(expr.operator.lexeme.to_string(), &[&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> String {
        self.parenthesize("group".to_string(), &[&expr.expression])
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> String {
        expr.value.to_string()
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> String {
        self.parenthesize(expr.operator.lexeme.to_string(), &[&expr.right])
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> String {
        println!("{}", expr.name);
        self.parenthesize(expr.name.lexeme.to_string(), &[])
    }
}
