use crate::ast::Program;
use nom::{IResult, Parser, combinator::map};

use crate::parsers::command_parsers::parse_command;

// Parser do programa
pub fn parse_program(input: &str) -> IResult<&str, Program> {
    // Se desejar, você pode delimitar com espaços ou outros tokens no início e fim.
    // Aqui usamos `map` para transformar o resultado do parse_command em um Program.
    map(parse_command, |cmd| Program::Command(cmd)).parse(input)
}
