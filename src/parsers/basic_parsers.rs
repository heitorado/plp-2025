use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, multispace0},
    combinator::recognize,
    sequence::pair,
};

//// Parser para espaços em branco
pub fn ws(input: &str) -> IResult<&str, &str> {
    multispace0(input)
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
