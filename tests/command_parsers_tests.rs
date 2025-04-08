#[cfg(test)]
mod command_parsers_tests {
    use estudos_rust::ast::{BinaryOperator, Command, ConcretValue, Expression, IOCommand, Value};
    use estudos_rust::parsers::command_parsers::parse_command;

    #[test]
    fn test_assignment() {
        let input = "x := 42";
        assert_eq!(
            parse_command(input),
            Ok((
                "",
                Command::Assignment(
                    "x".into(),
                    Expression::ConcretValue(ConcretValue::Value(Value::Int(42)))
                )
            ))
        );
    }
    #[test]
    fn test_assignment_complex() {
        let input = "x := x + 42";
        assert_eq!(
            parse_command(input),
            Ok((
                "",
                Command::Assignment(
                    "x".into(),
                    Expression::BinaryExp(
                        BinaryOperator::Add,
                        Box::new(Expression::Identifier("x".to_string())),
                        Box::new(Expression::ConcretValue(ConcretValue::Value(Value::Int(
                            42
                        ))))
                    )
                )
            ))
        );
    }

    #[test]
    fn test_declaration_block() {
        let input = "{ var x = 5; var y = 10 }";
        assert!(parse_command(input).is_ok());
    }

    #[test]
    fn test_while_loop() {
        let input = "while x == 0 do x := x + 1";
        let expected = Command::WhileLoop(
            Expression::BinaryExp(
                BinaryOperator::Equal,
                Box::new(Expression::Identifier("x".to_string())),
                Box::new(Expression::ConcretValue(ConcretValue::Value(Value::Int(0)))),
            ),
            Box::new(Command::Assignment(
                "x".to_string(),
                Expression::BinaryExp(
                    BinaryOperator::Add,
                    Box::new(Expression::Identifier("x".to_string())),
                    Box::new(Expression::ConcretValue(ConcretValue::Value(Value::Int(1)))),
                ),
            )),
        );
        assert_eq!(parse_command(input), Ok(("", expected)));
    }

    #[test]
    fn test_if_else() {
        let input = "if x then y := 1 else z := 2";
        let expected = Command::IfElse(
            Expression::Identifier("x".to_string()),
            Box::new(Command::Assignment(
                "y".into(),
                Expression::ConcretValue(ConcretValue::Value(Value::Int(1))),
            )),
            Box::new(Command::Assignment(
                "z".into(),
                Expression::ConcretValue(ConcretValue::Value(Value::Int(2))),
            )),
        );
        assert_eq!(parse_command(input), Ok(("", expected)));
    }

    #[test]
    fn test_io_write() {
        let input = "write(42)";
        assert_eq!(
            parse_command(input),
            Ok((
                "",
                Command::IO(IOCommand::Write(Box::new(Expression::ConcretValue(
                    ConcretValue::Value(Value::Int(42))
                ))))
            ))
        );
    }

    #[test]
    fn test_sequence() {
        let input = "x := 5; y := 10";
        let expected = Command::Sequence(
            Box::new(Command::Assignment(
                "x".into(),
                Expression::ConcretValue(ConcretValue::Value(Value::Int(5))),
            )),
            Box::new(Command::Assignment(
                "y".into(),
                Expression::ConcretValue(ConcretValue::Value(Value::Int(10))),
            )),
        );
        assert_eq!(parse_command(input), Ok(("", expected)));
    }
}
