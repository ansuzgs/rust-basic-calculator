#[derive(Debug, PartialEq, Clone)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn precedence(&self) -> u8 {
        match self {
            Operator::Add | Operator::Subtract => 1,
            Operator::Multiply | Operator::Divide => 2,
        }
    }

    fn from_str(op: &str) -> Option<Operator> {
        match op {
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Subtract),
            "*" => Some(Operator::Multiply),
            "/" => Some(Operator::Divide),
            _ => None,
        }
    }
}

pub fn calculate(expression: &str) -> Result<i32, String> {
    let tokens = tokenize(expression)?;
    let rpn = shuting_yard(&tokens)?;
    evaluate_rpn(&rpn)
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

fn shuting_yard(tokens: &[String]) -> Result<Vec<String>, String> {
    let mut output_queue: Vec<String> = Vec::new();
    let mut operator_stack: Vec<String> = Vec::new();

    for token in tokens {
        if let Ok(_num) = token.parse::<i32>() {
            output_queue.push(token.clone());
        } else if let Some(op1) = Operator::from_str(token) {
            while let Some(top) = operator_stack.last() {
                if let Some(op2) = Operator::from_str(top) {
                    if (op2.precedence() > op1.precedence())
                        || (op2.precedence() == op1.precedence())
                    {
                        output_queue.push(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            operator_stack.push(token.clone());
        } else {
            return Err(format!("Error: operador no soportado '{}'", token));
        }
    }

    while let Some(op) = operator_stack.pop() {
        if op == "(" || op == ")" {
            return Err(String::from("Error: Paréntesis no balanceados."));
        }
        output_queue.push(op);
    }

    Ok(output_queue)
}

fn evaluate_rpn(rpn: &[String]) -> Result<i32, String> {
    let mut stack: Vec<i32> = Vec::new();

    for token in rpn {
        if let Ok(num) = token.parse::<i32>() {
            stack.push(num);
        } else if let Some(op) = Operator::from_str(token) {
            if stack.len() < 2 {
                return Err(String::from("Error: Expresion invalida"));
            }
            let rhs = stack.pop().unwrap();
            let lhs = stack.pop().unwrap();
            let result = match op {
                Operator::Add => lhs + rhs,
                Operator::Subtract => lhs - rhs,
                Operator::Multiply => lhs * rhs,
                Operator::Divide => {
                    if rhs == 0 {
                        return Err(String::from("Error: division por cero"));
                    }
                    lhs / rhs
                }
            };
            stack.push(result);
        } else {
            return Err(format!("Error: Operador no soportado '{}'", token));
        }
    }

    if stack.len() == 1 {
        Ok(stack[0])
    } else {
        Err(String::from("Error: Expresion invalida"))
    }
}
