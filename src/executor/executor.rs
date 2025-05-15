use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::{
    BinaryOperator, CallProcedure, Command, ConcreteValue, Declaration, Expression, IOCommand,
    Program, UnaryOperator, Value,
};

use crate::executor::environment::RuntimeEnvironment;
use crate::executor::environment::RuntimeVariable;

#[derive(Debug, Clone)]
pub struct Executor {
    pub env: Rc<RefCell<RuntimeEnvironment>>,
    pub errors: Vec<String>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            env: RuntimeEnvironment::new(),
            errors: Vec::new(),
        }
    }

    pub fn execute_program(&mut self, program: &Program) -> Result<(), Vec<String>> {
        match program {
            Program::Command(cmd) => self.execute_command(cmd),
        }
        if !self.errors.is_empty() {
            Err(self.errors.clone())
        } else {
            Ok(())
        }
    }

    pub fn execute_command(&mut self, cmd: &Command) {
        match cmd {
            Command::Assignment(var, expr, is_move) => self.execute_assignment(var, expr, is_move),
            Command::DeclarationBlock(decls, body) => self.execute_declaration_block(decls, body),
            Command::WhileLoop(condition, body) => self.execute_while_loop(condition, body),
            Command::IfElse(cond, then_cmd, else_cmd) => {
                self.execute_if_else(cond, then_cmd, else_cmd)
            }
            Command::IO(io_command) => self.execute_io(io_command),
            Command::Sequence(cmd1, cmd2) => {
                self.execute_command(cmd1);
                self.execute_command(cmd2);
            }
            Command::Skip => {}
            Command::Evaluate(expr) => {
                self.execute_expression(expr);
            }
        }
    }

    pub fn execute_assignment(&mut self, var: &String, expr: &Expression, is_move: &bool) {
        let value = self.execute_expression(expr);

        let mut current_env = self.env.clone();
        loop {
            let found = {
                let mut env = current_env.borrow_mut();
                if let Some(v) = env.variables.get_mut(var) {
                    v.value = value.clone();
                    v.moved = *is_move;
                    true
                } else {
                    false
                }
            };

            if found {
                if *is_move {
                    if let Expression::Identifier(source_var) = expr {
                        self.remove_variable(source_var);
                    }
                }
                return;
            }

            let parent_env = {
                let borrowed = current_env.borrow();
                borrowed.parent.clone()
            };

            if let Some(parent) = parent_env {
                current_env = parent;
            } else {
                self.errors.push(format!(
                    "Atribuição inválida. Variável '{}' não declarada.",
                    var
                ));
                break;
            }
        }
    }

    pub fn execute_declaration_block(&mut self, decls: &[Declaration], body: &Command) {
        let old_env = self.env.clone();
        self.env = RuntimeEnvironment::nest(&old_env);

        for decl in decls {
            self.execute_declaration(decl);
        }

        self.execute_command(body);
        self.env = old_env;
    }

    pub fn execute_declaration(&mut self, decl: &Declaration) {
        match decl {
            Declaration::Variable(name, expr, is_moved) => {
                let value = self.execute_expression(expr);

                if *is_moved {
                    if let Expression::Identifier(source_var) = expr {
                        self.remove_variable(source_var);
                    }
                }

                self.env.borrow_mut().variables.insert(
                    name.clone(),
                    RuntimeVariable {
                        value,
                        moved: false,
                    },
                );
            }
            Declaration::Procedure(name, params, return_type, body) => {
                self.env.borrow_mut().procedures.insert(
                    name.clone(),
                    (params.clone(), return_type.clone(), *body.clone()),
                );
            }
            Declaration::Compound(decl_1, decl_2) => {
                self.execute_declaration(decl_1);
                self.execute_declaration(decl_2);
            } // _ => panic!("Error executing declaration [NOT SUPPORTED]: {:?}", decl),
        }
    }

    pub fn execute_while_loop(&mut self, condition: &Expression, body: &Command) {
        loop {
            let condition_result = self.execute_expression(condition);
            match condition_result {
                Value::Bool(true) => self.execute_command(body),
                Value::Bool(false) => break,
                _ => panic!("Invalid type for WHILE condition: {:?}", condition_result),
            }
        }
    }

    pub fn execute_if_else(
        &mut self,
        condition: &Expression,
        then_cmd: &Command,
        else_cmd: &Command,
    ) {
        let value = self.execute_expression(condition);
        if let Value::Bool(true) = value {
            self.execute_command(then_cmd);
        } else {
            self.execute_command(else_cmd);
        }
    }

    pub fn execute_io(&mut self, io_command: &IOCommand) {
        match io_command {
            IOCommand::Write(expr) => {
                let value = self.execute_expression(expr);
                println!("{}", value);
            }
            IOCommand::Read(var) => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.trim().to_string();

                let value = if let Ok(n) = input.parse::<i64>() {
                    Value::Int(n)
                } else if let Ok(b) = input.parse::<bool>() {
                    Value::Bool(b)
                } else {
                    Value::Str(input)
                };

                self.env.borrow_mut().variables.insert(
                    var.clone(),
                    RuntimeVariable {
                        value,
                        moved: false,
                    },
                );
            }
        }
    }

    fn execute_call_procedure(&mut self, call: &CallProcedure) -> Value {
        let proc = self.env.borrow().lookup_procedure(&call.id);
        if proc.is_none() {
            self.errors
                .push(format!("Procedimento '{}' não declarado.", call.id));
            return Value::Unit;
        }

        let (params, _, body) = proc.unwrap();
        let args: Vec<Value> = call
            .args
            .iter()
            .map(|e| self.execute_expression(e))
            .collect();

        let old_env = self.env.clone();
        self.env = RuntimeEnvironment::nest(&old_env);

        for (param, arg_value) in params.iter().zip(args) {
            self.env.borrow_mut().variables.insert(
                param.identifier.clone(),
                RuntimeVariable {
                    value: arg_value,
                    moved: false,
                },
            );
        }

        self.execute_command(&body);

        let result = self.get_last_value(&body);
        self.env = old_env;
        result
    }

    pub fn execute_expression(&mut self, expr: &Expression) -> Value {
        match expr {
            Expression::CallProcedure(call) => self.execute_call_procedure(call),
            Expression::ConcreteValue(value) => self.execute_concrete_value(value),
            Expression::Identifier(var) => {
                let env = self.env.borrow();
                let variable = env
                    .lookup_variable(var)
                    .ok_or_else(|| format!("Variável '{}' não definida", var));
                match variable {
                    Ok(variable_info) => {
                        return variable_info.value;
                    }
                    Err(e) => {
                        panic!("{e}")
                    }
                }
            }
            Expression::UnaryExp(op, expr) => self.execute_unary_expression(op, expr),
            Expression::BinaryExp(left, op, right) => {
                self.execute_binary_expression(left, op, right)
            }
        }
    }

    fn get_last_value(&mut self, cmd: &Command) -> Value {
        match cmd {
            Command::Sequence(_, cmd2) => self.get_last_value(cmd2),
            Command::Evaluate(expr) => self.execute_expression(expr),
            _ => Value::Unit,
        }
    }

    pub fn execute_concrete_value(&mut self, value: &ConcreteValue) -> Value {
        match value {
            ConcreteValue::Value(value) => match value {
                Value::Int(value) => Value::Int(*value),
                Value::Bool(value) => Value::Bool(*value),
                Value::Str(value) => Value::Str(value.to_string()),
                Value::Unit => Value::Unit,
            },
        }
    }

    pub fn execute_unary_expression(&mut self, op: &UnaryOperator, expr: &Expression) -> Value {
        match op {
            UnaryOperator::Not => match expr {
                Expression::ConcreteValue(value) => match value {
                    ConcreteValue::Value(value) => match value {
                        Value::Bool(value) => Value::Bool(!value),
                        _ => panic!("Invalid type for NOT operator: {:?}", value),
                    },
                },
                _ => {
                    let result = self.execute_expression(expr);
                    match result {
                        Value::Bool(value) => Value::Bool(!value),
                        _ => panic!("Invalid type for NOT operator in expression: {:?}", result),
                    }
                }
            },
            UnaryOperator::Neg => match expr {
                Expression::ConcreteValue(value) => match value {
                    ConcreteValue::Value(value) => match value {
                        Value::Int(value) => Value::Int(-value),
                        _ => panic!("Invalid type for NEG operator: {:?}", value),
                    },
                },
                _ => {
                    let result = self.execute_expression(expr);
                    match result {
                        Value::Int(value) => Value::Int(-value),
                        _ => panic!("Invalid type for NEG operator in expression: {:?}", result),
                    }
                }
            },
            UnaryOperator::Length => match expr {
                Expression::ConcreteValue(value) => match value {
                    ConcreteValue::Value(value) => match value {
                        Value::Str(value) => Value::Int(value.len().try_into().unwrap()),
                        _ => panic!("Invalid type for LENGTH operator: {:?}", value),
                    },
                },
                _ => {
                    let result = self.execute_expression(expr);
                    match result {
                        Value::Str(value) => Value::Int(value.len().try_into().unwrap()),
                        _ => panic!(
                            "Invalid type for LENGTH operator in expression: {:?}",
                            result
                        ),
                    }
                }
            },
        }
    }

    pub fn execute_binary_expression(
        &mut self,
        op: &BinaryOperator,
        left: &Expression,
        right: &Expression,
    ) -> Value {
        match op {
            BinaryOperator::Add => {
                let left = self.execute_expression(left);
                let right = self.execute_expression(right);
                match (left.clone(), right.clone()) {
                    (Value::Int(left), Value::Int(right)) => Value::Int(left + right),
                    (Value::Str(left), Value::Str(right)) => {
                        Value::Str(format!("{}{}", left, right))
                    }
                    _ => panic!("Invalid types for ADD operator: {:?} and {:?}", left, right),
                }
            }
            BinaryOperator::Sub => {
                let left = self.execute_expression(left);
                let right = self.execute_expression(right);
                match (left.clone(), right.clone()) {
                    (Value::Int(left), Value::Int(right)) => Value::Int(left - right),
                    _ => panic!("Invalid types for SUB operator: {:?} and {:?}", left, right),
                }
            }
            BinaryOperator::Equal => {
                let left = self.execute_expression(left);
                let right = self.execute_expression(right);
                match (left.clone(), right.clone()) {
                    (Value::Int(left), Value::Int(right)) => Value::Bool(left == right),
                    (Value::Str(left), Value::Str(right)) => Value::Bool(left == right),
                    (Value::Bool(left), Value::Bool(right)) => Value::Bool(left == right),
                    _ => panic!(
                        "Invalid types for EQUAL operator: {:?} and {:?}",
                        left, right
                    ),
                }
            }
            BinaryOperator::And => {
                let left = self.execute_expression(left);
                let right = self.execute_expression(right);
                match (left.clone(), right.clone()) {
                    (Value::Bool(left), Value::Bool(right)) => Value::Bool(left && right),
                    _ => panic!("Invalid types for AND operator: {:?} and {:?}", left, right),
                }
            }
            BinaryOperator::Or => {
                let left = self.execute_expression(left);
                let right = self.execute_expression(right);
                match (left.clone(), right.clone()) {
                    (Value::Bool(left), Value::Bool(right)) => Value::Bool(left || right),
                    _ => panic!("Invalid types for OR operator: {:?} and {:?}", left, right),
                }
            }
            BinaryOperator::Concat => {
                let left = self.execute_expression(left);
                let right = self.execute_expression(right);
                match (left.clone(), right.clone()) {
                    (Value::Str(left), Value::Str(right)) => {
                        Value::Str(format!("{}{}", left, right))
                    }
                    _ => panic!(
                        "Invalid types for CONCAT operator: {:?} and {:?}",
                        left, right
                    ),
                }
            }
            BinaryOperator::Less => {
                let left = self.execute_expression(left);
                let right = self.execute_expression(right);
                match (left.clone(), right.clone()) {
                    (Value::Int(left), Value::Int(right)) => Value::Bool(left < right),
                    _ => panic!(
                        "Invalid types for LESS operator: {:?} and {:?}",
                        left, right
                    ),
                }
            }
            BinaryOperator::LessEqual => {
                let left = self.execute_expression(left);
                let right = self.execute_expression(right);
                match (left.clone(), right.clone()) {
                    (Value::Int(left), Value::Int(right)) => Value::Bool(left <= right),
                    _ => panic!(
                        "Invalid types for LESS_EQUAL operator: {:?} and {:?}",
                        left, right
                    ),
                }
            }
            BinaryOperator::Greater => {
                let left = self.execute_expression(left);
                let right = self.execute_expression(right);
                match (left.clone(), right.clone()) {
                    (Value::Int(left), Value::Int(right)) => Value::Bool(left > right),
                    _ => panic!(
                        "Invalid types for GREATER operator: {:?} and {:?}",
                        left, right
                    ),
                }
            }
            BinaryOperator::GreaterEqual => {
                let left = self.execute_expression(left);
                let right = self.execute_expression(right);
                match (left.clone(), right.clone()) {
                    (Value::Int(left), Value::Int(right)) => Value::Bool(left >= right),
                    _ => panic!(
                        "Invalid types for GREATER_EQUAL operator: {:?} and {:?}",
                        left, right
                    ),
                }
            }
        }
    }

    fn remove_variable(&mut self, var: &str) {
        let mut current = Rc::clone(&self.env);

        loop {
            let removed = {
                let mut env = current.borrow_mut();
                env.variables.remove(var).is_some()
            };
            if removed {
                break;
            }

            let parent_opt = {
                let env = current.borrow();
                env.parent.clone()
            };

            match parent_opt {
                Some(parent) => current = parent,
                None => break,
            }
        }
    }
}
