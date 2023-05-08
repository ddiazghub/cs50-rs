use std::env;

/// Unicode code point for the letter a.
static A_: u32 = 'a' as u32;

/// Unicode code point for the letter A.
static A: u32 = 'A' as u32;

pub fn main() {
    // Reads plaintext and key from stdin.
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Usage:\n cargo run -- <plaintext> <key>");
    }

    let text = (&args[1..args.len() - 1]).join(" ");

    let key = match args[args.len() - 1].parse::<i32>() {
        Ok(n) => ((n % 26) + 26) % 26,
        _ => panic!("The key should be an integer")
    };

    // Encrypts text and prints.
    println!("{}", caesar(&text, key));
}

/// Encrypts an input text using caesar encryption and shifting each character by a specified amount.
///
/// # Arguments
/// * `text` - The plaintext to encrypt.
/// * `key` - The number of characters that each character will be shifted.
fn caesar(text: &str, key: i32) -> String {
    let ciphertext = text.chars().map(|ch| {
        let shifted = match ch {
            'a'..='z' => A_ + (((ch as u32 - A_) as i32 + key) % 26) as u32,
            'A'..='Z' => A + (((ch as u32 - A) as i32 + key) % 26) as u32,
            _ => ch as u32
        };

        char::from_u32(shifted).unwrap()
    }).collect::<String>();

    ciphertext
}