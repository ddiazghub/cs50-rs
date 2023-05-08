use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use csv::ReaderBuilder;
use itertools::Itertools;

/// Single DNA record. A Hashmap which contains the name of the person and the longest consecutive sequence of an STR.
type DnaRecord = HashMap<String, String>;

/// Reads the database file. Returns a Vector containing each record in the DNA database.
///
/// # Arguments
/// * `filename` - Name of the database file.
fn read_database(filename: &str) -> Vec<DnaRecord> {
    let reader = BufReader::new(File::open(filename).unwrap());
    let mut csv_reader = ReaderBuilder::new().from_reader(reader);

    csv_reader.deserialize().collect::<Result<_, _>>().unwrap()
}

/// Reads the DNA sequence file. Returns the sequence as a string.
///
/// # Arguments
/// * `filename` - Name of the sequence file.
fn read_sequence(filename: &str) -> String {
    let mut reader = BufReader::new(File::open(filename).unwrap());
    let mut sequence = String::new();
    reader.read_to_string(&mut sequence).unwrap();

    sequence
}

/// Finds the longest consecutive sequence of an STR in a DNA sequence. Returns the number of times that the STR is repeated.
///
/// # Arguments
/// * `str_sequence` - The STR sequence.
/// * `dna_sequence` - DNA sequence where the STR will be found.
fn longest_match(str_sequence: &str, dna_sequence: &str) -> usize {
    let str_bytes = str_sequence.as_bytes();
    let dna_bytes = dna_sequence.as_bytes();
    let len = str_bytes.len();
    let end = dna_bytes.len() - len;
    let mut max_repeats = 0;
    let mut i = 0;

    while i < end {
        let mut repeats = 0;

        while i < end && str_bytes == &dna_bytes[i..i + len] {
            i += len;
            repeats += 1;
        }

        if repeats > max_repeats {
            max_repeats = repeats;
        }

        i += 1;
    }

    max_repeats
}

pub fn main() {
    // Reads from database file and DNA sequence file.
    let (database_file, sequence_file): (String, String) = env::args().skip(1).collect_tuple().unwrap();
    let database = read_database(&database_file);
    let sequence = read_sequence(&sequence_file);

    // Finds the longest consecutive sequence of each STR in the DNA sequence.
    let longest_matches: Vec<_> = database.first()
        .expect("Empty database.")
        .keys()
        .map(|key| key.clone())
        .filter(|key| key != "name")
        .map(|str_sequence| {
            let repeats = longest_match(&str_sequence, &sequence);
            (str_sequence, repeats)
        })
        .collect();

    // Finds the if the DNA sequence belongs to a person in the database.
    for record in database {
        if longest_matches.iter().all(|(str_seq, repeats)| record[str_seq].parse::<usize>().unwrap() == *repeats) {
            println!("{}", record["name"]);
            return
        }
    }

    println!("No match")
}