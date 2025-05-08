use crate::ast::{ProcedureParameter, Type, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// #[derive(Debug, Clone, PartialEq)]
// pub enum RuntimeValue {
//     Int(i64),
//     Bool(bool),
//     Str(String),
// }

#[derive(Debug, Clone, PartialEq)]
pub struct VariableInfo {
    // pub moved: bool,
    pub value: Value,
}

#[derive(Debug, Clone)]
pub struct RuntimeEnvironment {
    // Armazenar as varáveis
    // Tipo
    // foi movida
    // se é mutável
    pub variables: HashMap<String, VariableInfo>,

    // Funções
    // Parametros
    pub procedures: HashMap<String, (Vec<ProcedureParameter>, Option<Type>)>,

    // Blocos aninhados
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
