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
    x: Option<CalculatorInput>,
    y: Option<CalculatorInput>,
    operation: fn(&i32, &i32) -> i32,
) -> Option<CalculatorInput> {
    match x {
        Some(Value(xx)) => match y {
            Some(Value(yy)) => Some(Value(operation(&xx, &yy))),
            _ => None,
        },
        _ => None,
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let result = inputs.iter().fold(vec![], |mut stack, input| {
        let new_val = match input {
            Value(n) => Some(Value(*n)),
            Add => stack
                .pop()
                .and_then(|a| stack.pop().and_then(|b| operate(a, b, |x, y| x + y))),
            Subtract => stack
                .pop()
                .and_then(|a| stack.pop().and_then(|b| operate(a, b, |x, y| y - x))),
            Multiply => stack
                .pop()
                .and_then(|a| stack.pop().and_then(|b| operate(a, b, |x, y| x * y))),
            Divide => stack
                .pop()
                .and_then(|a| stack.pop().and_then(|b| operate(a, b, |x, y| y / x))),
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
