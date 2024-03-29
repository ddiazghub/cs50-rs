use super::img::bmp::{ImageFilter, BMPFile24};
use std::env;

pub fn main() {
    // Reads filter to apply, input file and output filename from command line args.
    let args: Vec<String> = env::args().collect();

    // Parses image filter.
    let (filter, input, output): (ImageFilter, &str, &str) = match &args[1..] {
        [f, i, o] => {
            let fi = match f.as_str() {
                "-g" => ImageFilter::GrayScale,
                "-s" => ImageFilter::Sepia,
                "-r" => ImageFilter::Reflection,
                "-b" => ImageFilter::Blur,
                "-e" => ImageFilter::Edges,
                _ => panic!("Unknown filter type")
            };

            (fi, i, o)
        },
        _ => panic!("Usage:\n./filter <filter type> <input> <output>")
    };

    // Reads BMP file.
    let file = match BMPFile24::new(input) {
        Ok(f) => f,
        Err(e) => panic!("{:?}", e)
    };

    // Applies filter to image.
    match file.filter(output, filter) {
        Err(e) => panic!("{:?}", e),
        _ => ()
    };
}