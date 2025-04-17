use estudos_rust::parsers::program_parser::parse_program;

fn main() {
    // let code = r#"{ var x = 0; while x <= 20 do if (x <= 10) then x := x + 1 else x := x + 2 }"#;
    // let code = r"{ var x = 1; x := 5; x := x + 1; if x == 6 then write(x) else write(x + 2); read(x) }";
    // let code = r#"{ var x = 5; var y = 10; y := y + 10; write(x + y) }"#;
    // let code = r#"{ var x = length("test"); write(x) }"#;
    // let code = "{ var x = (10); write(10) }";
    //Define a multiline string
    // let code = r#"
    // {
    //   var b = 3,
    //   proc escreveRecursivo (int a) {
    //     if (not (a == 0)) then {
    //       var x = 0; x := a - 1;
    //       write("Ola");
    //       call escreveRecursivo(x)
    //     } else skip
    //   };

    //   call escreveRecursivo(b)
    // }"#;
    let code = r#"{ proc test(int a) { write(a) }; write(true) }"#;

    match parse_program(code) {
        Ok((_, program)) => println!("{:#?}", program),
        Err(e) => println!("Erro: {:?}", e),
    }
}
