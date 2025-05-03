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
            Command::IfElse(condition, then_branch, else_branch) => println!("<IfElse> Condition: {:?} | Then: {:?} | Else: {:?}", condition, then_branch, else_branch),//self.execute_if_else(condition, then_branch, else_branch),
            Command::IO(io_command) => self.execute_io(io_command),
            Command::Sequence(left, right) => println!("<Sequence> Left: {:?} | Right: {:?}", left, right),//self.execute_sequence(left, right),
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

    pub fn execute_io(&self, io_command: &IOCommand) -> () {
        match io_command {
            IOCommand::Write(expr) => self.execute_write(expr),
            IOCommand::Read(var) => println!("<Read> Var: {:?}", var),
        }
    }

    // pub fn execute_write(&self, expr: &Expression) -> () {
    //     match expr {
    //         Expression::ConcreteValue(value) => self.write_concrete_value(value),
    //         Expression::Identifier(var) => println!("<Write> Var: {:?}", var),
    //         Expression::UnaryExp(op, expr) => self.execute_write(&self.execute_unary_expression(op, expr)),
    //         Expression::BinaryExp(left, op, right) => println!("<Write> BinaryExp: {:?} | Left: {:?} | Right: {:?}", op, left, right),
    //     }
    // }

    pub fn execute_write(&self, expr: &Expression) -> () {
        println!("{}", self.execute_expression(expr));
    }

    // pub fn write_concrete_value(&self, value: &ConcreteValue) -> () {
    //     match value {
    //         ConcreteValue::Value(value) => match value {
    //             Value::Int(value) => println!("{}", value),
    //             Value::Bool(value) => println!("{}", value),
    //             Value::Str(value) => println!("{}", value),
    //         }
    //     }
    // }

    pub fn execute_expression(&self, expr: &Expression) -> Value {
        match expr {
            Expression::ConcreteValue(value) => self.execute_concrete_value(value),
            Expression::Identifier(var) => format!("<Identifier> Var: {:?}", var),
            Expression::UnaryExp(op, expr) => self.execute_unary_expression(op, expr),
            Expression::BinaryExp(left, op, right) => format!("<BinaryExp> Op: {:?} | Left: {:?} | Right: {:?}", op, left, right),
        }
    }

    pub fn execute_concrete_value(&self, value: &ConcreteValue) -> String {
        match value {
            ConcreteValue::Value(value) => match value {
                Value::Int(value) => format!("{}", value),
                Value::Bool(value) => format!("{}", value),
                Value::Str(value) => format!("{}", value),
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
                    _ => !self.execute_expression(expr),
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
                    _ => self.execute_expression(expr),
                }
            },
            UnaryOperator::Length => {
                match expr {
                    Expression::ConcreteValue(value) => {
                        match value {
                            ConcreteValue::Value(value) => match value {
                                Value::Str(value) => Value::Int(value.len()),
                                _ => panic!("Invalid type for LENGTH operator: {:?}", value),
                            }
                        }
                    },
                    _ => self.execute_expression(expr),
                }
            },
        }
    }

}
