fn main() {
    use plp_2025::executor::executor::Executor;
    use plp_2025::parsers::program_parser;
    use plp_2025::semantic::semantic::SemanticAnalyzer;

    let _sample_code_1 = r#"
    {
        var c = 5;
        var d = 10;
        proc soma(int a, int b) int {
            a + b
        };
        write(c);
        c := call soma(c, d);
        write(c);
        call soma(20, 40)
    }
    "#;

    let code = _sample_code_1;

    // === PARSING ===
    let parsed = program_parser::parse_program(code);
    let program = match parsed {
        Ok((_, ref program)) => program,
        Err(e) => {
            eprintln!("Erro de sintaxe: {:?}", e);
            return;
        }
    };

    // === ANÁLISE SEMÂNTICA ===
    let mut analyzer = SemanticAnalyzer::new();
    match analyzer.check_program(program) {
        Ok(()) => {
            // === EXECUÇÃO ===
            let mut executor = Executor::new();
            let result = executor.execute_program(program);

            if !executor.errors.is_empty() {
                eprintln!("Erros de execução encontrados:");
                for error in executor.errors {
                    eprintln!("- {}", error);
                }
            } else {
                println!("Resultado: {:?}", result);
            }
        }
        Err(errors) => {
            eprintln!("Erros semânticos encontrados:");
            for error in errors {
                eprintln!("- {}", error);
            }
        }
    }
}
