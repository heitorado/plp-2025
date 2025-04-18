fn main() {
    // let code = r#"{ var a  =  0 ,
    // proc incA (int a)  {
    //     a := a + 1
    // };
    // call incA(10);
    // call incA(20);
    // write(a)
    // }
    // "#;

    let code = r#"{ var x = 0,
            var z = move x,
        proc p (int y) {x := x + y};
        { var x = 1;
            call p(3); write(x)
        };
        call p(4); write(x)
        }
"#;

    // let code = r#""#;

    match parser::parsers::program_parser::parse_program(code) {
        Ok((_, program)) => println!("{:#?}", program),
        Err(e) => println!("Erro: {:?}", e),
    }
}
