pub fn calculate(expression: &str) -> Result<i32, String> {
    let tokens: Vec<&str> = expression.split_whitespace().collect();

    if tokens.len() != 3 {
        return Err(String::from("Error: la expresion debe tener 3 elementos"));
    }

    let lhs = match tokens[0].parse::<i32>() {
        Ok(lhs) => lhs,
        Err(_) => return Err(String::from("numero erroneo")),
    };

    let operator = tokens[1];

    let rhs = match tokens[2].parse::<i32>() {
        Ok(rhs) => rhs,
        Err(_) => return Err(String::from("numero erroneo")),
    };

    match operator {
        "+" => Ok(lhs + rhs),
        "-" => Ok(lhs - rhs),
        "*" => Ok(lhs * rhs),
        "/" => {
            if rhs == 0 {
                Err(String::from("Error: no se puede dividir por cero"))
            } else {
                Ok(lhs / rhs)
            }
        }
        _ => Err(String::from(
            "Error: Operador no soportado. Solo se soporta '+', '-', '*' y '/'.",
        )),
    }
}
