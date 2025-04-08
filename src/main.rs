use estudos_rust::parsers::program_parser::parse_program;

fn main() {
    let code = r#"{ var x = 0; while x == 0 do x := x + 1 }"#;
    // let code =
    // r"{ var x = 1; x := 5; x := x + 1; if x == 6 then write(x) else write(x + 2); read(x) }";
    // let code = r#"{ var x = 5; var y = 10; y := y + 10; write(x + y) }"#;
    // let code = r#"{ var x = length("test"); write(x) }"#;
    // let code = "{ var x = (10); write(10) }";

    match parse_program(code) {
        Ok((_, program)) => println!("{:#?}", program),
        Err(e) => println!("Erro: {:?}", e),
    }
}
