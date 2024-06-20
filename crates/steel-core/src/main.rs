use std::{env, fs};
use steel_driver::driver::Driver;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let mut driver = Driver::new();
    let mut module = driver.compile(filename.to_string()).unwrap();

    println!("{:?}", module);
}
