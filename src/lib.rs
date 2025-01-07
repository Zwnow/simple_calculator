pub mod calculator {
    pub fn parse_expression(args: impl Iterator<Item = String>) -> String 
    {
        args.collect::<Vec<String>>().join("")
    }

    pub fn tokenize(expression: &str) -> Vec<Token> {
        let tokens: Vec<Token> = Vec::new();
        let mut pos = 0;
        let bytes = expression.as_bytes();

        for i in 0..bytes.len() {
            match bytes[i] {
                // 0..9
                48..=57 => (),

                b'(' | b')' | b'+' | b'*' | b'/' => (),

                b'-' => (),

                // Illegal
                _ => (),
            }
        }

        tokens
    }

    #[derive(Debug, PartialEq)]
    pub struct Token<'a> {
        pub token_type: TokenType,
        pub value: &'a str,
    }

    impl Token<'_> {
        fn new(token_type: TokenType, value: &str) -> Token<'_> {
            Token { token_type, value }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum TokenType {
        Number,
        Operator
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
            Token { token_type: TokenType::Number, value: "1" },
            Token { token_type: TokenType::Operator, value: "1" },
            Token { token_type: TokenType::Number, value: "1" },
        ];

        let got = calculator::tokenize(expression);

        assert_eq!(expected, got);
    }
}
