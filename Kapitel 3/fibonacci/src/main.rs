use std::io;

fn main() {
    println!("Which Fibonacci number do you want? (0-186)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u32 = match input.trim().parse() {
        Ok(n) if n <= 186 => n,
        _ => {
            println!("Please enter a whole number between 0 and 186.");
            return;
        }
    };

    println!("fib({n}) = {}", fibonacci(n));

    print!("First 15:");
    for i in 0..15 {
        print!(" {}", fibonacci(i));
    }
    println!();
}

fn fibonacci(n: u32) -> u128 {
    if n == 0 {
        return 0;
    }
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    for _ in 1..n {
        let next = a + b;
        a = b;
        b = next;
    }
    b
}
