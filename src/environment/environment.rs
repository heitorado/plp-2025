use crate::ast::{ProcedureParameter, Type};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
    // Armazenar as varáveis
    // Tipo
    // foi movida
    // se é mutável
    pub variables: HashMap<String, VariableInfo>,

    // Funções
    // Parametros
    pub procedures: HashMap<String, (Vec<ProcedureParameter>, Box<Command>)>,

    // Blocos aninhados
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

    pub fn nest(parent: &Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            variables: HashMap::new(),
            procedures: HashMap::new(),
            parent: Some(parent.clone()), // Mantém referência ao escopo pai
        }))
    }

    // Buscar variaveis em todos os escopos
    pub fn lookup_variable(&self, name: &str) -> Option<VariableInfo> {
        self.variables.get(name).cloned().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|p| p.borrow().lookup_variable(name))
        })
    }

    // Buscar funcoes em todos os escopos
    pub fn lookup_procedure(&self, name: &str) -> Option<(Vec<ProcedureParameter>, Option<Type>)> {
        self.procedures.get(name).cloned().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.borrow().lookup_procedure(name))
        })
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
}
