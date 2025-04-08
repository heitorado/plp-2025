use estudos_rust::parsers::program_parser::parse_program;

fn main() {
    // let code = r#"if x == 5 then y := x + 1 else z := 2 - x"#;
    // let code =
    //     r"{ var x = 1; x := 5; x := x + 1; if x == 6 then write(x) else write(x + 2); read(x) }";
    let code = r#"{ var x = 5; var y = 10; y := y + length"test"; write(x + y) }"#;
    // let code = r#"{ var x = length"test"; write(x) }"#;

    match parse_program(code) {
        Ok((_, program)) => println!("{:#?}", program),
        Err(e) => println!("Erro: {:?}", e),
    }
}
