use core::num;
use std::collections::HashMap;
use std::env;
use std::fmt::{Debug, Formatter};
use std::fmt;

use super::helpers;

/// The given candidate does not exist.
struct CandidateNotFoundError;

impl Debug for CandidateNotFoundError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "A Candidate was not found")
    }
}

/// Hashmap which associates each candidate to its number of votes.
struct CandidateTable {
    /// Hashmap which associates each candidate to its number of votes.
    table: HashMap<String, u32>
}

impl CandidateTable {
    /// Creates a new candidate table containing the given candidates.
    ///
    /// # Arguments
    /// * `candidates` - The election's candidates.
    pub fn new(candidates: &[String]) -> CandidateTable {
        CandidateTable {
            table: (candidates)
                .into_iter()
                .map(|candidate|  (candidate.clone(), 0))
                .collect()
        }
    }

    /// Votes for the given candidate.
    ///
    /// # Arguments
    /// * `name` - The candidate's name.
    pub fn vote(&mut self, name: &str) -> Result<(), CandidateNotFoundError> {
        match self.table.get_mut(name) {
            Some(votes) => {
                *votes += 1;
                Ok(())
            },
            None => Err(CandidateNotFoundError)
        }
    }

    /// Finds the winner of the election.
    /// Returns a tuple with the winner's name and the number of votes.
    pub fn winner(&self) -> Result<(&str, u32), CandidateNotFoundError> {
        self.table
            .iter()
            .fold(Err(CandidateNotFoundError), |winner, (candidate, votes)| {
                match winner {
                    Ok((name, winner_votes)) => if *votes > winner_votes {
                        Ok((candidate, *votes))
                    } else {
                        Ok((name, winner_votes))
                    },
                    _ => Ok((candidate, *votes))
                }
        })
    }
}

pub fn main() {
    // Reads candidates from command line args.
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Usage:\n ./plurality <candidate1> <candidate2> <...> <candidateN>\nMinimun number of candidates is 2");
    }

    // Creates candidate table.
    let mut table: CandidateTable = CandidateTable::new(&args[1..]);

    // Reads number of voters.
    let number_of_voters: i32 = loop {
        match helpers::read_line("Number of voters: ").unwrap().parse::<i32>() {
            Ok(n) => break n,
            _ => eprintln!("The number of voters should be and integer")
        };
    };

    // Get votes for each voter.
    vote(&mut table, number_of_voters);
    println!("\nWinner is {}", table.winner().unwrap().0);
}

/// Votes the given number of times.
///
/// # Arguments
/// * `table` - The candidate table. Votes for candidates which are not in this table are not allowed.
/// * `number_of_voters` - Number of voters in the election.
fn vote(table: &mut CandidateTable, number_of_voters: i32) {
    for i in 0..number_of_voters {
        let candidate = helpers::read_line("Vote: ").unwrap();

        if let Err(_) = table.vote(&candidate) {
            eprintln!("Invalid Vote");
        };
    }
}
