use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, multispace0},
    combinator::{map, recognize},
    sequence::pair,
};

use crate::ast::Type;

//// Parser para espaços em branco
pub fn ws(input: &str) -> IResult<&str, &str> {
    multispace0(input)
}

// Parser para espaços em branco e parenteses
pub fn ws_paren(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_whitespace() || c == '(' || c == ')').parse(input)
}

// Parse para identificador
pub fn parse_identifier(input: &str) -> IResult<&str, String> {
    let mut parser = recognize(pair(
        alt((alpha1, tag("_"))),
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
    ));

    let (input, matched_str) = parser.parse(input)?;
    Ok((input, matched_str.to_string()))
}

pub fn parse_type(input: &str) -> IResult<&str, Type> {
    map(
        alt((
            tag("int"),
            tag("string"),
            tag("bool"),
        )),
        |matched_str| match matched_str {
            "int" => crate::ast::Type::Int,
            "string" => crate::ast::Type::Str,
            "bool" => crate::ast::Type::Bool,
            _ => unreachable!(),
        },
    )
    .parse(input)
}
