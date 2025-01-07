use std::env;
use simple_calc::calculator;

fn main() {
    let mut args = env::args();
    match args.next() {
        Some(..) => (),
        None => eprintln!("Not enough arguments given!"),
    };
    
    let expression = calculator::parse_expression(args);
    let tokens = calculator::tokenize(&expression);
    let rpn = calculator::shunting_yard(tokens);
    let result = calculator::evaluate(rpn);

    println!("Output: {}", result);
}
