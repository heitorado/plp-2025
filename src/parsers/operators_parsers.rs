use crate::ast::{BinaryOperator, UnaryOperator};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    sequence::delimited,
};

use crate::parsers::basic_parsers::ws;

// Binario
pub fn precedence(op: &BinaryOperator) -> u8 {
    match op {
        BinaryOperator::Concat => 5,
        BinaryOperator::Add | BinaryOperator::Sub => 3,
        BinaryOperator::Equal
        | BinaryOperator::Less
        | BinaryOperator::LessEqual
        | BinaryOperator::Greater
        | BinaryOperator::GreaterEqual => 2,
        BinaryOperator::And => 1,
        BinaryOperator::Or => 0,
    }
}

pub fn parse_binary_operator(input: &str) -> IResult<&str, BinaryOperator> {
    delimited(
        ws,
        alt((
            value(BinaryOperator::Concat, tag("++")),
            value(BinaryOperator::Add, tag("+")),
            value(BinaryOperator::Sub, tag("-")),
            value(BinaryOperator::And, tag("and")),
            value(BinaryOperator::Or, tag("or")),
            value(BinaryOperator::Equal, tag("==")),
            value(BinaryOperator::LessEqual, tag("<=")),
            value(BinaryOperator::GreaterEqual, tag(">=")),
            value(BinaryOperator::Less, tag("<")),
            value(BinaryOperator::Greater, tag(">")),
        )),
        ws,
    )
    .parse(input)
}

// Unários
pub fn parse_unary_operator(input: &str) -> IResult<&str, UnaryOperator> {
    delimited(
        ws,
        alt((
            map(tag("-"), |_| UnaryOperator::Neg),
            map(tag("not"), |_| UnaryOperator::Not),
            map(tag("length"), |_| UnaryOperator::Length),
        )),
        ws,
    )
    .parse(input)
}
