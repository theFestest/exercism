#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {

    // let mut stack: Vec<CalculatorInput> = Vec::new();
    let mut stack: Vec<i32> = Vec::new();
    // for i in inputs {
    //     stack.push(i.clone())
    // }

    let mut val_a;
    let mut val_b;

    // while !stack.is_empty(){
    //     match stack.pop() {
    //         None => return None, // Didn't get a first value
    //         Some(CalculatorInput::Value(x)) => val_a = x.clone(),
    //         Some(_) => return None, // Bad operator position
    //     }

    //     match stack.pop() {
    //         None => return Some(val_a), // Didn't get a second value, evaluate to the first
    //         Some(CalculatorInput::Value(x)) => val_b = x.clone(),
    //         Some(_) => return None, // Bad operator position
    //     }

    //     match stack.pop() {
    //         None => return None, // Missing operator
    //         Some(CalculatorInput::Value(_)) => return None, // Bad value position
    //         Some(CalculatorInput::Add) => stack.push(CalculatorInput::Value(val_a + val_b)),
    //         Some(CalculatorInput::Subtract) => stack.push(CalculatorInput::Value(val_a - val_b)),
    //         Some(CalculatorInput::Multiply) => stack.push(CalculatorInput::Value(val_a * val_b)),
    //         Some(CalculatorInput::Divide) => stack.push(CalculatorInput::Value(val_a / val_b)),
    //     }
    // }

    // let mut inputs = inputs.iter();

    // for (val_a, val_b) in inputs {

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

        // let val_a = stack.pop().unwrap();
        // let val_b = stack.pop().unwrap();

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

        // stack.push(CalculatorInput::Value(result));
        println!("Intermediate result: {}", result);
        performed_computation = true;
        stack.push(result);
    }

    stack.pop()
}
