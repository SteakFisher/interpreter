use crate::expr::Expr;

#[derive(Clone)]
pub enum Stmt {
    Expression(Expression),
    Print(Print),
}

#[derive(Clone)]
pub struct Expression {
    pub expression: Box<Expr>
}

#[derive(Clone)]
pub struct Print {
    pub expression: Box<Expr>
}

pub trait Visitor<R> {
    fn visit_expression_expr(&self, expr: &Expression) -> R;
    fn visit_print_expr(&self, expr: &Print) -> R;
}

impl Stmt {
    pub fn accept<V: Visitor<R>, R>(&self, visitor: &V) -> R {
        match self {
            Stmt::Expression(expr) => visitor.visit_expression_expr(expr),
            Stmt::Print(expr) => visitor.visit_print_expr(expr),
        }
    }
}

