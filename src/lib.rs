pub mod ast;

use ast::{ConcretValue, Value};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, digit1, multispace0},
    combinator::recognize,
    sequence::pair,
};

//// Parser para espaços em branco
pub fn ws(input: &str) -> IResult<&str, &str> {
    multispace0(input)
}

pub fn identifier(input: &str) -> IResult<&str, String> {
    let mut parser = recognize(pair(
        alt((alpha1, tag("_"))),
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
    ));

    let (input, matched_str) = parser.parse(input)?;
    Ok((input, matched_str.to_string()))

    // Não funciona, o recognize cria uma implementação e não pode ser passado diretamente para o map sem utilizar o .parse
    // map(
    //     recognize(pair(
    //         alt((alpha1, tag("_"))),
    //         take_while(|c: char| c.is_alphanumeric() || c == '_'),
    //     )),
    //     |s: &str| s.to_string(),
    // )(input)
}

// PARSERS PARA VALORES CONCRETOS

// INT
// Tomar cuidado na hora de importar os arquivos das libs, autocomplete é troll e importa o modulo e não a função
pub fn parse_int(input: &str) -> IResult<&str, ConcretValue> {
    // Reconhece os números
    let (input, converted_int_str) = digit1(input)?;

    // Converse os valores para I64
    let num = converted_int_str.parse::<i64>().map_err(|_| {
        nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
    })?;

    Ok((input, ConcretValue::Value(Value::Int(num))))
    // map(digit1, |s: &str| {
    //     ConcretValue::Value(Value::Int(s.parse().unwrap()))
    // })
    // .parse(input);
}
