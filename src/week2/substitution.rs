use std::env;
use std::collections::HashMap;

/// Unicode code point for the letter a.
static A_: u32 = 'a' as u32;

/// Unicode code point for the letter A.
static A: u32 = 'A' as u32;

pub fn main() {
    /// Reads plaintext and key from stdin.
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Usage:\n cargo run -- <plaintext> <key>");
    }

    let text = (&args[1..args.len() - 1]).join(" ");
    let key = &args[args.len() - 1];

    // Encrypts text and prints.
    match substitution_cipher(&text, key) {
        Ok(ciphertext) => println!("{}", ciphertext),
        Err(err) => panic!("{}", err)
    }
}

/// Encrypts an input text using substitution encryption.
/// Each character will be mapped to a different character.
///
/// # Arguments
/// * `text` - The plaintext to encrypt.
/// * `key` - A string which contains the char mappings.
pub fn substitution_cipher(text: &str, key: &str) -> Result<String, String> {
    let table: HashMap<char, char> = key
        .chars()
        .enumerate()
        .fold(HashMap::new(), |mut table, (i, ch) | {
            if !ch.is_alphabetic() {
                return table;
            }

            let original = (char::from_u32(A_ + i as u32).unwrap(), char::from_u32(A + i as u32).unwrap());
            table.insert(original.0, ch.to_ascii_lowercase());
            table.insert(original.1, ch);
            table
        });

    if table.len() < 26 * 2 {
        return Err(String::from("Invalid Key"));
    }

    let ciphertext: String = text.chars().map(|ch| {
        match table.get(&ch) {
            Some(&subs) => subs,
            None => ch
        }
    }).collect();

    Ok(ciphertext)
}