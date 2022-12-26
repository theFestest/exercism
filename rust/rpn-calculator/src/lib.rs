#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {

    let mut stack: Vec<i32> = Vec::new();
    let mut val_a;
    let mut val_b;
    let mut inputs = inputs.iter();

    // Push first value, has to be a value
    match inputs.next() {
        None => return None,
        Some(CalculatorInput::Value(x)) => stack.push(*x),
        _ => return None,
    };

    // Push second value, has to be a value
    match inputs.next() {
        None => return Some(stack.pop().unwrap()),
        Some(CalculatorInput::Value(x)) => stack.push(*x),
        _ => return None,
    };

    let mut performed_computation = false;

    while !stack.is_empty() {

        let result = match inputs.next() {
            None => {
                // No more inputs, evaluate the stack
                match stack.pop() {
                    None => return None,
                    Some(x) => {
                        // Hacky check to make sure we actually performed a computation
                        //  (not too many operands)
                        if performed_computation {
                            return Some(x);
                        } else {
                            return None;
                        }
                    },
                }
            },
            Some(CalculatorInput::Value(x)) => x.clone(),
            Some(CalculatorInput::Add) => {
                stack.pop().unwrap() + stack.pop().unwrap()
            },
            Some(CalculatorInput::Subtract) => {
                val_b = stack.pop().unwrap();
                val_a = stack.pop().unwrap();

                val_a - val_b
            },
            Some(CalculatorInput::Multiply) => {
                stack.pop().unwrap() * stack.pop().unwrap()
            },
            Some(CalculatorInput::Divide) => {
                val_b = stack.pop().unwrap();
                val_a = stack.pop().unwrap();

                val_a / val_b
            },
        };

        // println!("Intermediate result: {}", result);
        performed_computation = true;
        stack.push(result);
    }

    stack.pop()
}
