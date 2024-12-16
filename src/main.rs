mod calculator;

use calculator::calculate;

fn main() {
    println!("Bienvenidos a la calculadora en Rust");

    let mut input = String::new();
    println!("Introduce la operacion: ");
    std::io::stdin().read_line(&mut input).unwrap();
    println!("{}", input);

    match calculate(&input) {
        Ok(result) => println!("= {}", result),
        Err(e) => println!("{}", e),
    }
}
