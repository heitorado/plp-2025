use crate::ast::Expression;
use crate::parsers::basic_parsers::{parse_identifier, ws, ws_paren};
use crate::parsers::concret_value_parsers::{parse_bool, parse_int, parse_string};
use crate::parsers::operators_parsers::{parse_binary_operator, parse_unary_operator, precedence};
use nom::sequence::preceded;
use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag, combinator::map, sequence::delimited,
};

pub fn parse_concrete_value(input: &str) -> IResult<&str, Expression> {
    delimited(
        ws,
        alt((
            map(parse_int, Expression::ConcreteValue),
            map(parse_bool, Expression::ConcreteValue),
            map(parse_string, Expression::ConcreteValue),
        )),
        ws,
    )
    .parse(input)
}

// Depois implemento
pub fn parse_parenthesized(input: &str) -> IResult<&str, Expression> {
    delimited(tag("("), delimited(ws, parse_expression, ws), tag(")")).parse(input)
}

pub fn parse_expression_atomic(input: &str) -> IResult<&str, Expression> {
    delimited(
        ws,
        alt((
            parse_concrete_value,
            map(parse_identifier, Expression::Identifier),
            parse_parenthesized,
        )),
        ws,
    )
    .parse(input)
}

// Parser principal
pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    // let (input, _) = parse_parenthesized(input)?;
    parse_expr_bp(input, 0)
}

// Função recursiva para pegar blocos diferentes de expressoes. Ex: 5 + 10 = 15 = ((5 + 10) = 15)
pub fn parse_expr_bp(input: &str, min_prec: u8) -> IResult<&str, Expression> {
    let (mut input, mut lhs) = parse_primary(input)?;

    while let Ok((remaining, op)) = parse_binary_operator(input) {
        let prec = precedence(&op);
        if prec < min_prec {
            break;
        }

        let (new_input, rhs) = parse_expr_bp(remaining, prec + 1)?;
        lhs = Expression::BinaryExp(op, Box::new(lhs), Box::new(rhs));
        input = new_input;
    }

    Ok((input, lhs))
}

// Primary Expressions
pub fn parse_primary(input: &str) -> IResult<&str, Expression> {
    alt((
        parse_length_expression,
        delimited(ws, parse_unary_expression, ws),
        parse_expression_atomic,
    ))
    .parse(input)
}

fn parse_length_expression(input: &str) -> IResult<&str, Expression> {
    map(
        preceded(
            tag("length"),
            delimited(ws_paren, parse_expression, ws_paren),
        ),
        |expr| Expression::UnaryExp(crate::ast::UnaryOperator::Length, Box::new(expr)),
    )
    .parse(input)
}

// Parser unário
pub fn parse_unary_expression(input: &str) -> IResult<&str, Expression> {
    let (input, op) = parse_unary_operator(input)?;
    let (input, exp) = parse_expr_bp(input, 100)?; // Alta precedência

    Ok((input, Expression::UnaryExp(op, Box::new(exp))))
}

// Primeiro teste
// Não passa para exemplos do tipo: 5 + 10 = 15 = ((5 + 10) = 15)

// // Parser para expressões unárias
// pub fn parse_unary_expression(input: &str) -> IResult<&str, Expression> {
//     let (input, op) = parse_unary_operator(input)?;
//     let (input, exp) = parse_expression_atomic(input)?;

//     Ok((input, Expression::UnaryExp(op, Box::new(exp))))
// }

// // Parser para expressões binarias
// pub fn parse_binary_expression(input: &str) -> IResult<&str, Expression> {
//     let (input, left) = parse_expression_atomic(input)?;
//     let (input, op) = parse_binary_operator(input)?;
//     let (input, right) = parse_expression_atomic(input)?;

//     Ok((
//         input,
//         Expression::BinaryExp(op, Box::new(left), Box::new(right)),
//     ))
// }

// // Parser para expressões
// pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
//     let (input, expr) = delimited(
//         ws,
//         alt((
//             parse_binary_expression,
//             parse_expression_atomic,
//             parse_unary_expression,
//         )),
//         ws,
//     )
//     .parse(input)?;

//     Ok((input, expr))
// }
