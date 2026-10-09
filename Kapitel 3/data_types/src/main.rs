use std::io;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Parsed: {guess}");

    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    println!("{decimal} {hex} {octal} {binary} {byte}");

    let big: u8 = 255;
    println!("wrapping_add: {}", big.wrapping_add(1));
    println!("checked_add: {:?}", big.checked_add(1));
    println!("overflowing_add: {:?}", big.overflowing_add(1));
    println!("saturating_add: {}", big.saturating_add(1));

    let x = 2.0;
    let y: f32 = 3.0;
    println!("floats: {x} {y}");

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;
    println!("{sum} {difference} {product} {quotient} {truncated} {remainder}");

    let t = true;
    let f: bool = false;
    println!("bools: {t} {f}");

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("chars: {c} {z} {heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("tuple destructured: {a} {b} {c}");
    println!("tuple indexed: {} {} {}", tup.0, tup.1, tup.2);

    let months = [
        "January", "February", "March", "April", "May", "June", "July", "August", "September",
        "October", "November", "December",
    ];
    let zeros = [0; 5];
    println!("{} months, zeros: {zeros:?}", months.len());

    println!("Please enter an array index (0-11).");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    match months.get(index) {
        Some(month) => println!("The value of the element at index {index} is: {month}"),
        None => println!("Index {index} is out of bounds, no panic today!"),
    }
}
