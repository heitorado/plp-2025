#[cfg(test)]
mod declaration_parsers_tests {
    use estudos_rust::ast::{ConcreteValue, Declaration, Expression, Value};
    use estudos_rust::parsers::declaration_parsers::parse_declaration;

    #[test]
    fn test_single_declaration() {
        let input = "var x = 42";
        let result = parse_declaration(input);
        assert_eq!(
            result,
            Ok((
                "",
                Declaration::Variable(
                    "x".to_string(),
                    Expression::ConcreteValue(ConcreteValue::Value(Value::Int(42)))
                )
            ))
        );
    }

    #[test]
    fn test_multiple_declarations() {
        let input = "var a = 5; var b = 10";
        let result = parse_declaration(input);
        assert_eq!(
            result,
            Ok((
                "",
                Declaration::Compound(
                    Box::new(Declaration::Variable(
                        "a".to_string(),
                        Expression::ConcreteValue(ConcreteValue::Value(Value::Int(5)))
                    )),
                    Box::new(Declaration::Variable(
                        "b".to_string(),
                        Expression::ConcreteValue(ConcreteValue::Value(Value::Int(10)))
                    ))
                )
            ))
        );
    }

    #[test]
    fn test_whitespace_variations() {
        let input = "var\nx\t=\t7;var y= \"text\"";
        let result = parse_declaration(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_declaration() {
        let input = "var = 5";
        let result = parse_declaration(input);
        assert!(result.is_err());
    }
}
