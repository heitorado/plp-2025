use estudos_rust::ast::{Command, ConcretValue, Expression, Value};
use estudos_rust::parser_assignment;

#[cfg(test)]
mod test_parsers_command {

    use estudos_rust::{ast::Declaration, parser_declaration};

    use super::*;

    #[test]
    fn test_parser_assignment() {
        let input = "x := 5";
        let result = parser_assignment(input);

        assert_eq!(
            result,
            Ok((
                "",
                Command::Assignment(
                    "x".to_string(),
                    Expression::ConcretValue(ConcretValue::Value(Value::Int(5)))
                )
            ))
        )
    }

    #[test]
    fn test_parser_declaration() {
        let input = "var x = 4";
        let result = parser_declaration(input);

        assert_eq!(
            result,
            Ok((
                "",
                Declaration::Variable(
                    "x".to_string(),
                    Expression::ConcretValue(ConcretValue::Value(Value::Int(4)))
                )
            ))
        )
    }
}
