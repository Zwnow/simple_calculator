pub mod calculator {
    use std::str;
    use std::collections::HashMap;

    pub fn parse_expression(args: impl Iterator<Item = String>) -> String 
    {
        args.collect::<Vec<String>>().join("")
    }

    pub fn tokenize(expression: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let bytes = expression.as_bytes();

        let mut i = 0;
        while i < bytes.len() {
            match bytes[i] {
                // 0..9
                48..=57 => {
                    let (num, end) = parse_number(i, &bytes);
                    i = end;
                    tokens.push(Token::new(TokenType::Number, num));
                },
                b'+' | b'*' | b'/' => {
                    tokens.push(Token::new(TokenType::Operator, parse_operator(i, &bytes)));
                    i += 1;
                },
                b'(' | b')' => {
                    tokens.push(Token::new(TokenType::Parenthesis, parse_operator(i, &bytes)));
                    i += 1;
                },
                b'-' => {
                    let is_number = bytes[i + 1] >= 48 && bytes[i + 1] <= 57;
                    if is_number {
                        let (num, end) = parse_number(i, &bytes);
                        i = end;
                        tokens.push(Token::new(TokenType::Number, num));
                    } else {
                        tokens.push(Token::new(TokenType::Operator, parse_operator(i, &bytes)));
                        i += 1;
                    }
                },
                // Illegal
                _ => panic!("Invalid character in expression"),
            }
        }

        tokens
    }

    pub fn shunting_yard(tokens: Vec<Token>) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        let mut operators: Vec<String> = vec![];

        // Define operator precedence and associativity
        let precedence: HashMap<&str, (u8, bool)> = HashMap::from([
            ("+", (1, true)), // (precedence, left-associative)
            ("-", (1, true)),
            ("*", (2, true)),
            ("/", (2, true)),
        ]);

        for token in tokens {
            match token.token_type {
                TokenType::Number => {
                    output.push(token.value);
                }
                TokenType::Operator => {
                    // Pop operators with higher or equal precedence
                    while let Some(top_op) = operators.last() {
                        if let Some(&(top_prec, top_left_assoc)) = precedence.get(top_op.as_str()) {
                            let &(curr_prec, _) = precedence
                                .get(token.value.as_str())
                                .expect("Unknown operator");

                            if top_prec > curr_prec || (top_prec == curr_prec && top_left_assoc) {
                                output.push(operators.pop().unwrap());
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    // Push the current operator to the stack
                    operators.push(token.value);
                }
                TokenType::Parenthesis => {
                    if token.value == "(" {
                        operators.push(token.value);
                    } else if token.value == ")" {
                        // Pop to output until a left parenthesis is found
                        while let Some(top_op) = operators.pop() {
                            if top_op == "(" {
                                break;
                            }
                            output.push(top_op);
                        }
                    }
                }
            }
        }

        // Pop any remaining operators to the output
        while let Some(op) = operators.pop() {
            output.push(op);
        }

        output
    }

    pub fn evaluate(expr: Vec<String>) -> f64 {
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
                    let b: f64 = stack.pop().expect("Failed to pop from stack");
                    stack.push(b - a);                },
                "*" => {
                    let a: f64 = stack.pop().expect("Failed to pop from stack");
                    let b: f64 = stack.pop().expect("Failed to pop from stack");
                    stack.push(b * a);                },
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

        let mut r = 0.0;
        for num in stack {
            r += num;
        }

        r
    }

    #[derive(Debug, PartialEq)]
    pub struct Token {
        pub token_type: TokenType,
        pub value: String,
    }

    impl Token {
        fn new(token_type: TokenType, value: String) -> Token {
            Token { token_type, value }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum TokenType {
        Number,
        Operator,
        Parenthesis
    }

    fn parse_number(pos: usize, bytes: &[u8]) -> (String, usize) {
        let mut end = pos + 1;

        while end < bytes.len() && bytes[end].is_ascii_digit() {
            end += 1;
        }

        let num = match str::from_utf8(&bytes[pos..end]) {
            Ok(v) => v,
            Err(_) => panic!("Invalid character in byte sequence!"),
        };

        (num.to_string(), end)
    }

    fn parse_operator(pos: usize, bytes: &[u8]) -> String {
        match str::from_utf8(&bytes[pos..pos + 1]) {
            Ok(v) => v.to_string(),
            Err(_) => panic!("Tried to parse invalid operator!"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use calculator;
    use calculator::Token;
    use calculator::TokenType;

    #[test]
    fn test_parse_expression() {
        let expected = "1+1";
        let args = vec![
            String::from("1"),
            String::from("+1")]
            .into_iter();

        let result = calculator::parse_expression(args);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_tokenize() {
        let expression = "1+1";
        let expected: Vec<Token> = vec![
            Token { token_type: TokenType::Number, value: "1".to_string() },
            Token { token_type: TokenType::Operator, value: "1".to_string() },
            Token { token_type: TokenType::Number, value: "1".to_string() },
        ];

        let got = calculator::tokenize(expression);

        assert_eq!(expected, got);
    }
}
