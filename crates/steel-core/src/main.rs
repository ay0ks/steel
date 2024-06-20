use std::{env, fs};
use steel_driver::lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let file = fs::read_to_string(filename).expect("Unable to read file");

    let mut lexer = Lexer::<8, 16>::new();
    let tokens = lexer.lex(file);

    println!("==============================");
    println!("Tokens: {:}", tokens.len());
    // iterate by 3 elements at once
    for chunk in tokens.chunks(3) {
        for token in chunk {
            println!("{:?}", token);
        }
        println!();
    }
}
