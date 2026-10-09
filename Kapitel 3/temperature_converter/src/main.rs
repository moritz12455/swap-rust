use std::io;

fn main() {
    loop {
        println!("Enter a temperature like 100F or 37.5C (q to quit):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input.eq_ignore_ascii_case("q") || input.is_empty() {
            println!("Bye!");
            break;
        }

        let unit = input.chars().last().unwrap();
        let value = &input[..input.len() - unit.len_utf8()];

        let value: f64 = match value.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Could not read the number, try again.");
                continue;
            }
        };

        match unit {
            'F' | 'f' => println!("{value}°F = {:.2}°C", fahrenheit_to_celsius(value)),
            'C' | 'c' => println!("{value}°C = {:.2}°F", celsius_to_fahrenheit(value)),
            _ => println!("Unknown unit '{unit}', use C or F."),
        }
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}
