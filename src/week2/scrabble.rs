use std::cmp::Ordering;
use std::collections::HashMap;

/// Hashmap which associates each char to it's score.
struct PointsTable {
    /// Hashmap which associates each char to it's points.
    table: HashMap<char, i32>
}

impl PointsTable {
    /// Creates a new points table.
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

    /// Gets the number of points for a character.
    ///
    /// # Arguments
    /// * `ch` - The character.
    pub fn get(&self, ch: char) -> i32 {
        match self.table.get(&ch.to_ascii_lowercase()) {
            Some(&points) => points,
            None => 0
        }
    }

    /// Calculates the number of points for a string.
    ///
    /// # Arguments
    /// * `string` - The score for this string will be calculated.
    pub fn get_points(&self, string: &str) -> i32 {
        string.chars().fold(0, |score, ch| {
            score + self.get(ch)
        })
    }
}

pub fn main() {
    // Creates point table.
    let points = PointsTable::new();

    // Reads words for each player.
    let texts = [
        super::helpers::read_line("Player 1:").unwrap(),
        super::helpers::read_line("Player 2:").unwrap()
    ];

    // Calculates score for each player.
    let scores: Vec<i32> = texts.into_iter().map(|text| {
        points.get_points(&text)
    }).collect();

    // Compares and finds winner.
    println!("{}", match scores[0].cmp(&scores[1]) {
        Ordering::Greater => "Player 1 Wins!",
        Ordering::Equal => "Tie!",
        Ordering::Less => "Player 2 Wins!"
    });
}