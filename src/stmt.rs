use crate::expr::Expr;
use crate::token::Token;

#[derive(Clone)]
pub enum Stmt {
    Expression(Expression),
    Print(Print),
    Var(Var),
}

#[derive(Clone)]
pub struct Expression {
    pub expression: Box<Expr>
}

#[derive(Clone)]
pub struct Print {
    pub expression: Box<Expr>
}

#[derive(Clone)]
pub struct Var {
    pub name: Token, pub initializer: Option<Box<Expr>>
}

pub trait Visitor<R> {
    fn visit_expression_expr(&mut self, expr: &Expression) -> R;
    fn visit_print_expr(&mut self, expr: &Print) -> R;
    fn visit_var_expr(&mut self, expr: &Var) -> R;
}

impl Stmt {
    pub fn accept<V: Visitor<R>, R>(&self, visitor: &mut V) -> R {
        match self {
            Stmt::Expression(expr) => visitor.visit_expression_expr(expr),
            Stmt::Print(expr) => visitor.visit_print_expr(expr),
            Stmt::Var(expr) => visitor.visit_var_expr(expr),
        }
    }
}

