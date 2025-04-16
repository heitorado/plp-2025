#[cfg(test)]
mod declaration_parsers_tests {
    use estudos_rust::ast::{ConcreteValue, Declaration, Expression, Value, Type, ProcedureParameter, IOCommand};
    use estudos_rust::parsers::declaration_parsers::{ parse_declaration, parse_procedure_parameter, parse_procedure_parameters, parse_procedure_declaration };
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

    #[test]
    fn test_procedure_parameter_declaration_int() {
        let input = "int brun0";
        let result = parse_procedure_parameter(input);
        assert_eq!(result, Ok(("", ProcedureParameter { identifier: Expression::Identifier("brun0".to_string()), type_name: Type::Int })));
    }

    #[test]
    fn test_procedure_parameter_declaration_string() {
        let input = "string g10v4nn4";
        let result = parse_procedure_parameter(input);
        assert_eq!(result, Ok(("", ProcedureParameter { identifier: Expression::Identifier("g10v4nn4".to_string()), type_name: Type::Str })));
    }

    #[test]
    fn test_procedure_parameter_declaration_bool() {
        let input = "bool h31t0r";
        let result = parse_procedure_parameter(input);
        assert_eq!(result, Ok(("", ProcedureParameter { identifier: Expression::Identifier("h31t0r".to_string()), type_name: Type::Bool })));
    }

    #[test]
    fn test_procedure_parameter_declaration_with_multiple_parameters() {
        let input = "int a, string b, bool c";
        let result = parse_procedure_parameters(input);
        assert_eq!(result, Ok(("", vec![
            ProcedureParameter { identifier: Expression::Identifier("a".to_string()), type_name: Type::Int },
            ProcedureParameter { identifier: Expression::Identifier("b".to_string()), type_name: Type::Str },
            ProcedureParameter { identifier: Expression::Identifier("c".to_string()), type_name: Type::Bool },
        ])));
    }

    #[test]
    fn test_procedure_parameter_declaration_with_a_single_parameter() {
        let input = "int askfbjkasf329842308g3u210g";
        let result = parse_procedure_parameters(input);
        assert_eq!(result, Ok(("", vec![
            ProcedureParameter { identifier: Expression::Identifier("askfbjkasf329842308g3u210g".to_string()), type_name: Type::Int },
        ])));
    }

    #[test]
    fn test_procedure_declaration() {
        let input = r#"proc test(int a) { write(a) }"#;
        let result = parse_procedure_declaration(input);
        assert_eq!(result, Ok(("", Declaration::Procedure(
            "test".to_string(),
            vec![ProcedureParameter { identifier: Expression::Identifier("a".to_string()), type_name: Type::Int }],
            Box::new(estudos_rust::ast::Command::IO(IOCommand::Write(Box::new(Expression::Identifier("a".to_string())))))
        ))));
    }
}
