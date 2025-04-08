#[cfg(test)]
mod operators_parsers_tests {
    use estudos_rust::ast::{BinaryOperator, UnaryOperator};
    use estudos_rust::parsers::operators_parsers::{parse_binary_operator, parse_unary_operator};

    // Testes para parse_binary_operator
    #[test]
    fn test_parse_binary_add() {
        let input = "+";
        let result = parse_binary_operator(input);
        assert_eq!(result, Ok(("", BinaryOperator::Add)));
    }

    #[test]
    fn test_parse_binary_add_with_whitespace() {
        let input = "  +  ";
        let result = parse_binary_operator(input);
        assert_eq!(result, Ok(("", BinaryOperator::Add)));
    }

    #[test]
    fn test_parse_binary_sub() {
        let input = "-";
        let result = parse_binary_operator(input);
        assert_eq!(result, Ok(("", BinaryOperator::Sub)));
    }

    #[test]
    fn test_parse_binary_and() {
        let input = "and";
        let result = parse_binary_operator(input);
        assert_eq!(result, Ok(("", BinaryOperator::And)));
    }

    #[test]
    fn test_parse_binary_and_with_remaining() {
        let input = "andxyz";
        let result = parse_binary_operator(input);
        assert_eq!(result, Ok(("xyz", BinaryOperator::And)));
    }

    #[test]
    fn test_parse_binary_or() {
        let input = "or";
        let result = parse_binary_operator(input);
        assert_eq!(result, Ok(("", BinaryOperator::Or)));
    }

    #[test]
    fn test_parse_binary_concat() {
        let input = "++";
        let result = parse_binary_operator(input);
        assert_eq!(result, Ok(("", BinaryOperator::Concat)));
    }

    #[test]
    fn test_parse_binary_equal() {
        let input = "==";
        let result = parse_binary_operator(input);
        assert_eq!(result, Ok(("", BinaryOperator::Equal)));
    }

    #[test]
    fn test_parse_binary_invalid_operator() {
        let input = "&";
        let result = parse_binary_operator(input);
        assert!(result.is_err());
    }

    // Testes para parse_unary_operator
    #[test]
    fn test_parse_unary_neg() {
        let input = "-";
        let result = parse_unary_operator(input);
        assert_eq!(result, Ok(("", UnaryOperator::Neg)));
    }

    #[test]
    fn test_parse_unary_neg_with_whitespace() {
        let input = "\t-\n";
        let result = parse_unary_operator(input);
        assert_eq!(result, Ok(("", UnaryOperator::Neg)));
    }

    #[test]
    fn test_parse_unary_not() {
        let input = "not";
        let result = parse_unary_operator(input);
        assert_eq!(result, Ok(("", UnaryOperator::Not)));
    }

    #[test]
    fn test_parse_unary_not_with_remaining() {
        let input = "not123";
        let result = parse_unary_operator(input);
        assert_eq!(result, Ok(("123", UnaryOperator::Not)));
    }

    #[test]
    fn test_parse_unary_length() {
        let input = "length";
        let result = parse_unary_operator(input);
        assert_eq!(result, Ok(("", UnaryOperator::Length)));
    }

    #[test]
    fn test_parse_unary_length_with_whitespace() {
        let input = "  length  ";
        let result = parse_unary_operator(input);
        assert_eq!(result, Ok(("", UnaryOperator::Length)));
    }

    #[test]
    fn test_parse_unary_invalid_operator() {
        let input = "neg";
        let result = parse_unary_operator(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_unary_partial_match() {
        let input = "no";
        let result = parse_unary_operator(input);
        assert!(result.is_err());
    }
}
