pub fn main() {
    // Reads text from stdin then counts letters, sentences and words.
    let text = super::helpers::read_line("Text: ").unwrap();
    let (letters, sentences, words) = letters_sentences_words(&text);

    // Calculates cl index and prints grade.
    let index = coleman_liau_index(letters, sentences, words);

    match index {
        1..=15 => println!("Grade {}", index),
        _ if index < 1 => println!("Before Grade 1"),
        _ => print!("Grade 16+")
    };
}

/// Counts the number of letters, sentences and words in a text.
///
/// # Arguments
/// * `text` - The text for which to count letters, sentences and words.
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

/// Calculates the coleman liau index for a text based on the given data.
///
/// # Arguments
/// * `letters` - Number of letters in the text.
/// * `sentences` - Number of sentences in the text.
/// * `words` - Number of words in the text.
fn coleman_liau_index(letters: i32, sentences: i32, words: i32) -> i32 {
    let letters_per_word: f64 = letters as f64 / words as f64;
    let sentences_per_word: f64 = sentences as f64 / words as f64;

    (100.0 * (0.0588 * letters_per_word - 0.296 * sentences_per_word) - 15.8).round() as i32
}