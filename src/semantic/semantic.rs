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
                // Busca o procedimento
                let procedure_info = self
                    .env
                    .borrow()
                    .lookup_procedure(&call.id)
                    .ok_or_else(|| vec![format!("Procedure '{}' não foi declarada", call.id)])?;

                // Verificações iniciais sem modificar o estado
                if call.args.len() != procedure_info.0.len() {
                    self.report_error(format!(
                        "Esperados {} parâmetros, recebidos {} na chamada de {}",
                        procedure_info.0.len(),
                        call.args.len(),
                        call.id
                    ));
                }

                // Coletar variáveis para mover após verificação de tipos
                let mut vars_to_move = Vec::new();

                for (arg, param) in call.args.iter().zip(procedure_info.0.iter()) {
                    let arg_type = self.check_expression(arg)?;

                    if arg_type != param.r#type {
                        self.report_error(format!(
                            "Tipo incompatível para parâmetro '{}': esperado {:?}, recebido {:?}",
                            param.identifier, param.r#type, arg_type
                        ));
                    }

                    // Coletar identificadores para mover depois
                    if let Expression::Identifier(var_name) = arg {
                        vars_to_move.push(var_name.clone());
                    }
                }

                // Modificações de estado separadamente
                for var_name in vars_to_move {
                    // Verificando o estado
                    let is_already_moved = {
                        let env = self.env.borrow();
                        env.lookup_variable(&var_name)
                            .map(|info| info.moved)
                            .unwrap_or(false)
                    };

                    if is_already_moved {
                        self.report_error(format!(
                            "Variável '{}' já foi movida anteriormente",
                            var_name
                        ));
                    }
                    self.mark_variable_as_moved(&var_name);
                }

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
        // Variável existe?
        let var_info = self
            .env
            .borrow()
            .lookup_variable(var)
            .ok_or_else(|| vec![format!("Variável não declarada: {}", var)])?;

        // Verifica se variável foi movida
        if var_info.moved {
            self.report_error(format!("Variável '{}' já foi movida.", var));
        }

        // Expressão
        let expr_type = self.check_expression(expr)?;

        // Types
        if expr_type != var_info.type_ {
            self.report_error(format!(
                "Tipo incompatível para '{}': esperado {:?}, encontrado {:?}",
                var, var_info.type_, expr_type
            ))
        }

        // Atualiza o estado se estiver movendo
        if is_move {
            if let Expression::Identifier(moved_var) = expr {
                self.mark_as_moved(moved_var);
            } else {
                self.report_error("Operação move só pode ser aplicada a identificadores.")
            }
        }

        Ok(())
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
            // Verifica se a variável existe neste escopo
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

            // Move para o escopo pai se existir
            let parent = match &current_env.borrow().parent {
                Some(p) => p.clone(),
                None => break,
            };

            current_env = parent;
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
                        mutable: true,
                    },
                );

                Ok(())
            }
            Declaration::Procedure(name, params, body) => {
                // Duplicadas
                let mut param_names = HashSet::new();

                for param in params {
                    if param_names.contains(&param.identifier) {
                        self.report_error(format!("Parâmetro duplicado: {}", param.identifier));
                    }
                    param_names.insert(param.identifier.clone());
                }

                // Procedimento no ambiente
                self.env
                    .borrow_mut()
                    .procedures
                    .insert(name.clone(), (params.clone(), None));

                // novo escopo para o corpo
                let old_env = self.env.clone();
                self.env = Environment::nest(&old_env);

                // adiciona parâmetros ao ambiente
                for param in params {
                    self.env.borrow_mut().variables.insert(
                        param.identifier.clone(),
                        VariableInfo {
                            type_: param.r#type.clone(),
                            moved: false,
                            mutable: true,
                        },
                    );
                }

                // Corpo da funcao
                self.check_command(body)?;

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
                            self.report_error("Negação aplicada a um não-inteiro");
                        }

                        Ok(Type::Int)
                    }
                    UnaryOperator::Not => {
                        if expr_type != Type::Bool {
                            self.report_error("Negação lógica aplicada a um não-boleano");
                        }

                        Ok(Type::Bool)
                    }
                    UnaryOperator::Length => {
                        if expr_type != Type::Str {
                            self.report_error("Length aplicado a um não-string");
                        }

                        Ok(Type::Str)
                    }
                }
            }
            Expression::BinaryExp(op, left, right) => {
                let left_type = self.check_expression(left)?;
                let right_type = self.check_expression(right)?;

                match op {
                    BinaryOperator::Add | BinaryOperator::Sub => {
                        if left_type != Type::Int || right_type != Type::Int {
                            self.report_error(format!("Operações aritimétricas precisão de 2 inteiros, passados {:?} e {:?}", left_type, right_type));
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
                            self.report_error("Operador lógico precisa de 2 boleanos");
                        }

                        Ok(Type::Bool)
                    }
                    BinaryOperator::Concat => {
                        if left_type != Type::Str || right_type != Type::Str {
                            self.report_error("Concatenação precisa de duas strings".to_string());
                        }
                        Ok(Type::Str)
                    }

                    BinaryOperator::Less
                    | BinaryOperator::LessEqual
                    | BinaryOperator::Greater
                    | BinaryOperator::GreaterEqual => {
                        if left_type != Type::Int || right_type != Type::Int {
                            self.report_error("Comparações precisão de dois inteiros".to_string());
                        }
                        Ok(Type::Bool)
                    }
                }
            }
        }
    }

    pub fn report_error(&mut self, msg: impl Into<String>) {
        self.errors.push(msg.into());
    }
}
