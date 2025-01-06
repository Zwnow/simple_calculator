pub mod calculator {
    pub fn parse_expression<I>(args: I) -> String 
    where
        I: IntoIterator<Item = String>,
    {
        args.into_iter()
            .fold(String::new(), |mut expr, part| {
                expr.push_str(part.trim());
                expr
            })
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
