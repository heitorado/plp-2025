use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::{
    BinaryOperator, Command, ConcreteValue, Declaration, Expression, IOCommand, Program, UnaryOperator, Value
};

use crate::environment::runtime_environment::{RuntimeEnvironment, VariableInfo};

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

    pub fn execute_program(&mut self, program: &Program) -> () {
        match program {
            Program::Command(cmd) => self.execute_command(cmd),
        }
    }
    
    pub fn execute_command(&mut self, cmd: &Command) -> () {
        match cmd {
            Command::Assignment(var, expr, is_move) => self.execute_assignment(var, expr, is_move), //println!("<Assignment> Var: {:?} | Expr: {:?} | IsMove: {:?}", var, expr, is_move),//self.execute_assignment(var, expr, *is_move),
            Command::DeclarationBlock(decls, body) => self.execute_declaration_block(decls, body),
            Command::WhileLoop(condition, body) => println!("<WhileLoop> Condition: {:?} | Body: {:?}", condition, body),//self.execute_while_loop(condition, body),
            Command::IfElse(condition, then_branch, else_branch) => self.execute_ifelse(condition, then_branch, else_branch),
            Command::IO(io_command) => self.execute_io(io_command),
            Command::Sequence(left, right) => { self.execute_command(left); self.execute_command(right); }
            Command::Skip => {},
            Command::CallProcedure(proc_name) => println!("<CallProcedure> ProcName: {:?}", proc_name),//self.execute_call_procedure(proc_name, args),
        }
    }

    // TODO: make use of is_move
    pub fn execute_assignment(&mut self, var: &String, expr: &Expression, is_move: &bool) -> () {
        let new_value = self.execute_expression(expr);

        let mut current_env = self.env.clone();
        loop {
            let found = {
                let mut env = current_env.borrow_mut();
                if let Some(v) = env.variables.get_mut(var) {
                    v.value = new_value.clone();
                    true
                } else {
                    false
                }
            };

            if found {
                return;
            }

            let parent = match &current_env.borrow().parent {
                Some(p) => p.clone(),
                None => break
            };

            current_env = parent;
        }
        panic!("Atribuição inválida. Variável '{}' não declarada.", var);
    }

    pub fn execute_declaration_block(&mut self, decls: &Vec<Declaration>, body: &Command) -> () {
        let old_env = self.env.clone();
        self.env = RuntimeEnvironment::nest(&old_env);

        for decl in decls {
            self.execute_declaration(decl);
        }
            
        self.execute_command(body);

        self.env = old_env;
    }

    pub fn execute_declaration(&mut self, decl: &Declaration) -> () {
        match decl {
            Declaration::Compound(decl_1, decl_2 ) => {
                self.execute_declaration(decl_1);
                self.execute_declaration(decl_2);
            }
            Declaration::Variable(name, expr, was_moved) => {
                let value = self.execute_expression(expr);
                self.env.borrow_mut().variables.insert(
                    name.clone(),
                    VariableInfo { value },
                );

                println!("Env State: {:?}", self.env.borrow_mut().variables);
            },
            _ => panic!("Error executing declaration [NOT SUPPORTED]: {:?}", decl)
        }
    }

    pub fn execute_ifelse(&mut self, condition: &Expression, then_branch: &Command, else_branch: &Command) -> () {
        let condition_result = self.execute_expression(condition);
        match condition_result {
            Value::Bool(true) => self.execute_command(then_branch),
            Value::Bool(false) => self.execute_command(else_branch),
            _ => panic!("Invalid type for IF-ELSE condition: {:?}", condition_result),
        }
    }

    pub fn execute_io(&mut self, io_command: &IOCommand) -> () {
        match io_command {
            IOCommand::Write(expr) => self.execute_write(expr),
            IOCommand::Read(var) => println!("<Read> Var: {:?}", var),
        }
    }

    pub fn execute_write(&mut self, expr: &Expression) -> () {
        println!("{}", self.execute_expression(expr));
    }

    pub fn execute_expression(&mut self, expr: &Expression) -> Value {
        match expr {
            Expression::ConcreteValue(value) => self.execute_concrete_value(value),
            Expression::Identifier(var) => {
                let env = self.env.borrow();
                let variable = env.lookup_variable(var).ok_or_else(|| format!("Variável '{}' não definida", var));
                match variable {
                    Ok(variable_info) => {
                        return variable_info.value;
                    },
                    Err(e) => {
                        panic!("{e}")
                    }
                }
            },
            Expression::UnaryExp(op, expr) => self.execute_unary_expression(op, expr),
            Expression::BinaryExp(left, op, right) => self.execute_binary_expression(left, op, right),
        }
    }

    pub fn execute_concrete_value(&mut self, value: &ConcreteValue) -> Value {
        match value {
            ConcreteValue::Value(value) => match value {
                Value::Int(value) => Value::Int(*value),
                Value::Bool(value) => Value::Bool(*value),
                Value::Str(value) => Value::Str(value.to_string()),
            }
        }
    }

    pub fn execute_unary_expression(&mut self, op: &UnaryOperator, expr: &Expression) -> Value {
        match op {
            UnaryOperator::Not => {
                match expr {
                    Expression::ConcreteValue(value) => {
                        match value {
                            ConcreteValue::Value(value) => match value {
                                Value::Bool(value) => Value::Bool(!value),
                                _ => panic!("Invalid type for NOT operator: {:?}", value),
                            }
                        }
                    },
                    _ => {
                        let result = self.execute_expression(expr);
                        match result {
                            Value::Bool(value) => Value::Bool(!value),
                            _ => panic!("Invalid type for NOT operator in expression: {:?}", result),
                        }
                    },
                }
            },
            UnaryOperator::Neg => {
                match expr {
                    Expression::ConcreteValue(value) => {
                        match value {
                            ConcreteValue::Value(value) => match value {
                                Value::Int(value) => Value::Int(-value),
                                _ => panic!("Invalid type for NEG operator: {:?}", value),
                            }
                        }
                    },
                    _ => {
                        let result = self.execute_expression(expr);
                        match result {
                            Value::Int(value) => Value::Int(-value),
                            _ => panic!("Invalid type for NEG operator in expression: {:?}", result),
                        }
                    }
                }
            },
            UnaryOperator::Length => {
                match expr {
                    Expression::ConcreteValue(value) => {
                        match value {
                            ConcreteValue::Value(value) => match value {
                                Value::Str(value) => Value::Int(value.len().try_into().unwrap()),
                                _ => panic!("Invalid type for LENGTH operator: {:?}", value),
                            }
                        }
                    },
                    _ => {
                        let result = self.execute_expression(expr);
                        match result {
                            Value::Str(value) => Value::Int(value.len().try_into().unwrap()),
                            _ => panic!("Invalid type for LENGTH operator in expression: {:?}", result),
                        }
                    }
                }
            },
        }
    }

    pub fn execute_binary_expression(&mut self, op: &BinaryOperator, left: &Expression, right: &Expression) -> Value {
        match op {
            BinaryOperator::Add => {
                let left = self.execute_expression(left);
                let right = self.execute_expression(right);
                match (left.clone(), right.clone()) {
                    (Value::Int(left), Value::Int(right)) => Value::Int(left + right),
                    (Value::Str(left), Value::Str(right)) => Value::Str(format!("{}{}", left, right)),
                    _ => panic!("Invalid types for ADD operator: {:?} and {:?}", left, right),
                }
            },
            BinaryOperator::Sub => {
                let left = self.execute_expression(left);
                let right = self.execute_expression(right);
                match (left.clone(), right.clone()) {
                    (Value::Int(left), Value::Int(right)) => Value::Int(left - right),
                    _ => panic!("Invalid types for SUB operator: {:?} and {:?}", left, right),
                }
            },
            BinaryOperator::Equal => {
                let left = self.execute_expression(left);
                let right = self.execute_expression(right);
                match (left.clone(), right.clone()) {
                    (Value::Int(left), Value::Int(right)) => Value::Bool(left == right),
                    (Value::Str(left), Value::Str(right)) => Value::Bool(left == right),
                    (Value::Bool(left), Value::Bool(right)) => Value::Bool(left == right),
                    _ => panic!("Invalid types for EQUAL operator: {:?} and {:?}", left, right),
                }
            },
            BinaryOperator::And => {
                let left = self.execute_expression(left);
                let right = self.execute_expression(right);
                match (left.clone(), right.clone()) {
                    (Value::Bool(left), Value::Bool(right)) => Value::Bool(left && right),
                    _ => panic!("Invalid types for AND operator: {:?} and {:?}", left, right),
                }
            },
            BinaryOperator::Or => {
                let left = self.execute_expression(left);
                let right = self.execute_expression(right);
                match (left.clone(), right.clone()) {
                    (Value::Bool(left), Value::Bool(right)) => Value::Bool(left || right),
                    _ => panic!("Invalid types for OR operator: {:?} and {:?}", left, right),
                }
            },
            BinaryOperator::Concat => {
                let left = self.execute_expression(left);
                let right = self.execute_expression(right);
                match (left.clone(), right.clone()) {
                    (Value::Str(left), Value::Str(right)) => Value::Str(format!("{}{}", left, right)),
                    _ => panic!("Invalid types for CONCAT operator: {:?} and {:?}", left, right),
                }
            },
            BinaryOperator::Less => {
                let left = self.execute_expression(left);
                let right = self.execute_expression(right);
                match (left.clone(), right.clone()) {
                    (Value::Int(left), Value::Int(right)) => Value::Bool(left < right),
                    _ => panic!("Invalid types for LESS operator: {:?} and {:?}", left, right),
                }
            },
            BinaryOperator::LessEqual => {
                let left = self.execute_expression(left);
                let right = self.execute_expression(right);
                match (left.clone(), right.clone()) {
                    (Value::Int(left), Value::Int(right)) => Value::Bool(left <= right),
                    _ => panic!("Invalid types for LESS_EQUAL operator: {:?} and {:?}", left, right),
                }
            },
            BinaryOperator::Greater => {
                let left = self.execute_expression(left);
                let right = self.execute_expression(right);
                match (left.clone(), right.clone()) {
                    (Value::Int(left), Value::Int(right)) => Value::Bool(left > right),
                    _ => panic!("Invalid types for GREATER operator: {:?} and {:?}", left, right),
                }
            },
            BinaryOperator::GreaterEqual => {
                let left = self.execute_expression(left);
                let right = self.execute_expression(right);
                match (left.clone(), right.clone()) {
                    (Value::Int(left), Value::Int(right)) => Value::Bool(left >= right),
                    _ => panic!("Invalid types for GREATER_EQUAL operator: {:?} and {:?}", left, right),
                }
            },
        }
    }
}
