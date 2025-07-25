use crate::environment::Environment;
use crate::expr::{Assign, Binary, Expr, Grouping, Literal, Unary, Variable, Visitor as ExprVisitor};
use crate::stmt::{Expression, Print, Stmt, Var, Visitor as StmtVisitor};
use crate::token_type::{LiteralValue, TokenType};
use crate::util::Utils;

pub struct Interpreter {
    environment: Environment
}

impl ExprVisitor<Result<LiteralValue, String>> for Interpreter {
    fn visit_assign_expr(&mut self, expr: &Assign) -> Result<LiteralValue, String> {
        let value = self.evaluate(&expr.value)?;
        self.environment.assign(expr.clone().name, value.clone())?;

        Ok(value)
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> Result<LiteralValue, String> {
        let left = self.evaluate(&expr.clone().left);
        let right = self.evaluate(&expr.clone().right);

        match left? {
            LiteralValue::String(left) => match right? {
                LiteralValue::String(right) => match expr.operator.token_type {
                    TokenType::Plus => Ok(LiteralValue::String(format!("{}{}", left, right))),
                    TokenType::EqualEqual => Ok(LiteralValue::Bool(left == right)),
                    TokenType::BangEqual => Ok(LiteralValue::Bool(left != right)),
                    _ => Err("Invalid operator".to_string()),
                },
                LiteralValue::Number(left) => Interpreter::str_int_equality(expr),
                _ => Err("Invalid operator".to_string()),
            },
            LiteralValue::Number(left) => match right? {
                LiteralValue::Number(right) => match expr.operator.token_type {
                    TokenType::Minus => Ok(LiteralValue::Number(left - right)),
                    TokenType::Star => Ok(LiteralValue::Number(left * right)),
                    TokenType::Slash => Ok(LiteralValue::Number(left / right)),
                    TokenType::Plus => Ok(LiteralValue::Number(left + right)),
                    TokenType::Greater => Ok(LiteralValue::Bool(left > right)),
                    TokenType::GreaterEqual => Ok(LiteralValue::Bool(left >= right)),
                    TokenType::Less => Ok(LiteralValue::Bool(left < right)),
                    TokenType::LessEqual => Ok(LiteralValue::Bool(left <= right)),
                    TokenType::EqualEqual => Ok(LiteralValue::Bool(left == right)),
                    TokenType::BangEqual => Ok(LiteralValue::Bool(left != right)),
                    _ => Err("Invalid operator".to_string()),
                },
                LiteralValue::String(left) => Interpreter::str_int_equality(expr),
                _ => Err("Invalid operator".to_string()),
            },
            LiteralValue::Bool(left) => match right? {
                LiteralValue::Bool(right) => match expr.operator.token_type {
                    TokenType::EqualEqual => Ok(LiteralValue::Bool(left == right)),
                    TokenType::BangEqual => Ok(LiteralValue::Bool(left != right)),
                    _ => Err("Invalid operator".to_string()),
                },
                _ => Err("Invalid operator".to_string()),
            },
            LiteralValue::Nil => match right? {
                LiteralValue::Nil => Ok(LiteralValue::Bool(true)),
                _ => Ok(LiteralValue::Bool(false)),
            },
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<LiteralValue, String> {
        Ok(self.evaluate(&expr.expression)?)
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<LiteralValue, String> {
        Ok(expr.clone().value)
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> Result<LiteralValue, String> {
        let right = self.evaluate(&expr.right);

        match expr.operator.token_type {
            TokenType::Minus => match right? {
                LiteralValue::Number(n) => Ok(LiteralValue::Number(-n)),
                _ => {
                    eprintln!("Tried negating a non number");
                    Err("Tried negating a non number".to_string())
                }
            },
            TokenType::Bang => Ok(Interpreter::is_truthy(right?)),
            _ => {
                eprintln!("Tried to evaluate a non-unary operator in the unary Visitor");
                Err("Tried to evaluate a non-unary operator in the unary Visitor".to_string())
            }
        }
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> Result<LiteralValue, String> {
        self.environment.get(expr.clone().name)
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &Expression) {
        self.evaluate(&stmt.expression).unwrap_or_else(|_| {
            eprintln!("Tried executing an expression which is not an expression");
            std::process::exit(70)
        });
    }

    fn visit_print_stmt(&mut self, stmt: &Print) {
        let val = Utils::print_literal(&self.evaluate(&stmt.expression).unwrap_or_else(|_| {
            eprintln!("Tried executing an expression which is not an expression");
            std::process::exit(70)
        }));

        println!("{}", val);
    }

    fn visit_var_stmt(&mut self, stmt: &Var) -> () {
        let mut val = LiteralValue::Nil;

        if let Some(expr) = stmt.clone().initializer {
            val = self.evaluate(&Box::from(expr)).unwrap_or_else(|_| {
                eprintln!("Tried executing an expression which is not an expression");
                std::process::exit(70)
            });
        }

        self.environment.define(stmt.name.lexeme.clone(), val);
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new()
        }
    }

    pub fn interpret(&mut self, stmts: &Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            self.execute(stmt);
        }
        Ok(())
    }

    pub fn interpret_expression(&mut self, expr: &Box<Expr>) -> Result<LiteralValue, String> {
        self.evaluate(expr)
    }

    fn execute(&mut self, stmt: &Stmt)  {
        stmt.accept(self)
    }

    fn evaluate(&mut self, expr: &Box<Expr>) -> Result<LiteralValue, String> {
        expr.accept(self)
    }

    fn str_int_equality(expr: &Binary) -> Result<LiteralValue, String> {
        match expr.operator.token_type {
            TokenType::EqualEqual => Ok(LiteralValue::Bool(false)),
            TokenType::BangEqual => Ok(LiteralValue::Bool(true)),
            _ => Err("Invalid operator".to_string()),
        }
    }

    fn is_truthy(val: LiteralValue) -> LiteralValue {
        match val {
            LiteralValue::Number(n) => {
                if n == 0.0 {
                    LiteralValue::Bool(true)
                } else {
                    LiteralValue::Bool(false)
                }
            }
            LiteralValue::Bool(b) => LiteralValue::Bool(!b),
            LiteralValue::Nil => LiteralValue::Bool(true),
            LiteralValue::String(_) => {
                panic!("Tried banging a string lmao")
            }
        }
    }
}
