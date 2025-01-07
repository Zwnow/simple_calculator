pub mod calculator {
    pub fn parse_expression(args: impl Iterator<Item = String>) -> String 
    {
        args.collect::<Vec<String>>().join("")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use calculator;

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
}
