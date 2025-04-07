use estudos_rust::parser_program;
// main.rs
// fn main() {
//     let inputs = vec!["3 -  5", " x == 10", "42 +y"];

//     for input in inputs {
//         println!("\nTestando: '{}'", input);
//         match parser_binary_expression(input) {
//             Ok((remaining, expr)) => {
//                 println!("Restante: '{}'", remaining);
//                 println!("AST: {:?}", expr);
//             }
//             Err(e) => println!("Erro: {:?}", e),
//         }
//     }
// }

fn main() {
    let code = "{var x = 10;y := x + 5;}";

    match parser_program(code) {
        Ok((remaining, program)) => {
            println!("Parsed: {:?}", program);
            assert!(remaining.trim().is_empty());
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
