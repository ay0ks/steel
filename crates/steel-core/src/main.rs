use std::{env, fs};
use steel_driver::parser::TextParser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let file = fs::read_to_string(filename).expect("Unable to read file");

    let mut parser = TextParser::new();
    let tokens = parser.parse(file.clone().into());

    println!("==============================");
    println!("Tokens: {:}", tokens.len());
    for token in tokens {
        println!("{:?}", token);
    }
}
