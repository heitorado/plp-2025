#[cfg(test)]
mod expression_parsers_tests {
    use estudos_rust::ast::{BinaryOperator, ConcreteValue, UnaryOperator};
    use estudos_rust::ast::{Expression, Value};
    use estudos_rust::parsers::expression_parsers::{
        parse_concrete_value, parse_expression, parse_expression_atomic, parse_unary_expression,
    };

    #[test]
    fn test_parse_concrete_int() {
        let input = "42";
        let result = parse_concrete_value(input);
        assert_eq!(
            result,
            Ok((
                "",
                Expression::ConcreteValue(ConcreteValue::Value(Value::Int(42)))
            ))
        );
    }

    #[test]
    fn test_parse_concrete_bool() {
        let input = "true";
        let result = parse_concrete_value(input);
        assert_eq!(
            result,
            Ok((
                "",
                Expression::ConcreteValue(ConcreteValue::Value(Value::Bool(true)))
            ))
        );
    }

    #[test]
    fn test_parse_concrete_string() {
        let input = "\"hello\"";
        let result = parse_concrete_value(input);
        assert_eq!(
            result,
            Ok((
                "",
                Expression::ConcreteValue(ConcreteValue::Value(Value::Str("hello".to_string())))
            ))
        );
    }

    #[test]
    fn test_parse_atomic_identifier() {
        let input = "var_name";
        let result = parse_expression_atomic(input);
        assert_eq!(
            result,
            Ok(("", Expression::Identifier("var_name".to_string())))
        );
    }

    #[test]
    fn test_parse_atomic_mixed_whitespace() {
        let input = "\n\t123 \t";
        let result = parse_expression_atomic(input);
        assert_eq!(
            result,
            Ok((
                "",
                Expression::ConcreteValue(ConcreteValue::Value(Value::Int(123)))
            ))
        );
    }

    #[test]
    fn test_parse_unary_neg() {
        let input = "-15";
        let result = parse_unary_expression(input);
        assert_eq!(
            result,
            Ok((
                "",
                Expression::UnaryExp(
                    UnaryOperator::Neg,
                    Box::new(Expression::ConcreteValue(ConcreteValue::Value(Value::Int(
                        15
                    ))))
                )
            ))
        );
    }

    #[test]
    fn test_parse_unary_not() {
        let input = "not true";
        let result = parse_unary_expression(input);
        assert_eq!(
            result,
            Ok((
                "",
                Expression::UnaryExp(
                    UnaryOperator::Not,
                    Box::new(Expression::ConcreteValue(ConcreteValue::Value(Value::Bool(
                        true
                    ))))
                )
            ))
        );
    }

    #[test]
    fn test_parse_unary_length() {
        let input = "length \"test\"";
        let result = parse_unary_expression(input);
        assert_eq!(
            result,
            Ok((
                "",
                Expression::UnaryExp(
                    UnaryOperator::Length,
                    Box::new(Expression::ConcreteValue(ConcreteValue::Value(Value::Str(
                        "test".to_string()
                    ))))
                )
            ))
        );
    }

    #[test]
    fn test_parse_binary_add() {
        let input = "10 + 20";
        let result = parse_expression(input);
        assert_eq!(
            result,
            Ok((
                "",
                Expression::BinaryExp(
                    BinaryOperator::Add,
                    Box::new(Expression::ConcreteValue(ConcreteValue::Value(Value::Int(
                        10
                    )))),
                    Box::new(Expression::ConcreteValue(ConcreteValue::Value(Value::Int(
                        20
                    ))))
                )
            ))
        );
    }

    #[test]
    fn test_parse_binary_concat() {
        let input = "\"foo\" ++ \"bar\"";
        let result = parse_expression(input);
        assert_eq!(
            result,
            Ok((
                "",
                Expression::BinaryExp(
                    BinaryOperator::Concat,
                    Box::new(Expression::ConcreteValue(ConcreteValue::Value(Value::Str(
                        "foo".to_string()
                    )))),
                    Box::new(Expression::ConcreteValue(ConcreteValue::Value(Value::Str(
                        "bar".to_string()
                    ))))
                )
            ))
        );
    }

    #[test]
    fn test_parse_binary_complex_operands() {
        let input = "x and y";
        let result = parse_expression(input);
        assert_eq!(
            result,
            Ok((
                "",
                Expression::BinaryExp(
                    BinaryOperator::And,
                    Box::new(Expression::Identifier("x".to_string())),
                    Box::new(Expression::Identifier("y".to_string()))
                )
            ))
        );
    }

    #[test]
    fn test_parse_expression_unary() {
        let input = "not false";
        let result = parse_expression(input);
        assert_eq!(
            result,
            Ok((
                "",
                Expression::UnaryExp(
                    UnaryOperator::Not,
                    Box::new(Expression::ConcreteValue(ConcreteValue::Value(Value::Bool(
                        false
                    ))))
                )
            ))
        );
    }

    #[test]
    fn test_parse_expression_nested_binary() {
        let input = "5 + 10 == 15";
        let result = parse_expression(input);
        assert_eq!(
            result,
            Ok((
                "",
                Expression::BinaryExp(
                    BinaryOperator::Equal,
                    Box::new(Expression::BinaryExp(
                        BinaryOperator::Add,
                        Box::new(Expression::ConcreteValue(ConcreteValue::Value(Value::Int(5)))),
                        Box::new(Expression::ConcreteValue(ConcreteValue::Value(Value::Int(
                            10
                        ))))
                    )),
                    Box::new(Expression::ConcreteValue(ConcreteValue::Value(Value::Int(
                        15
                    ))))
                )
            ))
        );
    }

    #[test]
    fn test_parse_expression_invalid() {
        let input = "123 +";
        let result = parse_expression(input);
        assert!(result.is_err());
    }
}
