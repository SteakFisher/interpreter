use crate::expr::{Binary, Expr, Grouping, Literal, Unary, Visitor};
use crate::token_type::{LiteralValue, TokenType};

pub struct Interpreter {}

impl Visitor<Result<LiteralValue, String>> for Interpreter {
    fn visit_binary_expr(&self, expr: &Binary) -> Result<LiteralValue, String> {
        let left = self.evaluate(&expr.clone().left);
        let right = self.evaluate(&expr.clone().right);

        match left? {
            LiteralValue::String(left) => {
                match right? {
                    LiteralValue::String(right) => {
                        match expr.operator.token_type {
                            TokenType::Plus => {
                                Ok(LiteralValue::String(format!("{}{}", left, right)))
                            }
                            TokenType::EqualEqual => {
                                Ok(LiteralValue::Bool(left == right))
                            }
                            TokenType::BangEqual => {
                                Ok(LiteralValue::Bool(left != right))
                            }
                            _ => {
                                panic!("Unsupported binary operator");
                            }
                        }
                    }
                    _ => {
                        panic!("Unsupported binary operator");
                    }
                }
            }
            LiteralValue::Number(left) => {
                match right? {
                    LiteralValue::Number(right) => {
                       match expr.operator.token_type {
                           TokenType::Minus => {
                               Ok(LiteralValue::Number(left - right))
                           }
                           TokenType::Star => {
                               Ok(LiteralValue::Number(left * right))
                           }
                           TokenType::Slash => {
                               Ok(LiteralValue::Number(left / right))
                           }
                           TokenType::Plus => {
                               Ok(LiteralValue::Number(left + right))
                           }
                           TokenType::Greater => {
                               Ok(LiteralValue::Bool(left > right))
                           }
                           TokenType::GreaterEqual => {
                               Ok(LiteralValue::Bool(left >= right))
                           }
                           TokenType::Less => {
                               Ok(LiteralValue::Bool(left < right))
                           }
                           TokenType::LessEqual => {
                               Ok(LiteralValue::Bool(left <= right))
                           }
                           TokenType::EqualEqual => {
                               Ok(LiteralValue::Bool(left == right))
                           }
                           TokenType::BangEqual => {
                               Ok(LiteralValue::Bool(left != right))
                           }
                           _ => {
                               panic!("Unsupported binary operator");
                           }
                       }
                    }
                    _ => {
                        Ok(LiteralValue::Bool(false))
                    }
                }
            }
            LiteralValue::Bool(left) => {
                match right? {
                    LiteralValue::Bool(right) => {
                        match expr.operator.token_type {
                            TokenType::EqualEqual => {
                                Ok(LiteralValue::Bool(left == right))
                            }
                            TokenType::BangEqual => {
                                Ok(LiteralValue::Bool(left != right))
                            }
                            _ => {
                                panic!("Unsupported binary operator");
                            }
                        }
                    }
                    _ => {
                        Ok(LiteralValue::Bool(false))
                    }
                }
            }
            LiteralValue::Nil => {
                match right? {
                    LiteralValue::Nil => {
                        Ok(LiteralValue::Bool(true))
                    }
                    _ => {
                        Ok(LiteralValue::Bool(false))
                    }
                }
            }
        }

        // match *left {
        //     Expr::Literal(Literal { value: LiteralValue::Number(left) }) => {
        //        match *right {
        //            Expr::Literal(Literal { value: LiteralValue::Number(right) }) => {
        //                match expr.operator.token_type {
        //                    TokenType::Minus => {
        //                        LiteralValue::Number(left - right)
        //                    }
        //                    TokenType::Star => {
        //                        LiteralValue::Number(left * right)
        //                    }
        //                    TokenType::Slash => {
        //                        LiteralValue::Number(left / right)
        //                    }
        //                    TokenType::Plus => {
        //                        LiteralValue::Number(left + right)
        //                    }
        //                    TokenType::Greater => {
        //                        LiteralValue::Bool(left > right)
        //                    }
        //                    TokenType::GreaterEqual => {
        //                        LiteralValue::Bool(left >= right)
        //                    }
        //                    TokenType::Less => {
        //                        LiteralValue::Bool(left < right)
        //                    }
        //                    TokenType::LessEqual => {
        //                        LiteralValue::Bool(left <= right)
        //                    }
        //                    TokenType::EqualEqual => {
        //                        LiteralValue::Bool(left == right)
        //                    }
        //                    TokenType::BangEqual => {
        //                        LiteralValue::Bool(left != right)
        //                    }
        //                    _ => {
        //                        panic!("Unsupported binary operator");
        //                    }
        //                }
        //            }
        //            _ => {
        //                panic!("Unsupported binary operator");
        //            }
        //        }
        //    }
        //     Expr::Literal(Literal { value: LiteralValue::String(left) }) => {
        //         match *right {
        //             Expr::Literal(Literal { value: LiteralValue::String(right) }) => {
        //                 match expr.operator.token_type {
        //                     TokenType::Plus => {
        //                         LiteralValue::String(format!("{}{}", left, right))
        //                     }
        //                     TokenType::EqualEqual => {
        //                         LiteralValue::Bool(left == right)
        //                     }
        //                     TokenType::BangEqual => {
        //                         LiteralValue::Bool(left != right)
        //                     }
        //                     _ => {
        //                         panic!("Unsupported binary operator");
        //                     }
        //                 }
        //             }
        //             _ => {
        //                 panic!("Unsupported binary operator");
        //             }
        //         }
        //     }
        //     _ => {
        //         panic!("Unsupported binary operator");
        //     }
        // }
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> Result<LiteralValue, String> {
        Ok(self.evaluate(&expr.expression)?)
    }

    fn visit_literal_expr(&self, expr: &Literal) -> Result<LiteralValue, String> {
        Ok(expr.clone().value)
    }

    fn visit_unary_expr(&self, expr: &Unary) -> Result<LiteralValue, String> {
        let right = self.evaluate(&expr.right);

        match expr.operator.token_type {
            TokenType::Minus => {
                match right? {
                    LiteralValue::Number(n) => Ok(LiteralValue::Number(-n)),
                    _ => {
                        eprintln!("Tried negating a non number");
                        Err("Tried negating a non number".to_string())
                    }
                }
            }
            TokenType::Bang => {
                Ok(Interpreter::is_truthy(right?))
            }
            _ => {
                eprintln!("Tried to evaluate a non-unary operator in the unary Visitor");
                Err("Tried to evaluate a non-unary operator in the unary Visitor".to_string())
            }
        }
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn interpret(&self, expr: &Box<Expr>) -> Result<LiteralValue, String> {
        self.evaluate(expr)
    }

    fn evaluate(&self, expr: &Box<Expr>) -> Result<LiteralValue, String> {
        expr.accept(self)
    }

    fn is_truthy(val: LiteralValue) -> LiteralValue {
        match val {
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
}