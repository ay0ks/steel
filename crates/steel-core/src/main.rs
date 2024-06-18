use std::env;
use steel_driver::parser::TextParser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut parser = TextParser::new(args[1].chars().collect());
    let tokens = parser.parse();

    for token in tokens {
        println!("{:?}", token);
    }
}
