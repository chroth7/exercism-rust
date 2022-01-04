use crate::CalculatorInput::{Add, Divide, Multiply, Subtract, Value};

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn operate(
    input_pair: Option<(Option<CalculatorInput>, Option<CalculatorInput>)>,
    operation: fn(&i32, &i32) -> i32,
) -> Option<CalculatorInput> {
    match input_pair {
        Some((x, y)) => {
            if let (Some(Value(x_val)), Some(Value(y_val))) = (x, y) {
                Some(Value(operation(&x_val, &y_val)))
            } else {
                None
            }
        }
        None => None,
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let result = inputs.iter().fold(vec![], |mut stack, input| {
        // handle the Value
        if let Value(n) = input {
            stack.push(Some(Value(*n)));
            return stack;
        }
        let pair = stack
            .pop()
            .and_then(|a| stack.pop().and_then(|b| Some((a, b))));

        let new_val = match input {
            Add => operate(pair, |x, y| x + y),
            Subtract => operate(pair, |x, y| y - x),
            Multiply => operate(pair, |x, y| x * y),
            Divide => operate(pair, |x, y| y / x),
            _ => None,
        };
        stack.push(new_val);
        stack
    });

    if result.len() != 1 {
        return None;
    }
    match result.first().unwrap() {
        Some(Value(n)) => Some(*n),
        _ => None,
    }
}
