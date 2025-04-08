#[cfg(test)]
mod concret_value_parsers_tests {
    use estudos_rust::ast::{ConcretValue, Value};
    use estudos_rust::parsers::concret_value_parsers::{parse_bool, parse_int, parse_string};

    #[test]
    fn test_parse_bool_true() {
        let input: &str = "true";

        assert_eq!(
            parse_bool(input),
            Ok(("", ConcretValue::Value(Value::Bool(true))))
        )
    }

    #[test]
    fn test_parse_bool_false() {
        let input: &str = "false";

        assert_eq!(
            parse_bool(input),
            Ok(("", ConcretValue::Value(Value::Bool(false))))
        )
    }

    #[test]
    fn test_parse_string() {
        let input: &str = "\"teste de string\" 1233";

        assert_eq!(
            parse_string(input),
            Ok((
                " 1233",
                ConcretValue::Value(Value::Str("teste de string".to_string()))
            ))
        );
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
}
