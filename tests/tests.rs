use estudos_rust::ast::{ConcretValue, Expression, Value};
use estudos_rust::{
    identifier, parse_binary_expression, parse_int, parse_int_or_identifier, parser_add, ws,
};

#[cfg(test)]
mod tests {
    use estudos_rust::ast::BinaryOperator;

    use super::*;

    // #[test]
    // fn test_ws() {
    //     assert_eq!(ws(" "), Ok(("", " ")));
    //     assert_eq!(ws("  \t \t"), Ok(("", "  \t \t")));
    //     assert_eq!(ws("  \t \t \n"), Ok(("", "  \t \t \n")));
    // }

    #[test]
    fn test_ws() {
        let input: &str = "  let x = 5;";
        assert_eq!(ws(input), Ok(("let x = 5;", "  ")));
        assert_eq!(ws("  \t \t"), Ok(("", "  \t \t")));
    }

    #[test]
    fn test_identifier() {
        assert_eq!(identifier("x"), Ok(("", "x".to_string())));
        assert_eq!(identifier("var1"), Ok(("", "var1".to_string())));
        assert_eq!(identifier("_temp"), Ok(("", "_temp".to_string())));
        assert!(identifier("123").is_err()); // Não pode começar com número
    }

    #[test]
    fn test_parse_int_valid() {
        let input = "12345";
        let result = parse_int(input);

        assert_eq!(result, Ok(("", ConcretValue::Value(Value::Int(12345)))));
    }

    #[test]
    fn test_parse_int_with_remaining() {
        let input = "123abc";
        let result = parse_int(input);

        assert_eq!(result, Ok(("abc", ConcretValue::Value(Value::Int(123)))));
    }

    #[test]
    fn test_parse_int_invalid() {
        let input = "abc";
        let result = parse_int(input);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_plus_signal() {
        let input = "  +  ";
        let result = parser_add(input);

        assert_eq!(result, Ok(("", BinaryOperator::Add)))
    }

    // #[test]
    // fn test_parse_int() {
    //     // Inteiro sem espaços
    //     assert_eq!(
    //         parse_int_or_identifier("42"),
    //         Ok(("", Expression::ConcretValue(Value::Int(42)))
    //     );

    //     // Inteiro com espaços
    //     assert_eq!(
    //         parse_int_or_identifier("   123   "),
    //         Ok(("", Expression::Value(ConcretValue::Int(123)))
    //     );
    // }

    #[test]
    fn test_parse_identifier() {
        // Identificador simples
        assert_eq!(
            parse_int_or_identifier("x"),
            Ok(("", Expression::Identifier("x".to_string())))
        );

        // Identificador com underscores e números
        assert_eq!(
            parse_int_or_identifier("_var1"),
            Ok(("", Expression::Identifier("_var1".to_string())))
        );

        // Identificador com espaços
        assert_eq!(
            parse_int_or_identifier("   y   "),
            Ok(("", Expression::Identifier("y".to_string())))
        );
    }

    #[test]
    fn test_invalid_cases() {
        // Número inválido (começa com letra)
        assert!(parse_int_or_identifier("abc123").is_ok()); // Será Identifier, não erro

        // Caractere inválido (@)
        assert!(parse_int_or_identifier("@").is_err());

        // String vazia
        assert!(parse_int_or_identifier("").is_err());
    }

    #[test]
    fn test_valid_add_expressions() {
        // Caso básico: 3 + 5
        assert_eq!(
            parse_binary_expression("3+5"),
            Ok((
                "",
                Expression::BinaryExp(
                    BinaryOperator::Add,
                    Box::new(Expression::ConcretValue(ConcretValue::Value(Value::Int(3)))),
                    Box::new(Expression::ConcretValue(ConcretValue::Value(Value::Int(5))))
                )
            ))
        );

        // Com espaços: x + 10
        assert_eq!(
            parse_binary_expression("x + 10"),
            Ok((
                "",
                Expression::BinaryExp(
                    BinaryOperator::Add,
                    Box::new(Expression::Identifier("x".to_string())),
                    Box::new(Expression::ConcretValue(ConcretValue::Value(Value::Int(
                        10
                    ))))
                )
            ))
        );

        // Espaços extremos: " 42 + y "
        assert_eq!(
            parse_binary_expression(" 42 + y "),
            Ok((
                "",
                Expression::BinaryExp(
                    BinaryOperator::Add,
                    Box::new(Expression::ConcretValue(ConcretValue::Value(Value::Int(
                        42
                    )))),
                    Box::new(Expression::Identifier("y".to_string()))
                )
            ))
        );
    }

    #[test]
    fn test_invalid_add_expressions() {
        // Operador faltando: "3 5"
        assert!(parse_binary_expression("3 5").is_err());

        // Operador inválido: "3 * 5"
        assert!(parse_binary_expression("3 * 5").is_err());

        // Termo direito faltando: "3 + "
        assert!(parse_binary_expression("3 + ").is_err());
    }
}
