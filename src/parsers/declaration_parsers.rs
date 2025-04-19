use crate::ast::Declaration;
use crate::ast::ProcedureParameter;
use crate::parsers::basic_parsers::{parse_identifier, parse_type, ws};
use crate::parsers::command_parsers::parse_command;
use crate::parsers::expression_parsers::parse_expression;
use nom::Parser;
use nom::branch::alt;
use nom::combinator::opt;
use nom::{
    IResult, bytes::complete::tag, combinator::map, multi::separated_list1, sequence::delimited,
};
// Parser principal para declarações
pub fn parse_declaration(input: &str) -> IResult<&str, Declaration> {
    let (input, declarations) = separated_list1(
        delimited(ws, tag(","), ws),
        alt((parse_single_declaration, parse_procedure_declaration)),
    )
    .parse(input)?;

    let combined = declarations
        .into_iter()
        .reduce(|acc, decl| Declaration::Compound(Box::new(acc), Box::new(decl)))
        .expect("Pelo menos uma declaração");

    Ok((input, combined))
}

// Parser para uma única declaração
fn parse_single_declaration(input: &str) -> IResult<&str, Declaration> {
    map(
        (
            tag("var"),
            ws,
            parse_identifier,
            ws,
            tag("="),
            ws,
            alt((
                map((tag("move"), ws, parse_expression), |(_, _, expr)| {
                    (expr, true)
                }),
                map(parse_expression, |expr| (expr, false)),
            )),
        ),
        |(_, _, name, _, _, _, (expr, is_move))| Declaration::Variable(name, expr, is_move),
    )
    .parse(input)
}

pub fn parse_procedure_parameter(input: &str) -> IResult<&str, ProcedureParameter> {
    map(
        (parse_type, tag(" "), parse_identifier),
        |(type_name, _, identifier_name)| ProcedureParameter {
            identifier: identifier_name.to_string(),
            r#type: type_name,
        },
    )
    .parse(input)
}

pub fn parse_procedure_parameters(input: &str) -> IResult<&str, Vec<ProcedureParameter>> {
    separated_list1(delimited(ws, tag(","), ws), parse_procedure_parameter).parse(input)
}

pub fn parse_procedure_declaration(input: &str) -> IResult<&str, Declaration> {
    map(
        (
            tag("proc"),
            delimited(ws, parse_identifier, ws),
            delimited(
                tag("("),
                delimited(ws, opt(parse_procedure_parameters), ws),
                tag(")"),
            ),
            ws,
            delimited(tag("{"), delimited(ws, parse_command, ws), tag("}")),
        ),
        |(_, name, parameters, _, body)| {
            Declaration::Procedure(name, parameters.unwrap_or_default(), Box::new(body))
        },
    )
    .parse(input)
}
