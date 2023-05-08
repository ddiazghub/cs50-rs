use std::io;

pub fn main() {
    // Reads greeting from stdin.
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read stdin.");
    let greeting = input.trim_end();
    let mut chars = greeting.chars();

    // If the greeting is "hello" prints $100, if the greeting begins with "h" prints $20 otherwise prints $0
    let money = match chars.next() {
        Some(ch) if ch.to_ascii_lowercase() == 'h' => {
            let remaining: String = chars.collect();

            match remaining.to_ascii_lowercase().as_str() {
                "ello" => 100,
                _ => 20
            }
        },
        _ => 0
    };

    println!("${money}")
}