use crate::ast::{Command, IOCommand};
use crate::parsers::basic_parsers::{parse_identifier, ws};
use crate::parsers::declaration_parsers::parse_declaration;
use crate::parsers::expression_parsers::parse_expression;
use nom::sequence::terminated;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt, value},
    multi::separated_list1,
    sequence::{delimited, preceded},
};
// Parser principal
pub fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, cmd) = alt((
        parse_assignment,
        parse_declaration_block,
        parse_if_else,
        parse_while_loop,
        parse_io_command,
        parse_skip,
    ))
    .parse(input)?;

    parse_sequence(input, cmd)
}

// Helper para sequências
fn parse_sequence(input: &str, left: Command) -> IResult<&str, Command> {
    opt(preceded(delimited(ws, tag(";"), ws), parse_command))
        .map(|maybe_right| {
            maybe_right
                .map(|right| Command::Sequence(Box::new(left.clone()), Box::new(right)))
                .unwrap_or(left.clone())
        })
        .parse(input)
}

// Atribuição: x := 5
fn parse_assignment(input: &str) -> IResult<&str, Command> {
    map(
        (
            delimited(ws, parse_identifier, ws),
            delimited(ws, tag(":="), ws),
            delimited(ws, parse_expression, ws),
        ),
        |(var, _, expr)| Command::Assignment(var, expr),
    )
    .parse(input)
}

// Bloco de declarações: { var x = 5; var y = 10 }
fn parse_declaration_block(input: &str) -> IResult<&str, Command> {
    map(
        delimited(
            delimited(ws, tag("{"), ws),
            separated_list1(delimited(ws, tag(";"), ws), parse_declaration),
            terminated(tag("}"), ws),
        ),
        Command::DeclarationBlock,
    )
    .parse(input)
}

// While loop: while cond do cmd
fn parse_while_loop(input: &str) -> IResult<&str, Command> {
    map(
        (
            preceded(
                terminated(tag("while"), ws),
                delimited(ws, parse_expression, ws),
            ),
            preceded(terminated(tag("do"), ws), parse_command),
        ),
        |(cond, body)| Command::WhileLoop(cond, Box::new(body)),
    )
    .parse(input)
}

// If-else: if cond then cmd1 else cmd2
fn parse_if_else(input: &str) -> IResult<&str, Command> {
    map(
        (
            preceded(
                terminated(tag("if"), ws),
                delimited(ws, parse_expression, ws),
            ),
            preceded(terminated(tag("then"), ws), parse_command),
            preceded(terminated(tag("else"), ws), parse_command),
        ),
        |(cond, then_cmd, else_cmd)| Command::IfElse(cond, Box::new(then_cmd), Box::new(else_cmd)),
    )
    .parse(input)
}

// Comandos de IO: write(expr) ou read(var)
fn parse_io_command(input: &str) -> IResult<&str, Command> {
    alt((
        map(
            preceded(
                tag("write"),
                delimited(tag("("), parse_expression, tag(")")),
            ),
            |expr| Command::IO(IOCommand::Write(Box::new(expr))),
        ),
        map(
            preceded(tag("read"), delimited(tag("("), parse_identifier, tag(")"))),
            |var| Command::IO(IOCommand::Read(var)),
        ),
    ))
    .parse(input)
}

// Comando skip
fn parse_skip(input: &str) -> IResult<&str, Command> {
    value(Command::Skip, tag("skip")).parse(input)
}
