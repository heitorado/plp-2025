pub mod ast;

use ast::{
    BinaryOperator, Command, ConcretValue, Declaration, Expression, Program, UnaryOperator, Value,
};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, digit1, multispace0},
    combinator::{map, recognize, value},
    multi::separated_list0,
    sequence::{delimited, pair, terminated},
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
// PARSER PARA LINGUAGEM QUE RECONHECE A SOMA
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

// Parser do operador "+" com espaços opcionais
pub fn parser_add(input: &str) -> IResult<&str, BinaryOperator> {
    delimited(
        ws,       // Espaços antes
        tag("+"), // Operador
        ws,       // Espaços depois
    )
    .parse(input)
    .map(|(input, _)| (input, BinaryOperator::Add))
}

// Parser para Value ou identifier
pub fn parse_concret_value_or_identifier(input: &str) -> IResult<&str, Expression> {
    delimited(
        ws,
        alt((
            parser_concrect_value,
            map(identifier, Expression::Identifier),
        )),
        ws,
    )
    .parse(input)
}

// Parser da expressão binária (Add only)
// pub fn parse_binary_expression(input: &str) -> IResult<&str, Expression> {
//     let (input, left) = parse_int_or_identifier(input)?;
//     let (input, op) = parser_add(input)?;
//     let (input, right) = parse_int_or_identifier(input)?;

//     Ok((
//         input,
//         Expression::BinaryExp(op, Box::new(left), Box::new(right)),
//     ))
// }

//////////////// FIM DO TESTE 1 ///////////////////////

// Parser para Valores Concretos
// String
pub fn parser_string(input: &str) -> IResult<&str, ConcretValue> {
    delimited(
        tag("\""),
        map(take_while(|c: char| c != '"'), |s: &str| {
            ConcretValue::Value(Value::Str(s.to_string()))
        }),
        tag("\""),
    )
    .parse(input)
}

// Bool
pub fn parser_bool(input: &str) -> IResult<&str, ConcretValue> {
    alt((
        value(ConcretValue::Value(Value::Bool(true)), tag("true")),
        value(ConcretValue::Value(Value::Bool(false)), tag("false")),
    ))
    .parse(input)
}

pub fn parser_concrect_value(input: &str) -> IResult<&str, Expression> {
    delimited(
        ws,
        alt((
            map(parse_int, Expression::ConcretValue),
            map(parser_bool, Expression::ConcretValue),
            map(parser_string, Expression::ConcretValue),
        )),
        ws,
    )
    .parse(input)
}

// Parsers para operadores
// Binary
pub fn parser_binary_op(input: &str) -> IResult<&str, BinaryOperator> {
    delimited(
        ws,
        alt((
            value(BinaryOperator::Add, tag("+")),
            value(BinaryOperator::Sub, tag("-")),
            value(BinaryOperator::And, tag("and")),
            value(BinaryOperator::Or, tag("or")),
            value(BinaryOperator::Concat, tag("++")),
            value(BinaryOperator::Equal, tag("==")),
        )),
        ws,
    )
    .parse(input)
}

// Operadores Unários
pub fn parser_unary_op(input: &str) -> IResult<&str, UnaryOperator> {
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

// Parser para expressões
pub fn parser_expression(input: &str) -> IResult<&str, Expression> {
    let (input, _) = ws(input)?;
    let (input, expr) = alt((
        parser_concrect_value,
        map(identifier, Expression::Identifier),
        parser_unary_expression,
        parser_binary_expression,
    ))
    .parse(input)?;

    Ok((input, expr))
}

// Parser para expressões unárias
pub fn parser_unary_expression(input: &str) -> IResult<&str, Expression> {
    let (input, op) = parser_unary_op(input)?;
    let (input, exp) = parse_concret_value_or_identifier(input)?;

    Ok((input, Expression::UnaryExp(op, Box::new(exp))))
}

// Parser para expressões binarias
pub fn parser_binary_expression(input: &str) -> IResult<&str, Expression> {
    let (input, left) = parse_concret_value_or_identifier(input)?;
    let (input, op) = parser_binary_op(input)?;
    let (input, right) = parse_concret_value_or_identifier(input)?;

    Ok((
        input,
        Expression::BinaryExp(op, Box::new(left), Box::new(right)),
    ))
}

// Parser para while loop
pub fn parser_while_loop(input: &str) -> IResult<&str, Command> {
    let (input, _) = delimited(ws, tag("while"), ws).parse(input)?;
    let (input, cond) = parser_expression(input)?;
    let (input, _) = delimited(ws, tag("do"), ws).parse(input)?;
    let (input, body) = parser_command(input)?;

    Ok((input, Command::WhileLoop(cond, Box::new(body))))
}

// Parser para if then else
pub fn parse_if_then_else(input: &str) -> IResult<&str, Command> {
    let (input, _) = delimited(ws, tag("if"), ws).parse(input)?;
    let (input, cond) = parser_expression(input)?;
    let (input, _) = delimited(ws, tag("then"), ws).parse(input)?;
    let (input, then_branch) = parser_command(input)?;
    let (input, _) = delimited(ws, tag("else"), ws).parse(input)?;
    let (input, else_branch) = parser_command(input)?;

    Ok((
        input,
        Command::IfElse(cond, Box::new(then_branch), Box::new(else_branch)),
    ))
}

// Parser para sequence
pub fn parse_sequence(input: &str) -> IResult<&str, Command> {
    let (input, first_cmd) = parser_command(input)?;
    let (input, _) = delimited(ws, tag(";"), ws).parse(input)?;
    let (input, second_cmd) = parser_command(input)?;

    Ok((
        input,
        Command::Sequence(Box::new(first_cmd), Box::new(second_cmd)),
    ))
}

// Parser para comandos
pub fn parser_assignment(input: &str) -> IResult<&str, Command> {
    let (input, id) = terminated(identifier, ws).parse(input)?;
    let (input, _) = tag(":=").parse(input)?;
    let (input, exp) = parser_expression(input)?;

    Ok((input, Command::Assignment(id, exp)))
}

pub fn parser_declaration(input: &str) -> IResult<&str, Declaration> {
    // let (input, _) = ws(input)?;
    let (input, _) = terminated(tag("var"), ws).parse(input)?;
    let (input, id) = terminated(identifier, ws).parse(input)?;
    let (input, _) = tag("=").parse(input)?;
    let (input, expr) = parser_expression(input)?;

    Ok((input, Declaration::Variable(id, expr)))
}

pub fn parser_declaration_block(input: &str) -> IResult<&str, Command> {
    let (input, _) = terminated(tag("{"), ws).parse(input)?;
    let (input, decls) = separated_list0(tag(","), parser_declaration).parse(input)?;
    let (input, _) = terminated(tag(";"), ws).parse(input)?;
    let (input, cmd) = parser_command(input)?;
    let (input, _) = terminated(tag("}"), ws).parse(input)?;
    Ok((input, Command::DeclarationBlock(decls, Box::new(cmd))))
}

pub fn parser_command(input: &str) -> IResult<&str, Command> {
    let (input, _) = ws(input)?;

    alt((
        map(parser_assignment, |c| c),
        map(parser_declaration_block, |c| c),
        // map(parser_while_loop, |(cond, cmd)| {
        //     Command::WhileLoop(cond, cmd)
        // }),
        // map(parse_if_then_else, |(cond, t, f)| {
        //     Command::IfThenElse(cond, Box::new(t), Box::new(f))
        // }),
        // map(parse_io, Command::IO),
        // map(parse_sequence, |(c1, c2)| {
        //     Command::Sequence(Box::new(c1), Box::new(c2))
        // }),
        value(Command::Skip, tag("skip")),
    ))
    .parse(input)
}

pub fn parser_program(input: &str) -> IResult<&str, Program> {
    map(parser_command, Program::Command).parse(input)
}
