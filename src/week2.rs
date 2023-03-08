use super::helpers;

pub mod scrabble {
    use std::cmp::Ordering;
    use std::collections::HashMap;

    struct PointsTable {
        table: HashMap<char, i32>
    }

    impl PointsTable {
        pub fn new() -> Self {
            PointsTable {
                table: HashMap::from([
                    ('a', 1),
                    ('b', 3),
                    ('c', 3),
                    ('d', 2),
                    ('e', 1),
                    ('f', 4),
                    ('g', 2),
                    ('h', 4),
                    ('i', 1),
                    ('j', 8),
                    ('k', 5),
                    ('l', 1),
                    ('m', 3),
                    ('n', 1),
                    ('o', 1),
                    ('p', 3),
                    ('q', 10),
                    ('r', 1),
                    ('s', 1),
                    ('t', 1),
                    ('u', 1),
                    ('v', 4),
                    ('w', 4),
                    ('x', 8),
                    ('y', 4),
                    ('z', 10),
                ])
            }
        }

        pub fn get(&self, ch: char) -> i32 {
            match self.table.get(&ch.to_ascii_lowercase()) {
                Some(&points) => points,
                None => 0
            }
        }

        pub fn get_points(&self, string: &str) -> i32 {
            string.chars().fold(0, |score, ch| {
                score + self.get(ch)
            })
        }
    }

    pub fn main() {
        let points = PointsTable::new();

        let texts = [
            super::helpers::read_line("Player 1:").unwrap(),
            super::helpers::read_line("Player 2:").unwrap()
        ];

        let scores: Vec<i32> = texts.into_iter().map(|text| {
            points.get_points(&text)
        }).collect();

        println!("{}", match scores[0].cmp(&scores[1]) {
            Ordering::Greater => "Player 1 Wins!",
            Ordering::Equal => "Tie!",
            Ordering::Less => "Player 2 Wins!"
        });
    }
}

pub mod readability {
    pub fn main() {
        let text = super::helpers::read_line("Text: ").unwrap();
        let (letters, sentences, words) = letters_sentences_words(&text);
        let index = coleman_liau_index(letters, sentences, words);
        
        match index {
            1..=15 => println!("Grade {}", index),
            _ if index < 1 => println!("Before Grade 1"),
            _ => print!("Grade 16+")
        };
    }

    fn letters_sentences_words(text: &str) -> (i32, i32, i32) {
        let mut lsw = (0, 0, 0);
        let mut word = false;

        for ch in text.chars() {
            match ch {
                ' ' if word => {
                    word = false;
                },
                '.' | '!' | '?' => {
                    lsw.1 += 1;
                    word = false;
                },
                'a'..='z' | 'A'..='Z' => {
                    lsw.0 += 1;

                    if !word {
                        lsw.2 += 1;
                        word = true;
                    }
                },
                _ => ()
            }
        }

        lsw
    }

    fn coleman_liau_index(letters: i32, sentences: i32, words: i32) -> i32 {
        let letters_per_word: f64 = letters as f64 / words as f64;
        let sentences_per_word: f64 = sentences as f64 / words as f64;

        (100.0 * (0.0588 * letters_per_word - 0.296 * sentences_per_word) - 15.8).round() as i32
    }
}

pub mod caesar {
    use std::env;

    static A_: u32 = 'a' as u32;
    static A: u32 = 'A' as u32;
    
    pub fn main() {
        let args: Vec<String> = env::args().collect();

        if args.len() < 3 {
            panic!("Usage:\n cargo run -- <plaintext> <key>");
        }

        let text = (&args[1..args.len() - 1]).join(" ");

        let key = match args[args.len() - 1].parse::<i32>() {
            Ok(n) => ((n % 26) + 26) % 26,
            _ => panic!("The key should be an integer")
        };

        println!("{}", caesar(&text, key));
    }

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
}

pub mod substitution {
    use std::env;
    use std::collections::HashMap;

    static A_: u32 = 'a' as u32;
    static A: u32 = 'A' as u32;

    pub fn main() {
        let args: Vec<String> = env::args().collect();

        if args.len() < 3 {
            panic!("Usage:\n cargo run -- <plaintext> <key>");
        }

        let text = (&args[1..args.len() - 1]).join(" ");
        let key = &args[args.len() - 1];

        match substitution_cipher(&text, key) {
            Ok(ciphertext) => println!("{}", ciphertext),
            Err(err) => panic!("{}", err)
        }
    }

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
}