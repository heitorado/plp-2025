#[cfg(test)]
mod tests {
    use parser::ast::*;
    use plp_2025::semantic::semantic::SemanticAnalyzer;
    #[test]
    fn test_valid_move() {
        let program = Program::Command(Command::DeclarationBlock(
            vec![Declaration::Variable(
                "a".to_string(),
                Expression::ConcreteValue(ConcreteValue::Value(Value::Int(5))),
                false,
            )],
            Box::new(Command::Assignment(
                "b".to_string(),
                Expression::Identifier("a".to_string()),
                true,
            )),
        ));

        let mut analyzer = SemanticAnalyzer::new();
        let result = analyzer.check_program(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_move_twice() {
        let program = Program::Command(Command::Sequence(
            Box::new(Command::Assignment(
                "b".to_string(),
                Expression::Identifier("a".to_string()),
                true,
            )),
            Box::new(Command::Assignment(
                "c".to_string(),
                Expression::Identifier("a".to_string()),
                false,
            )),
        ));

        let mut analyzer = SemanticAnalyzer::new();
        let result = analyzer.check_program(&program);
        assert!(result.is_err());
        assert!(analyzer.errors.iter().any(|e| e.contains("movida")));
    }

    #[test]
    fn test_type_mismatch() {
        let program = Program::Command(Command::Assignment(
            "x".to_string(),
            Expression::BinaryExp(
                BinaryOperator::Add,
                Box::new(Expression::ConcreteValue(ConcreteValue::Value(Value::Int(
                    5,
                )))),
                Box::new(Expression::ConcreteValue(ConcreteValue::Value(Value::Str(
                    "texto".to_string(),
                )))),
            ),
            false,
        ));

        let mut analyzer = SemanticAnalyzer::new();
        let result = analyzer.check_program(&program);
        assert!(result.is_err());
        assert!(
            analyzer
                .errors
                .iter()
                .any(|e| e.contains("Tipo incompatível"))
        );
    }

    #[test]
    fn test_procedure_parameters() {
        let program = Program::Command(Command::DeclarationBlock(
            vec![Declaration::Procedure(
                "inc".to_string(),
                vec![ProcedureParameter {
                    identifier: "x".to_string(),
                    r#type: Type::Int,
                }],
                Box::new(Command::Assignment(
                    "x".to_string(),
                    Expression::BinaryExp(
                        BinaryOperator::Add,
                        Box::new(Expression::Identifier("x".to_string())),
                        Box::new(Expression::ConcreteValue(ConcreteValue::Value(Value::Int(
                            1,
                        )))),
                    ),
                    false,
                )),
            )],
            Box::new(Command::CallProcedure(CallProcedure {
                id: "inc".to_string(),
                args: vec![Expression::ConcreteValue(ConcreteValue::Value(Value::Str(
                    "erro".to_string(),
                )))],
            })),
        ));

        let mut analyzer = SemanticAnalyzer::new();
        let result = analyzer.check_program(&program);
        assert!(result.is_err());
        assert!(
            analyzer
                .errors
                .iter()
                .any(|e| e.contains("Tipo do parâmetro"))
        );
    }
}
