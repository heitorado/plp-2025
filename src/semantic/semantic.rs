use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::ast::{
    BinaryOperator, Command, ConcreteValue, Declaration, Expression, IOCommand, Program, Type,
    UnaryOperator, Value,
};

use crate::environment::environment::{Environment, VariableInfo};

#[derive(Debug, Clone)]
pub struct SemanticAnalyzer {
    pub env: Rc<RefCell<Environment>>,
    pub errors: Vec<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            env: Environment::new(),
            errors: Vec::new(),
        }
    }

    pub fn check_program(&mut self, program: &Program) -> Result<(), Vec<String>> {
        match program {
            Program::Command(cmd) => {
                self.check_command(cmd)?;
                // Ok(())
            }
        }
        // self.check_command(program.)?;

        if !self.errors.is_empty() {
            Err(self.errors.clone())
        } else {
            Ok(())
        }
    }

    pub fn check_command(&mut self, cmd: &Command) -> Result<(), Vec<String>> {
        match cmd {
            Command::Assignment(var, expr, is_move) => self.check_assignment(var, expr, *is_move),
            Command::DeclarationBlock(decls, cmd) => {
                let old_env = self.env.clone();
                self.env = Environment::nest(&old_env);

                for decl in decls {
                    self.check_declaration(decl)?;
                }

                self.check_command(cmd)?;

                self.env = old_env;

                Ok(())
            }
            Command::WhileLoop(cond, body) => {
                let cond_type = self.check_expression(cond)?;
                if cond_type != Type::Bool {
                    self.report_error("Condição do while precisa ser boleana.");
                }

                self.check_command(body)?;

                Ok(())
            }
            Command::IfElse(cond, cmd1, cmd2) => {
                let cond_type = self.check_expression(cond)?;

                if cond_type != Type::Bool {
                    self.report_error("Condição do If/Else precisa ser boleana.")
                }

                self.check_command(cmd1)?;
                self.check_command(cmd2)?;

                Ok(())
            }
            Command::IO(io_cmd) => match io_cmd {
                IOCommand::Read(var) => {
                    let var_info = self
                        .env
                        .borrow()
                        .lookup_variable(var)
                        .ok_or_else(|| vec![format!("Variável {} não declarada", var)])?;

                    if var_info.moved {
                        self.report_error(format!("Não pode ler uma variável movida '{}'", var));
                    }

                    Ok(())
                }
                IOCommand::Write(expr) => {
                    let _ = self.check_expression(expr)?;

                    Ok(())
                }
            },
            Command::Sequence(cmd1, cmd2) => {
                self.check_command(cmd1)?;
                self.check_command(cmd2)?;

                Ok(())
            }
            Command::Skip => Ok(()),
            Command::CallProcedure(call) => {
                let _ = self.check_expression(&Expression::CallProcedure(call.clone()))?;
                Ok(())
            }
        }
    }

    pub fn check_assignment(
        &mut self,
        var: &str,
        expr: &Expression,
        is_move: bool,
    ) -> Result<(), Vec<String>> {
        let rhs_type = self.check_expression(expr)?;

        if is_move {
            if let Expression::Identifier(source_var) = expr {
                let (exists, already_moved) = {
                    let env = self.env.borrow();
                    (
                        env.variables.contains_key(source_var),
                        env.variables.get(source_var).map_or(false, |v| v.moved),
                    )
                };

                if !exists {
                    return Err(vec![format!("Variável '{}' não declarada", source_var)]);
                } else if already_moved {
                    return Err(vec![format!("Variável '{}' já foi movida", source_var)]);
                }

                let mut env = self.env.borrow_mut();
                if let Some(source_info) = env.variables.get_mut(source_var) {
                    source_info.moved = true;
                }
            } else {
                return Err(vec![
                    "Move só pode ser aplicado a identificadores".to_string(),
                ]);
            }
        }

        let mut env = self.env.borrow_mut();
        match env.variables.get_mut(var) {
            Some(var_info) => {
                if rhs_type != var_info.type_ {
                    return Err(vec![format!(
                        "Tipo incompatível na atribuição de '{}': esperado {:?}, obtido {:?}",
                        var, var_info.type_, rhs_type
                    )]);
                }

                var_info.moved = false;
                Ok(())
            }
            None => Err(vec![format!("Variável '{}' não declarada", var)]),
        }
    }

    pub fn mark_as_moved(&mut self, var: &str) {
        let mut env = self.env.borrow_mut();
        if let Some(info) = env.variables.get_mut(var) {
            info.moved = true;
        }
    }

    pub fn mark_variable_as_moved(&mut self, var: &str) {
        let mut current_env = self.env.clone();

        loop {
            let found = {
                let mut borrowed_env = current_env.borrow_mut();
                if let Some(info) = borrowed_env.variables.get_mut(var) {
                    info.moved = true;
                    true
                } else {
                    false
                }
            };

            if found {
                break;
            }

            let parent = match &current_env.borrow().parent {
                Some(p) => p.clone(),
                None => break,
            };

            current_env = parent;
        }
    }

    fn get_last_expression_type(&mut self, cmd: &Command) -> Type {
        match cmd {
            Command::Sequence(_, cmd2) => self.get_last_expression_type(cmd2),
            Command::CallProcedure(call) => self.check_expression(&Expression::CallProcedure(call.clone())).unwrap_or(Type::Unit),
            _ => Type::Unit,
        }
    }

    pub fn check_declaration(&mut self, decl: &Declaration) -> Result<(), Vec<String>> {
        match decl {
            Declaration::Variable(name, expr, is_move) => {
                let expr_type = self.check_expression(expr)?;

                // Duplicata
                if self.env.borrow().variables.contains_key(name) {
                    self.report_error(format!("Variável já declarada: {}", name));
                }

                // Tratamento movimentacoes
                if *is_move {
                    if let Expression::Identifier(source_var) = expr {
                        // Verifica a variável fonte
                        let source_info = self
                            .env
                            .borrow()
                            .lookup_variable(source_var)
                            .ok_or_else(|| {
                                vec![format!("Variável não declarada: {}", source_var)]
                            })?;

                        if source_info.moved {
                            self.report_error(format!("Variável '{}' já foi movida", source_var));
                        }

                        // Marca a variável fonte como movida
                        self.mark_variable_as_moved(source_var);
                    } else {
                        self.report_error(
                            "Move só pode ser aplicado a identificadores".to_string(),
                        );
                    }
                }

                // Adiciona ao ambiente
                self.env.borrow_mut().variables.insert(
                    name.clone(),
                    VariableInfo {
                        type_: expr_type,
                        moved: false,
                    },
                );

                Ok(())
            }
            Declaration::Procedure(name, params, return_type, body) => {
                // Verificar parâmetros duplicados
                let mut param_names = HashSet::new();
                for param in params {
                    if param_names.contains(&param.identifier) {
                        self.report_error(format!("Parâmetro duplicado: {}", param.identifier));
                    }
                    param_names.insert(param.identifier.clone());
                }

                // Registrar procedimento no ambiente com tipo de retorno
                self.env
                    .borrow_mut()
                    .procedures
                    .insert(name.clone(), (params.clone(), return_type.clone()));

                // Criar novo escopo para o corpo
                let old_env = self.env.clone();
                self.env = Environment::nest(&old_env);

                // Adicionar parâmetros ao ambiente
                for param in params {
                    self.env.borrow_mut().variables.insert(
                        param.identifier.clone(),
                        VariableInfo {
                            type_: param.r#type.clone(),
                            moved: false,
                        },
                    );
                }

                // Verificar corpo do procedimento
                self.check_command(body)?;

                // Verificar tipo de retorno se necessário
                if let Some(declared_type) = return_type {
                    let body_type = self.get_last_expression_type(body);
                    if body_type != *declared_type {
                        self.report_error(format!(
                            "Tipo de retorno incompatível: esperado {:?}, encontrado {:?}",
                            declared_type, body_type
                        ));
                    }
                }

                self.env = old_env;
                Ok(())
            }

            Declaration::Compound(d1, d2) => {
                self.check_declaration(d1)?;
                self.check_declaration(d2)?;

                Ok(())
            }
        }
    }

    pub fn check_expression(&mut self, expr: &Expression) -> Result<Type, Vec<String>> {
        match expr {
            Expression::ConcreteValue(cv) => match cv {
                ConcreteValue::Value(Value::Int(_)) => Ok(Type::Int),
                ConcreteValue::Value(Value::Str(_)) => Ok(Type::Str),
                ConcreteValue::Value(Value::Bool(_)) => Ok(Type::Bool),
                ConcreteValue::Value(Value::Unit) => Ok(Type::Unit),
            },
            Expression::Identifier(var) => {
                let var_info = self
                    .env
                    .borrow()
                    .lookup_variable(var)
                    .ok_or_else(|| vec![format!("Variável não declarada: {}", var)])?;

                if var_info.moved {
                    self.report_error(format!("Uso de variável movida: {}", var));
                }

                Ok(var_info.type_.clone())
            }
            Expression::UnaryExp(op, expr) => {
                let expr_type = self.check_expression(expr)?;
                match op {
                    UnaryOperator::Neg => {
                        if expr_type != Type::Int {
                            self.report_error(format!("Sinal negativo aplicado a algo que não é inteiro: {:?}", expr_type));
                        }

                        Ok(Type::Int)
                    }
                    UnaryOperator::Not => {
                        if expr_type != Type::Bool {
                            self.report_error(format!("Negação lógica aplicada a algo que não é booleano: {:?}", expr_type));
                        }

                        Ok(Type::Bool)
                    }
                    UnaryOperator::Length => {
                        if expr_type != Type::Str {
                            self.report_error(format!("Length aplicado a algo que não é string: {:?}", expr_type));
                        }

                        Ok(Type::Int)
                    }
                }
            }
            Expression::BinaryExp(op, left, right) => {
                let left_type = self.check_expression(left)?;
                let right_type = self.check_expression(right)?;

                match op {
                    BinaryOperator::Add | BinaryOperator::Sub => {
                        if left_type != Type::Int || right_type != Type::Int {
                            self.report_error(format!("Operações aritméticas esperam 2 inteiros, porém foi passado {:?} e {:?}", left_type, right_type));
                        }

                        Ok(Type::Int)
                    }
                    BinaryOperator::Equal => {
                        if left_type != right_type {
                            self.report_error(format!(
                                "Não pode comprar tipos diferentes: {:?} e {:?}",
                                left_type, right_type
                            ));
                        }

                        Ok(Type::Bool)
                    }
                    BinaryOperator::And | BinaryOperator::Or => {
                        if left_type != Type::Bool || right_type != Type::Bool {
                            self.report_error("Operador lógico espera 2 booleanos");
                        }

                        Ok(Type::Bool)
                    }
                    BinaryOperator::Concat => {
                        if left_type != Type::Str || right_type != Type::Str {
                            self.report_error("Concatenação espera duas strings".to_string());
                        }
                        Ok(Type::Str)
                    }

                    BinaryOperator::Less
                    | BinaryOperator::LessEqual
                    | BinaryOperator::Greater
                    | BinaryOperator::GreaterEqual => {
                        if left_type != Type::Int || right_type != Type::Int {
                            self.report_error("Comparações esperam dois inteiros".to_string());
                        }
                        Ok(Type::Bool)
                    }
                }
            }
            Expression::CallProcedure(call) => {
                let proc_info = self
                    .env
                    .borrow()
                    .lookup_procedure(&call.id)
                    .ok_or_else(|| vec![format!("Procedimento não declarado: {}", call.id)])?;

                // Verificar número de argumentos
                if call.args.len() != proc_info.0.len() {
                    self.report_error(format!(
                        "Número incorreto de argumentos para {}: esperado {}, obtido {}",
                        call.id,
                        proc_info.0.len(),
                        call.args.len()
                    ));
                }

                // Verificar tipos e coletar variáveis para mover
                let mut vars_to_move = Vec::new();
                for (i, (arg, param)) in call.args.iter().zip(proc_info.0.iter()).enumerate() {
                    let arg_type = self.check_expression(arg)?;

                    // Verificar compatibilidade de tipos
                    if arg_type != param.r#type {
                        self.report_error(format!(
                            "Tipo inválido para argumento {} em {}: esperado {:?}, obtido {:?}",
                            i + 1,
                            call.id,
                            param.r#type,
                            arg_type
                        ));
                    }

                    // Coletar identificadores para mover após verificação
                    if let Expression::Identifier(var_name) = arg {
                        vars_to_move.push(var_name.to_string());
                    }
                }

                // Processar movimentação das variáveis
                for var_name in vars_to_move {
                    // Verificar com borrow imutável primeiro
                    let exists_and_not_moved = {
                        let env = self.env.borrow();
                        env.variables.get(&var_name).map_or(false, |v| !v.moved)
                    };

                    if !exists_and_not_moved {
                        // Verificar se existe para mensagem de erro precisa
                        let exists = self.env.borrow().variables.contains_key(&var_name);
                        if !exists {
                            self.report_error(format!("Variável '{}' não declarada", var_name));
                        } else {
                            self.report_error(format!("Variável '{}' já foi movida", var_name));
                        }
                    } else {
                        // Marcar como movida com borrow mutável
                        let mut env = self.env.borrow_mut();
                        if let Some(var_info) = env.variables.get_mut(&var_name) {
                            var_info.moved = true;
                        }
                    }
                }

                // Retornar tipo do procedimento
                match &proc_info.1 {
                    Some(t) => Ok(t.clone()),
                    None => Ok(Type::Unit),
                }
            }
        }
    }

    pub fn report_error(&mut self, msg: impl Into<String>) {
        self.errors.push(msg.into());
    }
}
