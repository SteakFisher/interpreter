use std::collections::HashMap;
use crate::token::Token;
use crate::token_type::LiteralValue;

pub struct Environment {
    variables: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.variables.insert(name, value);
    }

    pub fn assign(&mut self, name: Token, value: LiteralValue) -> Result<(), String> {
        if self.variables.contains_key(&name.lexeme) {
            self.variables.insert(name.lexeme, value);
            return Ok(())
        }

        return Err(format!("Variable {} not found", name.lexeme));
    }

    pub fn get(&mut self, name: Token) -> Result<LiteralValue, String> {
        if let Some(value) = self.variables.get(&name.lexeme) {
            Ok(value.clone())
        } else {
            Err(format!("Undefined variables '{}'.", name.lexeme))
        }
    }
}