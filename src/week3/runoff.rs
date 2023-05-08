use std::env;
use super::helpers;
use std::collections::{HashMap, HashSet};
use std::i32;

/// A candidate participating in the current election.
struct Candidate {
    /// The candidate's name.
    pub name: String,
    /// The number of votes that the candidate has.
    pub votes: i32,
    /// Whether the candidate has been eliminated or not.
    pub eliminated: bool
}

impl Candidate {
    /// Creates a new candidate with the given name.
    ///
    /// # Arguments
    /// * `name` - The candidate's name.
    pub fn new(name: String) -> Self {
        Candidate {
            name,
            votes: 0,
            eliminated: false
        }
    }
}

impl Clone for Candidate {
    fn clone(&self) -> Self {
        Candidate {
            name: self.name.clone(),
            ..*self
        }
    }
}

/// The result of a runoff election.
enum RunoffTabulationResult {
    /// A candidate won the election.
    Win(Candidate),
    /// A candidate has been eliminated from the election. The election continues with another round.
    Elimination(Candidate),
    /// The result of the election is a tie.
    Tie
}

pub fn main() {
    // Reads candidates from command line args.
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Usage:\n ./runoff <candidate1> <candidate2> <...> <candidateN>\nMinimun number of candidates is 2");
    }

    // Creates a hashmap which allows candidate indexing by name.
    let mut candidates: HashMap<String, Candidate> = (&args[1..])/*(&args[1..])*/
        .iter()
        .enumerate()
        .map(|(i, candidate)| (candidate.to_lowercase(), Candidate::new(candidate.clone())))
        .collect();

    // Reads number of voters in the election.
    let number_of_voters: i32 = loop {
        match helpers::read_line("Number of voters: ").unwrap().parse::<i32>() {
            Ok(n) => break n,
            _ => eprintln!("The number of voters should be and integer")
        };
    };

    // Read votes.
    let mut votes = vote(number_of_voters, &mut candidates);

    // Tabulates results
    let result = loop {
        match tabulate(&mut votes, &mut candidates) {
            RunoffTabulationResult::Win(candidate) => break format!("Winner is {}", candidate.name),
            RunoffTabulationResult::Elimination(candidate) => {
                candidates.get_mut(&candidate.name.to_lowercase()).unwrap().eliminated = true;

                for (_, value) in candidates.iter_mut() {
                    value.votes = if value.eliminated { -1 } else { 0 };
                }
            },
            RunoffTabulationResult::Tie => break String::from("Tie!")
        }
    };

    println!("{}", result);
}

/// Votes the given number of times.
///
/// # Arguments
/// * `number_of_voters` - Number of voters in the election.
/// * `candidates` - The candidate table. Votes for candidates which are not in this table are not allowed.
fn vote(number_of_voters: i32, candidates: &mut HashMap<String, Candidate>) -> Vec<Vec<String>> {
    (0..number_of_voters).map(|_| {
        let mut voted: HashSet<String> = HashSet::new();

        let votes = (0..candidates.len()).fold(Vec::new(), |mut votes, i| {
            let vote = loop {
                let vote = helpers::read_line(&format!("Rank {}: ", i + 1)).unwrap().to_lowercase();
                
                match candidates.get(&vote) {
                    Some(index) => if voted.insert(vote.to_string()) {
                        break vote.to_string();
                    } else {
                        println!("You already voted for that candidate");
                    },
                    _ => println!("That candidate does not exist")
                }
            };

            votes.push(vote);
            votes
        });

        println!("");
        votes
    }).collect()
}

/// Tabulates the results of a runoff election round.
///
/// # Arguments
/// * `votes` - The election votes.
/// * `candidates` - The candidate table. Votes for candidates which are not in this table are not allowed.
fn tabulate(votes: &Vec<Vec<String>>, candidates: &mut HashMap<String, Candidate>) -> RunoffTabulationResult {
    let number_of_candidates: i32 = votes[0].len() as i32;

    for voter_votes in votes {
        let i = voter_votes.iter()
            .position(|vote| if let Some(candidate) = candidates.get(vote) {
                !candidate.eliminated
            } else {
                false
            })
            .unwrap();

        candidates.get_mut(&voter_votes[i]).unwrap().votes += 1;
    }

    let initial_min = Candidate {
        name: String::from(""),
        votes: i32::MAX,
        eliminated: true
    };

    let initial_max = Candidate {
        name: String::from(""),
        votes: i32::MIN,
        eliminated: true
    };

    let (min, max) = candidates.values()
        .fold((&initial_min, &initial_max), |(mut min, mut max), candidate| {
            if !candidate.eliminated {
                if candidate.votes < min.votes {
                    min = candidate;
                }
            
                if candidate.votes > max.votes {
                    max = candidate;
                }
            }

            (min, max)
        });
        
    if min.votes == max.votes {
        RunoffTabulationResult::Tie
    } else if max.votes as f64 >= votes.len() as f64 / 2.0 {
        RunoffTabulationResult::Win(max.clone())
    } else {
        RunoffTabulationResult::Elimination(min.clone())
    }
}