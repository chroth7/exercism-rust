// very nice solution using `try_fold`:
// https://exercism.org/tracks/rust/exercises/rpn-calculator/solutions/insideoutclub
// this is more or less a copy of it

use crate::CalculatorInput::{Add, Divide, Multiply, Subtract, Value};

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn operate(stack: &mut Vec<i32>, operation: fn(&i32, &i32) -> i32) -> Option<i32> {
    stack
        .pop()
        .and_then(|y| stack.pop().map(|x| operation(&x, &y)))
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    inputs
        .iter()
        .try_fold(vec![], |mut stack, input| {
            match *input {
                Add => operate(&mut stack, |x, y| x + y),
                Subtract => operate(&mut stack, |x, y| x - y),
                Multiply => operate(&mut stack, |x, y| x * y),
                Divide => operate(&mut stack, |x, y| x / y),
                Value(x) => Some(x),
            }
            // note: this maps within the Option functor
            .map(|x| {
                stack.push(x);
                stack
            })
        })
        .and_then(|stack| match stack.as_slice() {
            [x] => Some(*x),
            _ => None,
        })
}
