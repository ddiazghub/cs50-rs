use std::{env, fs, io};
use std::collections::HashSet;
use std::io::Write;
use rand::seq::IteratorRandom;
use reqwest;
use figlet_rs::FIGfont;

pub fn main() {
    let fonts: HashSet<String> = fs::read_to_string("fonts.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();

    let mut args = env::args();
    args.next();

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

    let url = format!("http://www.figlet.org/fonts/{}.flf", font);
    let downloaded = reqwest::blocking::get(url).unwrap().text().unwrap();
    let fig_font = FIGfont::from_content(&downloaded).unwrap();
    let mut input = String::new();

    print!("Input: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    println!("Output:");
    println!("{}", fig_font.convert(&input).unwrap());
}