#[cfg(test)]
mod basic_parsers_tests {
    use estudos_rust::parsers::basic_parsers::{parse_identifier, ws};

    #[test]
    fn test_ws() {
        let input: &str = "  let x = 5;";
        assert_eq!(ws(input), Ok(("let x = 5;", "  ")));
        assert_eq!(ws("  \t \t"), Ok(("", "  \t \t")));
    }

    #[test]
    fn test_identifier() {
        assert_eq!(parse_identifier("x"), Ok(("", "x".to_string())));
        assert_eq!(parse_identifier("var1"), Ok(("", "var1".to_string())));
        assert_eq!(parse_identifier("_temp"), Ok(("", "_temp".to_string())));

        // Error
        assert!(parse_identifier("123").is_err());
    }
}
