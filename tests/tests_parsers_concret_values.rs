use estudos_rust::ast::{ConcretValue, Value};
use estudos_rust::{parse_int, parser_bool, parser_string, ws};

#[cfg(test)]
mod test_parsets_concrect_value {
    use super::*;

    #[test]
    fn test_parser_bool_true() {
        let input: &str = "true";

        assert_eq!(
            parser_bool(input),
            Ok(("", ConcretValue::Value(Value::Bool(true))))
        )
    }

    #[test]
    fn test_parser_bool_false() {
        let input: &str = "false";

        assert_eq!(
            parser_bool(input),
            Ok(("", ConcretValue::Value(Value::Bool(false))))
        )
    }

    #[test]
    fn test_parser_string() {
        let input: &str = "\"teste de string\" 1233";

        assert_eq!(
            parser_string(input),
            Ok((
                " 1233",
                ConcretValue::Value(Value::Str("teste de string".to_string()))
            ))
        );
        assert_eq!(ws("  \t \t"), Ok(("", "  \t \t")));
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
