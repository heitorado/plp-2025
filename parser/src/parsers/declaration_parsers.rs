use crate::ast::Declaration;
use crate::parsers::basic_parsers::{parse_identifier, ws};
use crate::parsers::expression_parsers::parse_expression;
use nom::Parser;
use nom::{
    IResult, bytes::complete::tag, combinator::map, multi::separated_list1, sequence::delimited,
};
// Parser principal para declarações
pub fn parse_declaration(input: &str) -> IResult<&str, Declaration> {
    let (input, declarations) =
        separated_list1(delimited(ws, tag(";"), ws), parse_single_declaration).parse(input)?;

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
            parse_expression,
        ),
        |(_, _, name, _, _, _, expr)| Declaration::Variable(name, expr),
    )
    .parse(input)
}
