use std::{env, fs, io};
use std::collections::HashSet;
use std::io::Write;
use rand::seq::IteratorRandom;
use reqwest;
use figlet_rs::FIGfont;

pub fn main() {
    // Reads file containing font names.
    let fonts: HashSet<String> = fs::read_to_string("fonts.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();

    // Reads font name from command line args. If no font name is supplied, chooses a random font.
    let mut args = env::args().skip(1);

    let font = match (args.next(), args.next()) {
        (Some(flag), Some(font)) => {
            if flag == "-f" || flag == "--font" && fonts.contains(&font) {
                font
            } else {
                panic!("Invalid usage");
            }
        },
        (None, _) => {
            (&fonts).into_iter()
                .map(|item| item.as_str())
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string()
        },
        _ => panic!("Invalid usage")
    };

    // Downloads font from figlet's font database.
    let url = format!("http://www.figlet.org/fonts/{}.flf", font);
    let downloaded = reqwest::blocking::get(url).unwrap().text().unwrap();
    let fig_font = FIGfont::from_content(&downloaded).unwrap();

    // Reads text to print with the chosen figlet font.
    print!("Input: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Prints the input text in the target font.
    println!("Output:");
    println!("{}", fig_font.convert(&input).unwrap());
}