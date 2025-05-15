fn main() {
    // use plp_2025::executor::executor::Executor;
    use plp_2025::parsers::program_parser;
    use plp_2025::semantic::semantic::SemanticAnalyzer;

    // let code: &str = r#"{
    //             var x = 0,
    //             var z = move x,
    //             var y = move z;
    //             write(y)
    //         }
    // "#;

    // let _sample_code_1 = r#"
    // {
    //   var b = 3;
    //   proc escreveRecursivo(int a) unit {
    //     if (not (a == 0)) then {
    //       var x = 0; x := a - 1;
    //       write("Ola");
    //       call escreveRecursivo(x)
    //     } else {
    //      skip
    //     }
    //    };

    //    call escreveRecursivo(b)
    //  }"#;

    let _sample_code_1 = r#"
<<<<<<< HEAD
            {
        var c = 5;
        proc soma(int a, int b) int {
            a + b
        };
        c := soma(10, 20);
        write(c);
        call soma(20, 40)
    }
        "#;

    // let sample_code_2 = r#"
    // {
    //   var x = 10; write(5)
    // }"#;
=======
    {
      var b = 3, var c = 1,
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

    // Test UNARY and BINARY expressions
    // let sample_code_2 = r#"
    // {
    //   var x = 10; write(not (not (not (not false)))); write(-(-(-2))); write(3); write(2 + 3); write(1 + 2 + 3 + 4 + 5); write(1 + 2 - 3); write(2 - 5); write(-(2 - 10)); write(2 == 2); write(not(2 == 2)); write(2 < 3); write(3 > 2); write(3 <= 3); write(3 >= 3); write("hello world"); write(2 == (1 + 3 - 2)); write("a" + "b" + "cde"); write(1 + (2 - (1 + (3 - (2 + (4 - (3 + (5 - 4))))))))
    // }"#;

    // Test IF-ELSE
    // let sample_code_2 = r#"
    // {
    //   var x = 10;
    //   if ((5+5) == (8+8-6)) then {
    //     var z = 1; write("ten is equal to ten")
    //   } else {
    //     var w = 1; write("ten is not equal to ten")
    //   }
    // }"#;

    // let sample_code_2 = r#"
    // {
    //   var x = 10, var y = 5, var z = x+y, var seraqueda = true; write(z); write(1+1); write(z+3);
    //   {
    //     var k = 10; write(k+k); write(2+6); write(-z); write(-(x-y+z))
    //   };
    //
    //   write(x+y+z);
    //   write(not (not seraqueda));
    //   write(k+z)
    // }
    // "#;

    let sample_code_2 = r#"
    {
      var x = 10, var y = 99, var z = 1;
      write(x); 
      write(y);
      x := y;
      z := y;
      write(x);
      {
        x := 10000;
        b := 9;
        write(x);
        write(b)
      };

      write(x)
    }
    "#;
>>>>>>> heitor/add-executor

    // let sample_code_3 = r#"
    //     { var x = 1;  write(length("xxx")) }
    // "#;
    // let code = r#"{ var x = 10,
    //     proc usar_int(int a) { write(a) };
    //     call usar_int(x);
    //     write(x)
    // }"#;

    // let code = "{ var x = length(30); write(x) }";

    // let code = r#""#;

    let code = _sample_code_1;
    let program = program_parser::parse_program(code);
    // match program_parser::parse_program(code) {
    //     Ok((_, program)) => println!("{:#?}", program),
    //     Err(e) => println!("Erro: {:?}", e),
    // }

<<<<<<< HEAD
    // println!("{}", program)

    // // Análise Semântica
=======
    println!("Program: {:?}", program);

    // Análise Semântica
>>>>>>> heitor/add-executor
    match program {
        Ok((_, ref program)) => {
            let mut analyzer = SemanticAnalyzer::new();
            let result = analyzer.check_program(&program);
            println!("Semantic Analysis: {:?}", result)
        }
        Err(ref e) => panic!("Erro: {:?}", e)
    }

<<<<<<< HEAD
    // println!("Running...");
    // // TODO: solve borrow problem above to avoid shadowing the program variable here
    // let program = program_parser::parse_program(code);

    // // Execução
    // match program {
    //     Ok((_, program)) => {
    //         let executor = Executor::new();
    //         let result = executor.execute_program(&program);
    //         println!("Result: {:?}", result)
    //     }
    //     Err(e) => println!("Erro: {:?}", e),
    // }
=======
    println!("Running...");

    // Execução
    match program {
        Ok((_, program)) => {
            let mut executor = Executor::new();
            // let result = executor.execute_program(&program);
            // println!("Result: {:?}", result)
            executor.execute_program(&program);
        }
        Err(e) => println!("Erro: {:?}", e),
    }
>>>>>>> heitor/add-executor
}
