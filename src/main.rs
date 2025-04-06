use estudos_rust::parse_binary_expression;
// main.rs
fn main() {
    let inputs = vec!["3 + 5", "  x  +  10  ", "42+y"];

    for input in inputs {
        println!("\nTestando: '{}'", input);
        match parse_binary_expression(input) {
            Ok((remaining, expr)) => {
                println!("Restante: '{}'", remaining);
                println!("AST: {:?}", expr);
            }
            Err(e) => println!("Erro: {:?}", e),
        }
    }
}
