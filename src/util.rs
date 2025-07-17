use crate::token_type::LiteralValue;

pub struct Utils {}

impl Utils {
    pub fn print_literal(literal: &LiteralValue) -> String {
        match literal {
            LiteralValue::Number(num) => {
                format!("{}", num)
            }
            val => val.to_string(), 
        }
    }
}