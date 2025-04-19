use crate::ast::{ConcreteValue, Value};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::digit1,
    combinator::{map, value},
    sequence::delimited,
};

// Parse Int
pub fn parse_int(input: &str) -> IResult<&str, ConcreteValue> {
    let (input, converted_int_str) = digit1(input)?; // Reconhece os Números

    let num = converted_int_str.parse::<i64>().map_err(|_| {
        // Converte para i64
        nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
    })?;

    Ok((input, ConcreteValue::Value(Value::Int(num))))
}

// Parse String
pub fn parse_string(input: &str) -> IResult<&str, ConcreteValue> {
    delimited(
        tag("\""),
        map(take_while(|c: char| c != '"'), |s: &str| {
            ConcreteValue::Value(Value::Str(s.to_string()))
        }),
        tag("\""),
    )
    .parse(input)
}

// Parse Bool
pub fn parse_bool(input: &str) -> IResult<&str, ConcreteValue> {
    alt((
        value(ConcreteValue::Value(Value::Bool(true)), tag("true")),
        value(ConcreteValue::Value(Value::Bool(false)), tag("false")),
    ))
    .parse(input)
}
