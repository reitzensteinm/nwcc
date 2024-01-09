use std::env;
use std::str::FromStr;

fn fibonacci(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let i = usize::from_str(&args[1]).unwrap();

    println!("fib({}) = {}", i, fibonacci(i));
}
