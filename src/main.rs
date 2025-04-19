fn main() {
    use plp_2025::semantic::semantic::SemanticAnalyzer;
    use plp_2025::executor::executor::Executor;
    use plp_2025::parsers::program_parser;

    // let code: &str = r#"{
    //             var x = 0,
    //             var z = move x,
    //             var y = move z;
    //             write(y)
    //         }
    // "#;

    let _sample_code_1 = r#"
    {
      var b = 3,
      proc escreveRecursivo (int a) {
        if (not (a == 0)) then {
          var x = 0; x := a - 1;
          write("Ola");
          call escreveRecursivo(x)
        } else skip
      };

      write(b)
    }"#;

    let sample_code_2 = r#"
    {
      var x = 10; write(5)
    }"#;

    // let code = r#"{ var x = 10,
    //     proc usar_int(int a) { write(a) };
    //     call usar_int(x);
    //     write(x)
    // }"#;

    // let code = "{ var x = length(30); write(x) }";

    // let code = r#""#;

    let code = sample_code_2;
    let program = program_parser::parse_program(code);

    // Análise Semântica
    match program {
        Ok((_, program)) => {
            let mut analyzer = SemanticAnalyzer::new();
            let result = analyzer.check_program(&program);
            println!("Semantic Analysis: {:?}", result)
        }
        Err(e) => println!("Erro: {:?}", e),
    }

    println!("Running...");
    // TODO: solve borrow problem above to avoid shadowing the program variable here
    let program = program_parser::parse_program(code);

    // Execução
    match program {
        Ok((_, program)) => {
            let executor = Executor::new();
            let result = executor.execute_program(&program);
            println!("Result: {:?}", result)
        }
        Err(e) => println!("Erro: {:?}", e),
    }
}
