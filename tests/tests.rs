use estudos_rust::ast::{ConcretValue, Value};
use estudos_rust::{identifier, parse_int, ws};

#[cfg(test)]
mod tests {
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
}
