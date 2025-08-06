use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use crate::token::Token;
use crate::token_type::LiteralValue;

pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    variables: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn global() -> Environment {
        Environment {
            enclosing: None,
            variables: HashMap::new(),
        }
    }

    pub fn local(enclosing: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            enclosing: Some(enclosing),
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.variables.insert(name, value);
    }

    pub fn assign(&mut self, name: Token, value: LiteralValue) -> Result<(), String> {
        if self.variables.contains_key(&name.lexeme) {
            self.variables.insert(name.lexeme, value);
            Ok(())
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(format!("Variable '{}' not found.", name.lexeme))
        }
    }

    pub fn get(&self, name: Token) -> Result<LiteralValue, String> {
        if let Some(value) = self.variables.get(&name.lexeme) {
            Ok(value.clone())
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.borrow().get(name)
        } else {
            Err(format!("Undefined variable '{}'.", name.lexeme))
        }
    }
}