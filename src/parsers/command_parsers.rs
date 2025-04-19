use crate::ast::{CallProcedure, Command, IOCommand};
use crate::parsers::basic_parsers::{parse_identifier, ws};
use crate::parsers::declaration_parsers::parse_declaration;
use crate::parsers::expression_parsers::parse_expression;
use nom::multi::{many0, separated_list1};
use nom::sequence::{pair, terminated};
use nom::{
    IResult,
    Parser,
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt, value},
    // multi::separated_list1,
    sequence::{delimited, preceded},
};

use super::expression_parsers::parse_expression_atomic;
// Parser principal
pub fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, cmd) = alt((
        parse_assignment,
        parse_if_else,
        parse_while_loop,
        parse_io_command,
        parse_skip,
        parse_declaration_block,
        parse_call_procedure,
    ))
    .parse(input)?;

    parse_sequence(input, cmd)
}

// Parse command helper with another order of parsers to help the parse_declaration_block to processo complex code
fn parse_command_with_sequence(input: &str) -> IResult<&str, Command> {
    let (input, initial_cmd) = alt((
        parse_if_else,
        parse_while_loop,
        parse_assignment,
        parse_io_command,
        parse_skip,
        parse_declaration_block,
        parse_call_procedure,
    ))
    .parse(input)?;

    parse_sequence(input, initial_cmd)
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
            delimited(ws, opt(tag("move")), ws),
            delimited(ws, parse_expression, ws),
        ),
        |(var, _, is_move, expr)| Command::Assignment(var, expr, is_move.is_some()),
    )
    .parse(input)
}

// Bloco de declarações: { var x = 5; var y = 10 }
fn parse_declaration_block(input: &str) -> IResult<&str, Command> {
    map(
        delimited(
            delimited(ws, tag("{"), ws),
            pair(
                // Declarações seguidas de ;
                many0(terminated(parse_declaration, delimited(ws, tag(";"), ws))),
                // Sequência de comandos (usando a mesma lógica do parse_command)
                parse_command_with_sequence,
            ),
            delimited(ws, tag("}"), ws),
        ),
        |(declarations, command)| Command::DeclarationBlock(declarations, Box::new(command)),
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
            preceded(tag("if"), delimited(ws, parse_expression, ws)),
            preceded(tag("then"), delimited(ws, parse_command, ws)),
            preceded(tag("else"), delimited(ws, parse_command, ws)),
        ),
        |(cond, then_cmd, else_cmd)| Command::IfElse(cond, Box::new(then_cmd), Box::new(else_cmd)),
    )
    .parse(input)
}

// Comandos de IO: write(expr) ou read(var)
fn parse_io_command(input: &str) -> IResult<&str, Command> {
    alt((
        map(
            delimited(
                ws,
                preceded(
                    tag("write"),
                    delimited(tag("("), parse_expression, tag(")")),
                ),
                ws,
            ),
            |expr| Command::IO(IOCommand::Write(Box::new(expr))),
        ),
        map(
            delimited(
                ws,
                preceded(tag("read"), delimited(tag("("), parse_identifier, tag(")"))),
                ws,
            ),
            |var| Command::IO(IOCommand::Read(var)),
        ),
    ))
    .parse(input)
}

// Comando skip
fn parse_skip(input: &str) -> IResult<&str, Command> {
    value(Command::Skip, tag("skip")).parse(input)
}

pub fn parse_call_procedure(input: &str) -> IResult<&str, Command> {
    let (input, _) = delimited(ws, tag("call"), ws).parse(input)?;
    let (input, id) = parse_identifier(input)?;
    let (input, _) = delimited(ws, tag("("), ws).parse(input)?;
    let (input, exps) = opt(separated_list1(
        delimited(ws, tag(","), ws),
        parse_expression_atomic,
    ))
    .parse(input)?;
    let (input, _) = delimited(ws, tag(")"), ws).parse(input)?;

    Ok((
        input,
        Command::CallProcedure(CallProcedure {
            id,
            args: exps.unwrap_or_default(),
        }),
    ))
}
