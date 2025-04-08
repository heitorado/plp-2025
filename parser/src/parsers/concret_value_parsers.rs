use crate::ast::{ConcretValue, Value};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::digit1,
    combinator::{map, value},
    sequence::delimited,
};

// Parse Int
pub fn parse_int(input: &str) -> IResult<&str, ConcretValue> {
    let (input, converted_int_str) = digit1(input)?; // Reconhece os Números

    let num = converted_int_str.parse::<i64>().map_err(|_| {
        // Converte para i64
        nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
    })?;

    Ok((input, ConcretValue::Value(Value::Int(num))))
}

// Parse String
pub fn parse_string(input: &str) -> IResult<&str, ConcretValue> {
    delimited(
        tag("\""),
        map(take_while(|c: char| c != '"'), |s: &str| {
            ConcretValue::Value(Value::Str(s.to_string()))
        }),
        tag("\""),
    )
    .parse(input)
}

// Parse Bool
pub fn parse_bool(input: &str) -> IResult<&str, ConcretValue> {
    alt((
        value(ConcretValue::Value(Value::Bool(true)), tag("true")),
        value(ConcretValue::Value(Value::Bool(false)), tag("false")),
    ))
    .parse(input)
}
