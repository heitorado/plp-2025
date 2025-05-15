use crate::ast::{Command, ProcedureParameter, Type, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct RuntimeVariable {
    pub value: Value,
    pub moved: bool,
}

#[derive(Debug, Clone)]
pub struct RuntimeEnvironment {
    pub variables: HashMap<String, RuntimeVariable>,
    pub procedures: HashMap<String, (Vec<ProcedureParameter>, Option<Type>, Command)>,
    pub parent: Option<Rc<RefCell<RuntimeEnvironment>>>,
}

impl RuntimeEnvironment {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            variables: HashMap::new(),
            procedures: HashMap::new(),
            parent: None,
        }))
    }

    pub fn nest(parent: &Rc<RefCell<RuntimeEnvironment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            variables: HashMap::new(),
            procedures: HashMap::new(),
            parent: Some(parent.clone()), // Mantém referência ao escopo pai
        }))
    }

    // Buscar variaveis em todos os escopos
    pub fn lookup_variable(&self, name: &str) -> Option<RuntimeVariable> {
        self.variables.get(name).cloned().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|p| p.borrow().lookup_variable(name))
        })
    }

    // Buscar funcoes em todos os escopos
    pub fn lookup_procedure(
        &self,
        name: &str,
    ) -> Option<(Vec<ProcedureParameter>, Option<Type>, Command)> {
        self.procedures
            .get(name)
            .cloned()
            .or_else(|| match &self.parent {
                Some(parent) => parent.borrow().lookup_procedure(name),
                None => None,
            })
    }
}
