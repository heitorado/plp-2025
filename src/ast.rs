// Core (Programa)
#[derive(Debug, Clone)]
pub enum Program {
    Command(Command),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    // Atribuição de valor
    Assignment(String, Expression, bool), // x := 5
    // Bloco de definições { x := x + 5 }
    DeclarationBlock(Vec<Declaration>, Box<Command>),
    // Expression (Condicional do Loop), Box<Command> (Corpo do Loop)
    WhileLoop(Expression, Box<Command>), // while
    // Expression (Condicional do If), Box<Command> (Corpo do If), Option<Box<Command>> (Corpo do Else)
    IfElse(Expression, Box<Command>, Box<Command>), // if ... then ... else ...
    // Entrada/Saída
    IO(IOCommand), // write(...) or read(...)
    // Sequência de comandos
    Sequence(Box<Command>, Box<Command>), // c1; c2
    Skip,
    CallProcedure(CallProcedure),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
    // Definição de variável única
    Variable(String, Expression, bool), // var x = 5
    // Definição de variável separadas por virgula.
    Procedure(String, Vec<ProcedureParameter>, Box<Command>),
    Compound(Box<Declaration>, Box<Declaration>), // var x = 5; var y = 10;
                                                  // MELHORIAS FUTURAS
                                                  // Definição de varias variáveis
                                                  // CompoundDeclaration(Vec<Box<Declaration>>), // var x = 5; var y = 10;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    // Valores literais
    ConcreteValue(ConcreteValue), // 5, true, "string"
    // Nome das variáveis
    Identifier(String), // x
    // Expressões unarias
    // Neg, Length e expressão -x -> x
    UnaryExp(UnaryOperator, Box<Expression>), // -x, not y
    // Expressões Binárias
    // BinaryOperator -> Tipo da expressão
    BinaryExp(BinaryOperator, Box<Expression>, Box<Expression>), // x + y, x - y, x == y

                                                                 // ProcedureCall(String, Box<ExpressionList>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperator {
    Neg,    // Negação
    Not,    // Negação lógica
    Length, // Tamanho da string/lista
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOperator {
    // Aritméticos
    Add, // +
    Sub, // -

    // Comparação
    Equal, // ==

    // Lógicos
    And, // &&
    Or,  // ||

    // Concat
    Concat, // ++

    // Menor, Maior que, Menor ou igual que, Maior ou igual que
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IOCommand {
    // Entrada
    Read(String), // read x
    // Saída
    Write(Box<Expression>), // write x
}

// O professor ainda não disse o porque desse valor concreto.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConcreteValue {
    // Valor
    Value(Value),
}

// Valor Real
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    // Inteiro
    Int(i64),
    // String
    Str(String),
    // Booleano
    Bool(bool),
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum ExpressionList {
//     // Lista de expressões
//     Expression(Box<Expression>),
//     Compound(Box<Expression>, Box<ExpressionList>),
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallProcedure {
    pub id: String,
    pub args: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcedureParameter {
    pub identifier: String,
    pub r#type: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Str,
    Bool,
}
