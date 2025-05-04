use crate::ast::*;

#[derive(Debug, Clone)]
pub struct Executor {}

impl Executor {
    pub fn new() -> Self {
        Executor {}
    }

    pub fn execute_program(&self, program: &Program) -> () {
        match program {
            Program::Command(cmd) => self.execute_command(cmd),
        }
    }

    pub fn execute_command(&self, cmd: &Command) -> () {
        match cmd {
            Command::Assignment(var, expr, is_move) => println!("<Assignment> Var: {:?} | Expr: {:?} | IsMove: {:?}", var, expr, is_move),//self.execute_assignment(var, expr, *is_move),
            Command::DeclarationBlock(decls, body) => self.execute_declaration_block(decls, body),
            Command::WhileLoop(condition, body) => println!("<WhileLoop> Condition: {:?} | Body: {:?}", condition, body),//self.execute_while_loop(condition, body),
            Command::IfElse(condition, then_branch, else_branch) => self.execute_ifelse(condition, then_branch, else_branch),
            Command::IO(io_command) => self.execute_io(io_command),
            Command::Sequence(left, right) => { self.execute_command(left); self.execute_command(right); }
            Command::Skip => {},
            Command::CallProcedure(proc_name) => println!("<CallProcedure> ProcName: {:?}", proc_name),//self.execute_call_procedure(proc_name, args),
        }
    }

    pub fn execute_declaration_block(&self, decls: &Vec<Declaration>, body: &Command) -> () {
        println!("<DeclarationBlock> Decls: {:?}", decls);
        // TODO: execute declaration block here...
        println!("<DeclarationBlock> Body: {:?}", body);
        self.execute_command(body);
    }

    pub fn execute_ifelse(&self, condition: &Expression, then_branch: &Command, else_branch: &Command) -> () {
        let condition_result = self.execute_expression(condition);
        match condition_result {
            Value::Bool(true) => self.execute_command(then_branch),
            Value::Bool(false) => self.execute_command(else_branch),
            _ => panic!("Invalid type for IF-ELSE condition: {:?}", condition_result),
        }
    }

    pub fn execute_io(&self, io_command: &IOCommand) -> () {
        match io_command {
            IOCommand::Write(expr) => self.execute_write(expr),
            IOCommand::Read(var) => println!("<Read> Var: {:?}", var),
        }
    }

    pub fn execute_write(&self, expr: &Expression) -> () {
        println!("{}", self.execute_expression(expr));
    }

    pub fn execute_expression(&self, expr: &Expression) -> Value {
        match expr {
            Expression::ConcreteValue(value) => self.execute_concrete_value(value),
            Expression::Identifier(var) => Value::Str(format!("<Identifier> Var: {:?}", var)),
            Expression::UnaryExp(op, expr) => self.execute_unary_expression(op, expr),
            Expression::BinaryExp(left, op, right) => self.execute_binary_expression(left, op, right),
        }
    }

    pub fn execute_concrete_value(&self, value: &ConcreteValue) -> Value {
        match value {
            ConcreteValue::Value(value) => match value {
                Value::Int(value) => Value::Int(*value),
                Value::Bool(value) => Value::Bool(*value),
                Value::Str(value) => Value::Str(value.to_string()),
            }
        }
    }

    pub fn execute_unary_expression(&self, op: &UnaryOperator, expr: &Expression) -> Value {
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

    pub fn execute_binary_expression(&self, op: &BinaryOperator, left: &Expression, right: &Expression) -> Value {
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
