use super::img::bmp::BMPFile24;

use std::env;

pub fn main() {
    // Reads input and output file.
    let args: Vec<String> = env::args().collect();

    let (input, output): (&str, &str) = match &args[1..] {
        [i, o] => (i, o),
        _ => panic!("Usage:\n./copy <input> <output>")
    };

    // Reads input file.
    let file = match BMPFile24::new(input) {
        Ok(f) => f,
        Err(e) => panic!("{:?}", e)
    };

    // Copies file to output file.
    match file.copy(output) {
        Err(e) => panic!("{:?}", e),
        _ => ()
    };
}