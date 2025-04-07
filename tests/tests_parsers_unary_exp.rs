use estudos_rust::parser_unary_op;

#[cfg(test)]
mod test_parsets_unary_expression {
    use estudos_rust::ast::UnaryOperator;

    use super::*;

    #[test]
    fn test_parser_unary_op_neg() {
        let input = "-";
        let result = parser_unary_op(input);

        assert_eq!(result, Ok(("", UnaryOperator::Neg)))
    }
    #[test]
    fn test_parser_unary_op_not() {
        let input = "not";
        let result = parser_unary_op(input);

        assert_eq!(result, Ok(("", UnaryOperator::Not)))
    }
    #[test]
    fn test_parser_unary_op_length() {
        let input = "length";
        let result = parser_unary_op(input);

        assert_eq!(result, Ok(("", UnaryOperator::Length)))
    }
}
