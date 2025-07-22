use crate::token::Token;
use crate::token_type::LiteralValue;

#[derive(Clone)]
pub enum Expr {
    Assign(Assign),
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Variable(Variable),
}

#[derive(Clone)]
pub struct Assign {
    pub name: Token, pub value: Box<Expr>,
}

#[derive(Clone)]
pub struct Binary {
    pub left: Box<Expr>, pub operator: Token, pub right: Box<Expr>,
}

#[derive(Clone)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

#[derive(Clone)]
pub struct Literal {
    pub value: LiteralValue,
}

#[derive(Clone)]
pub struct Unary {
    pub operator: Token, pub right: Box<Expr>,
}

#[derive(Clone)]
pub struct Variable {
    pub name: Token,
}

pub trait Visitor<R> {
    fn visit_assign_expr(&mut self, expr: &Assign) -> R;
    fn visit_binary_expr(&mut self, expr: &Binary) -> R;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> R;
    fn visit_literal_expr(&mut self, expr: &Literal) -> R;
    fn visit_unary_expr(&mut self, expr: &Unary) -> R;
    fn visit_variable_expr(&mut self, expr: &Variable) -> R;
}

impl Expr {
    pub fn accept<V: Visitor<R>, R>(&self, visitor: &mut V) -> R {
        match self {
            Expr::Assign(expr) => visitor.visit_assign_expr(expr),
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
            Expr::Variable(expr) => visitor.visit_variable_expr(expr),
        }
    }
}

