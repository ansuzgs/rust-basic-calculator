pub fn calculate(expression: &str) -> Result<i32, String> {
    //let tokens: Vec<&str> = expression.split_whitespace().collect();
    let tokens = tokenize(expression)?;

    if tokens.len() != 3 {
        return Err(String::from("Error: la expresion debe tener 3 elementos"));
    }

    let lhs = match tokens[0].parse::<i32>() {
        Ok(lhs) => lhs,
        Err(_) => return Err(String::from("numero erroneo")),
    };

    let operator = tokens[1].as_str();

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

fn tokenize(expression: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current_num = String::new();

    for c in expression.chars() {
        if c.is_ascii_digit() {
            current_num.push(c);
        } else if c == '.' {
            current_num.push(c);
        } else if is_operator(c) {
            if !current_num.is_empty() {
                tokens.push(current_num.clone());
                current_num.clear();
            }
            tokens.push(c.to_string());
        } else if c.is_whitespace() {
            if !current_num.is_empty() {
                tokens.push(current_num.clone());
                current_num.clear();
            }
            continue;
        } else {
            return Err(format!("Error: Caracter invalido '{}'", c));
        }
    }

    if !current_num.is_empty() {
        tokens.push(current_num);
    }

    Ok(tokens)
}

fn is_operator(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/')
}
