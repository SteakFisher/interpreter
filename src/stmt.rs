use crate::expr::Expr;
use crate::token::Token;
use crate::token_type::LiteralValue;

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
    fn visit_expression_stmt(&mut self, expr: &Expression) -> R;
    fn visit_print_stmt(&mut self, expr: &Print) -> R;
    fn visit_var_stmt(&mut self, expr: &Var) -> R;
}

impl Stmt {
    pub fn accept<V: Visitor<R>, R>(&self, visitor: &mut V) -> R {
        match self {
            Stmt::Expression(stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::Print(stmt) => visitor.visit_print_stmt(stmt),
            Stmt::Var(stmt) => visitor.visit_var_stmt(stmt),
        }
    }
}

