use estudos_rust::parsers::program_parser::parse_program;

fn main() {
    let code = r#"if x == 5 then y := x + 1 else z := 2 - x"#;

    match parse_program(code) {
        Ok((_, program)) => println!("{:#?}", program),
        Err(e) => println!("Erro: {:?}", e),
    }
}
