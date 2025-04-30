use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::Type;

pub mod environment;
pub use environment::*;


#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    Int(i64),
    Bool(bool),
    Str(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableInfo {
    pub type_: Type,
    pub value: RuntimeValue,
    pub moved: bool,
    pub mutable: bool,
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub variables: HashMap<String, VariableInfo>,
    pub procedures: HashMap<String, ()>, // ajuste depois
    pub parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            variables: HashMap::new(),
            procedures: HashMap::new(),
            parent: None,
        }))
    }
    pub fn declare_variable(
        &mut self,
        name: String,
        type_: Type,
        value: RuntimeValue,
        mutable: bool,
    ) -> Result<(), String> {
        if self.variables.contains_key(&name) {
            return Err(format!("Variável '{}' já foi declarada", name));
        }
        self.variables.insert(
            name,
            VariableInfo {
                type_,
                value,
                moved: false,
                mutable,
            },
        );
        Ok(())
    }

    pub fn move_variable(&mut self, name: &str) -> Result<RuntimeValue, String> {
        if let Some(info) = self.variables.get_mut(name) {
            if info.moved {
                return Err(format!("Variável '{}' já foi movida", name));
            }
            info.moved = true;
            Ok(info.value.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().move_variable(name)
        } else {
            Err(format!("Variável '{}' não encontrada", name))
        }
    }

    pub fn get_variable(&self, name: &str) -> Result<RuntimeValue, String> {
        if let Some(info) = self.variables.get(name) {
            if info.moved {
                Err(format!("Variável '{}' foi movida", name))
            } else {
                Ok(info.value.clone())
            }
        } else if let Some(parent) = &self.parent {
            parent.borrow().get_variable(name)
        } else {
            Err(format!("Variável '{}' não encontrada", name))
        }
    }

    pub fn set_variable(&mut self, name: &str, new_value: RuntimeValue) -> Result<(), String> {
        if let Some(info) = self.variables.get_mut(name) {
            if !info.mutable {
                return Err(format!("Variável '{}' não é mutável", name));
            }
            if info.moved {
                return Err(format!("Variável '{}' foi movida", name));
            }
            info.value = new_value;
            Ok(())
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().set_variable(name, new_value)
        } else {
            Err(format!("Variável '{}' não encontrada", name))
        }
    }

    pub fn nest(parent: &Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            variables: HashMap::new(),
            procedures: HashMap::new(),
            parent: Some(parent.clone()),
        }))
    }
}