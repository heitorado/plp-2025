// Core (Programa)
#[derive(Debug, Clone)]
pub enum Program {
    Command(Command),
}

#[derive(Debug, Clone)]
pub enum Command {
    // Atribuição de valor
    Assignment(String, Expression), // x := 5
    // Bloco de definições
    DeclarationBlock(Vec<Declaration>, Box<Command>),
    // Expression (Condicional do Loop), Box<Command> (Corpo do Loop)       // { var x = 10; ... }
    WhileLoop(Expression, Box<Command>), // while
    // Expression (Condicional do If), Box<Command> (Corpo do If), Option<Box<Command>> (Corpo do Else)
    IfElse(Expression, Box<Command>, Option<Box<Command>>), // if ... then ... else ...
    // Entrada/Saída
    IO(IOCommand), // write(...) or read(...)
    // Sequência de comandos
    Sequence(Box<Command>, Box<Command>), // c1; c2
    Skip,                                 // no operation
}

#[derive(Debug, Clone)]
pub enum Declaration {
    // Definição de variável única
    Variable(String, Expression), // var x = 5
    // Definição de variável separadas por virgula.
    Compound(Box<Declaration>, Box<Declaration>), // var x = 5; var y = 10;
                                                  // MELHORIAS FUTURAS
                                                  // Definição de varias variáveis
                                                  // CompoundDeclaration(Vec<Box<Declaration>>), // var x = 5; var y = 10;
}

#[derive(Debug, Clone)]
pub enum Expression {
    // Valores literais
    ConcretValue(ConcretValue), // 5, true, "string"
    // Nome das variáveis
    Identifier(String), // x
    // Expressões unarias
    // Neg, Length e expressão -x -> x
    UnaryExp(UnatyOperator, Box<Expression>), // -x, not y
    // Expressões Binárias
    // BinaryOperator -> Tipo da expressão
    BinaryExp(BinaryOperator, Box<Expression>, Box<Expression>), // x + y, x * y
}

#[derive(Debug, Clone)]
pub enum UnatyOperator {
    Neg,    // Negação
    Not,    // Negação lógica
    Length, // Tamanho da string/lista
}

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone)]
pub enum IOCommand {
    // Entrada
    Read(String), // read x
    // Saída
    Write(Box<Expression>), // write x
}

// O professor ainda não disse o porque desse valor concreto.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConcretValue {
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
