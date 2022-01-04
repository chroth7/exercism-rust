#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn operate(
    x: CalculatorInput,
    y: CalculatorInput,
    operation: fn(&i32, &i32) -> i32,
) -> Option<CalculatorInput> {
    match x {
        CalculatorInput::Value(xx) => match y {
            CalculatorInput::Value(yy) => Some(CalculatorInput::Value(operation(&xx, &yy))),
            _ => None,
        },
        _ => None,
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut result = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Value(n) => result.push(CalculatorInput::Value(*n)),
            CalculatorInput::Add => {
                if result.len() < 2 {
                    return None;
                }
                let foo = operate(result.pop().unwrap(), result.pop().unwrap(), |x, y| x + y);
                match foo {
                    None => return None,
                    Some(n) => result.push(n),
                }
            }
            CalculatorInput::Subtract => {
                if result.len() < 2 {
                    return None;
                }
                let foo = operate(result.pop().unwrap(), result.pop().unwrap(), |x, y| y - x);
                match foo {
                    None => return None,
                    Some(n) => result.push(n),
                }
            }
            CalculatorInput::Multiply => {
                if result.len() < 2 {
                    return None;
                }
                let foo = operate(result.pop().unwrap(), result.pop().unwrap(), |x, y| x * y);
                match foo {
                    None => return None,
                    Some(n) => result.push(n),
                }
            }
            CalculatorInput::Divide => {
                if result.len() < 2 {
                    return None;
                }
                let foo = operate(result.pop().unwrap(), result.pop().unwrap(), |x, y| y / x);
                match foo {
                    None => return None,
                    Some(n) => result.push(n),
                }
            }
        };
    }

    if result.len() != 1 {
        return None;
    }
    match result.first().unwrap() {
        CalculatorInput::Value(n) => Some(*n),
        _ => None,
    }
}
