use std::env;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

// Memory block size.
const BLOCK_SIZE: usize = 512;

pub fn main() {
    // Reads input file and output directory from command line args.
    let args: Vec<String> = env::args().collect();
    
    let (input, output_folder): (&str, &str) = match &args[1..] {
        [i, o] => (i, o),
        _ => panic!("Usage:\n./recover <input> <output folder>")
    };

    // Opens input file and output directory.
    let file = File::open(input).expect("Could not open file");
    let mut out_folder = Path::new(output_folder);

    if !out_folder.is_dir() {
        fs::create_dir(output_folder).expect("Could not create output folder");
    }

    // Creates reader and buffer for each block in input file.
    let mut reader = BufReader::new(file);
    let mut buffer: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];

    // Creates a new output file and creates a writer for it.
    let mut i = 0;
    let mut writer = BufWriter::new(File::create(out_folder.join(filename(i)))
        .expect("Could not open output file"));

    // Reads until there are no more bytes left.
    while reader.read(&mut buffer).unwrap() > 0 {
        // Checks if the current block has a JPEG start sequence.
        if (buffer[0] == 0xff) && (buffer[1] == 0xd8) && (buffer[2] == 0xff) && (buffer[3] >> 4 == 0x0e) {
            // Drops current writer and creates a new file and a new writer for it.
            drop(writer);
            writer = BufWriter::new(File::create(out_folder.join(filename(i)))
                .expect("Could not open output file"));

            i += 1
        }

        writer.write(&buffer).unwrap();
    }
}

/// Formats a file's name depending on the file's number.
///
/// # Arguments
/// * `i` - The file's number.
fn filename(i: i32) -> String {
    format!("{:0>3}.jpeg", i)
}