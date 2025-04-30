use crate::ast::*;
use crate::environment::Environment;
use crate::environment::RuntimeValue;
use crate::environment::VariableInfo;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Executor {
    env: Rc<RefCell<Environment>>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            env: Environment::new(),
        }
    }

    pub fn execute_program(&mut self, program: &Program) {
        if let Program::Command(cmd) = program {
            self.execute_command(cmd);
        }
    }

    pub fn execute_command(&mut self, cmd: &Command) {
        match cmd {
            Command::Assignment(var, expr, is_move) => {
                if *is_move {
                    if let Expression::Identifier(origin) = expr {
                        match self.env.borrow_mut().move_variable(origin) {
                            Ok(moved_value) => {
                                let _ = self.env.borrow_mut().declare_variable(
                                    var.clone(),
                                    self.infer_type(&moved_value),
                                    moved_value,
                                    true,
                                );
                            }
                            Err(e) => println!("Erro ao mover '{}': {}", origin, e),
                        }
                    } else {
                        println!("Erro: 'move' só pode ser usado com identificadores.");
                    }
                } else {
                    match self.evaluate_expression(expr) {
                        Ok(value) => {
                            if let Err(e) = self.env.borrow_mut().set_variable(var, value) {
                                println!("Erro ao atribuir '{}': {}", var, e);
                            }
                        }
                        Err(e) => println!("Erro ao avaliar expressão: {}", e),
                    }
                }
            }

            Command::DeclarationBlock(decls, body) => {
                let nested = Environment::nest(&self.env);
                let old = std::mem::replace(&mut self.env, nested);

                for decl in decls {
                    self.execute_declaration(decl);
                }

                self.execute_command(body);
                self.env = old;
            }

            Command::WhileLoop(cond, body) => {
                while let Ok(RuntimeValue::Bool(true)) = self.evaluate_expression(cond) {
                    self.execute_command(body);
                }
            }

            Command::IfElse(cond, then_b, else_b) => {
                match self.evaluate_expression(cond) {
                    Ok(RuntimeValue::Bool(true)) => self.execute_command(then_b),
                    Ok(RuntimeValue::Bool(false)) => self.execute_command(else_b),
                    Ok(_) => println!("Erro: condição não booleana."),
                    Err(e) => println!("Erro na condição: {}", e),
                }
            }

            Command::IO(io_cmd) => self.execute_io(io_cmd),

            Command::Sequence(left, right) => {
                self.execute_command(left);
                self.execute_command(right);
            }

            Command::Skip => {}

            Command::CallProcedure(proc_call) => {
                self.execute_call_procedure(proc_call);
            }            
        }
    }

    fn execute_declaration(&mut self, decl: &Declaration) {
        match decl {
            Declaration::Variable(name, expr, _) => {
                if let Ok(value) = self.evaluate_expression(expr) {
                    let _ = self.env.borrow_mut().declare_variable(
                        name.clone(),
                        self.infer_type(&value),
                        value,
                        true,
                    );
                }
            }

            Declaration::Compound(d1, d2) => {
                self.execute_declaration(d1);
                self.execute_declaration(d2);
            }

            Declaration::Procedure(name, params, body) => {
                self.env.borrow_mut().procedures.insert(name.clone(), (params.clone(), body.clone()));
            }
        }
    }

    fn execute_io(&mut self, io_command: &IOCommand) {
        match io_command {
            IOCommand::Write(expr) => match self.evaluate_expression(expr) {
                Ok(RuntimeValue::Int(i)) => println!("{}", i),
                Ok(RuntimeValue::Bool(b)) => println!("{}", b),
                Ok(RuntimeValue::Str(s)) => println!("{}", s),
                Err(e) => println!("Erro ao escrever: {}", e),
            },
            IOCommand::Read(var) => {
                println!("Leitura não implementada. Ignorando variável '{}'", var);
            }
        }
    }

    fn evaluate_expression(&self, expr: &Expression) -> Result<RuntimeValue, String> {
        match expr {
            Expression::ConcreteValue(ConcreteValue::Value(v)) => match v {
                Value::Int(i) => Ok(RuntimeValue::Int(*i)),
                Value::Bool(b) => Ok(RuntimeValue::Bool(*b)),
                Value::Str(s) => Ok(RuntimeValue::Str(s.clone())),
            },

            Expression::Identifier(name) => self.env.borrow().get_variable(name),

            Expression::UnaryExp(op, expr) => {
                let val = self.evaluate_expression(expr)?;
                match (op, val) {
                    (UnaryOperator::Neg, RuntimeValue::Int(i)) => Ok(RuntimeValue::Int(-i)),
                    (UnaryOperator::Not, RuntimeValue::Bool(b)) => Ok(RuntimeValue::Bool(!b)),
                    (UnaryOperator::Length, RuntimeValue::Str(s)) => {
                        Ok(RuntimeValue::Int(s.len() as i64))
                    }
                    _ => Err("Operador unário inválido.".to_string()),
                }
            }

            Expression::BinaryExp(op, left, right) => {
                let lval = self.evaluate_expression(left)?;
                let rval = self.evaluate_expression(right)?;
            
                match (op, lval, rval) {
                    (BinaryOperator::Add, RuntimeValue::Int(a), RuntimeValue::Int(b)) => {
                        Ok(RuntimeValue::Int(a + b))
                    }
                    (BinaryOperator::Sub, RuntimeValue::Int(a), RuntimeValue::Int(b)) => {
                        Ok(RuntimeValue::Int(a - b))
                    }
                    (BinaryOperator::Equal, a, b) => {
                        Ok(RuntimeValue::Bool(a == b))
                    }
                    (BinaryOperator::Concat, RuntimeValue::Str(a), RuntimeValue::Str(b)) => {
                        Ok(RuntimeValue::Str(a + &b))
                    }
                    _ => Err("Operador binário inválido.".to_string()),
                }
            }
            
        }
    }

    fn infer_type(&self, value: &RuntimeValue) -> Type {
        match value {
            RuntimeValue::Int(_) => Type::Int,
            RuntimeValue::Bool(_) => Type::Bool,
            RuntimeValue::Str(_) => Type::Str,
        }
    }

    fn execute_call_procedure(&mut self, call: &CallProcedure) {
        let (params, body) = match self.env.borrow().lookup_procedure(&call.id) {
            Some(proc) => proc,
            None => {
                println!("Procedimento '{}' não encontrado", call.id);
                return;
            }
        };
    
        if params.len() != call.args.len() {
            println!(
                "Procedimento '{}' esperava {} argumentos, mas recebeu {}.",
                call.id,
                params.len(),
                call.args.len()
            );
            return;
        }
    
        // Avalia argumentos
        let mut arg_values = Vec::new();
        for expr in &call.args {
            match self.evaluate_expression(expr) {
                Ok(val) => arg_values.push(val),
                Err(e) => {
                    println!("Erro ao avaliar argumento: {}", e);
                    return;
                }
            }
        }
    
        // Cria novo escopo
        let new_env = Environment::nest(&self.env);
        let old_env = std::mem::replace(&mut self.env, new_env);
    
        // Insere parâmetros no novo escopo
        for (param, value) in params.iter().zip(arg_values) {
            let _ = self.env.borrow_mut().declare_variable(
                param.identifier.clone(),
                param.r#type.clone(),
                value,
                true,
            );
        }
    
        // ✨ EXECUTA o corpo do procedimento
        self.execute_command(&body);
    
        // Volta para o ambiente anterior
        self.env = old_env;
    }    

}