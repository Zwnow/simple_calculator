pub mod calculator {
    use std::str;
    use std::collections::HashMap;

    /// The shunting yard algorithm transforms an expression to the reverse polish notation.
    pub fn shunting_yard(expression: String) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        let mut operators: Vec<String> = vec![];

        // Define operator precedence and associativity
        let precedence: HashMap<u8, u8> = HashMap::from([
            (b'+', 1), 
            (b'-', 1),
            (b'*', 2),
            (b'/', 2),
        ]);

        let mut position = 0;
        let mut prev_was_operator = true;
        let bytes = expression.as_bytes();

        while position < expression.len() {
            println!("{:?} {:?}", output, operators);
            let start = position;
            match bytes[position] {
                // Add number to the output
                b'0'..=b'9' => {
                    while position < expression.len() && u8::is_ascii_digit(&bytes[position]) {
                        position += 1;
                    }

                    output.push(expression[start..position].to_string());
                    prev_was_operator = false;
                },
                b'+' | b'*' | b'/' => {
                    if operators.is_empty() {
                        operators.push(expression[position..position + 1].to_string());
                    } else {
                        let mut precedence_flag = true;
                        while precedence_flag {
                            let top = operators.pop().unwrap();
                            let precedence_a = precedence.get(&bytes[position]);
                            let precedence_b = precedence.get(&top.as_bytes()[0]);

                            if precedence_b >= precedence_a {
                                output.push(top);
                            } else {
                                precedence_flag = false;
                                operators.push(top);
                                operators.push(expression[position..position + 1].to_string());
                            }
                        }
                    }
                    prev_was_operator = true;
                    position += 1;
                },
                b'-' => {
                    if prev_was_operator && bytes[position + 1] >= b'0' && bytes[position + 1] <= b'9' {
                        position += 1;
                        while position < expression.len() && u8::is_ascii_digit(&bytes[position]) {
                            position += 1;
                        }
                        output.push(expression[start..position].to_string());
                        prev_was_operator = false;
                    } else {
                        if operators.is_empty() {
                            operators.push(expression[position..position + 1].to_string());
                        } else {
                            let mut precedence_flag = true;
                            while precedence_flag {
                                let top = operators.pop().unwrap();
                                let precedence_a = precedence.get(&bytes[position]);
                                let precedence_b = precedence.get(&top.as_bytes()[0]);

                                if precedence_b >= precedence_a {
                                    output.push(top);
                                } else {
                                    precedence_flag = false;
                                    operators.push(top);
                                    operators.push(expression[position..position + 1].to_string());
                                }
                            }
                        }
                        prev_was_operator = true;
                        position += 1;
                    }
                },
                b'(' | b')' => {
                    if bytes[position] == b'(' {
                        operators.push(expression[position..position + 1].to_string());
                        prev_was_operator = true;
                    } else {
                        while operators.len() > 0 {
                            let top = operators.pop().unwrap();
                            if top == "(".to_string() {
                                break;
                            } else {
                                output.push(top);
                            }
                        }
                    }
                    position += 1;
                },
                _ => ()
            }
        }

        while !operators.is_empty() {
            output.push(operators.pop().unwrap());
        }
            
        output
    }

    /// Evaluates an expression in the form of the reverse polish notation.
    pub fn evaluate(expr: Vec<String>) -> f64 {
        println!("{:?}", expr);
        let mut stack: Vec<f64> = vec![];

        for item in expr {
            let w: &str = &item;
            match w {
                "+" => {
                    let a: f64 = stack.pop().expect("Failed to pop from stack");
                    let b: f64 = stack.pop().expect("Failed to pop from stack");
                    stack.push(b + a);
                },
                "-" => {
                    let a: f64 = stack.pop().expect("Failed to pop from stack");
                    if stack.len() != 0 {
                        let b: f64 = stack.pop().expect("Failed to pop from stack");
                        stack.push(b - a);                
                    } else {
                        stack.push(a);
                    }
                },
                "*" => {
                    let a: f64 = stack.pop().expect("Failed to pop from stack");
                    let b: f64 = stack.pop().expect("Failed to pop from stack");
                    stack.push(b * a);                
                },
                "/" => {
                    let a: f64 = stack.pop().expect("Failed to pop from stack");
                    let b: f64 = stack.pop().expect("Failed to pop from stack");
                    stack.push(b / a);                },
                _ => {
                    let number: f64 = item.parse().expect("Failed to parse number"); 
                    stack.push(number);
                }
            }
            println!("{:?}", stack);
        }

        stack.pop().unwrap()
    }
}
