use std::env;
use simple_calc::calculator;

fn main() {
    let mut args = env::args();
    args.next().expect("Not enough arguments given.");
    
    let result = calculator::evaluate(calculator::shunting_yard(args.collect::<Vec<String>>().join("")));
    println!("Output: {:?}", result);
}
