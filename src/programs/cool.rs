use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{} is cool", args[1]);
}
