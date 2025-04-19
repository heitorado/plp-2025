fn main() {
    use plp_2025::semantic::semantic::SemanticAnalyzer;
    use plp_2025::parsers::program_parser;

    // let code: &str = r#"{
    //             var x = 0,
    //             var z = move x,
    //             var y = move z;
    //             write(y)
    //         }
    // "#;

    let code = r#"
    {
      var b = 3,
      proc escreveRecursivo (int a) {
        if (not (a == 0)) then {
          var x = 0; x := a - 1;
          write("Ola");
          call escreveRecursivo(x)
        } else skip
      };

      call escreveRecursivo(b);
      write(b)
    }"#;

    // let code = r#"{ var x = 10,
    //     proc usar_int(int a) { write(a) };
    //     call usar_int(x);
    //     write(x)
    // }"#;

    // let code = "{ var x = length(30); write(x) }";

    // let code = r#""#;

    match program_parser::parse_program(code) {
        Ok((_, program)) => {
            let mut analyzer = SemanticAnalyzer::new();
            let result = analyzer.check_program(&program);
            println!("{:#?}", result)
        }
        Err(e) => println!("Erro: {:?}", e),
    }
}
