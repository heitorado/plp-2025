use crate::ast::Program;
use nom::{IResult, Parser, combinator::map};

use crate::parsers::command_parsers::parse_command;

// Parser do programa
pub fn parse_program(input: &str) -> IResult<&str, Program> {
    map(parse_command, |cmd| Program::Command(cmd)).parse(input)
}
