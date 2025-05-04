use crate::ast::{ProcedureParameter, Type};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct VariableInfo {
    pub type_: Type,
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
    pub procedures: HashMap<String, (Vec<ProcedureParameter>, Option<Type>)>,

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
}
