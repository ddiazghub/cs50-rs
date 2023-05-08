use std::cmp::Reverse;
use std::env;
use std::fs::File;
use csv::ReaderBuilder;
use itertools::Itertools;
use num_traits::Pow;
use serde::Deserialize;
use serde;
use rand;
use rand::Rng;

/// Number of tournament simulations to do.
const SIMULATIONS: u32 = 100000;

/// A team playing in the World Cup.
#[derive(Deserialize, Debug)]
struct Team {
    /// Name of the team.
    #[serde(rename = "team")]
    name: String,
    /// The team's skill rating.
    rating: u32
}

impl Team {
    /// Simulates a match with another Team. Returns true if this team is the winner.
    ///
    /// # Arguments
    /// * `other` - The other team being faced in the match.
    pub fn game<'a>(&'a self, other: &'a Self) -> bool {
        let probability = 1.0 / (1.0 + 10_f64.pow((other.rating as f64 - self.rating as f64) / 600.0));
        let random = rand::thread_rng().gen::<f64>();

        random < probability
    }
}

/// A world cup tournament.
struct Tournament {
    /// The list of teams participating in the tournament.
    teams: Vec<Team>
}

impl Tournament {
    /// Simulates a single tournament. Returns the index of the winner.
    fn simulate_one(&self) -> usize {
        let mut teams: Vec<_> = self.teams.iter()
            .enumerate()
            .collect();

        while teams.len() > 1 {
            teams = Self::simulate_round(teams);
        }

        teams[0].0
    }

    /// Simulates the current tournament a specific number of times.
    /// Returns a Vec containing each team and the number of simulations where that team won.
    ///
    /// # Arguments
    /// * `times` - Number of times to simulate the tournament.
    pub fn simulate(&self, times: u32) -> Vec<(&Team, u32)> {
        let mut wins = vec![0_u32; self.teams.len()];

        for i in 0..times {
            let winner = self.simulate_one();
            wins[winner] += 1;
        }

        let mut teams: Vec<_> = self.teams.iter()
            .zip(wins.into_iter())
            .collect();

        teams.sort_unstable_by_key(|(_, wins)| Reverse(*wins));

        teams
    }

    /// Simulates a single round of a tournament.
    /// Returns a Vec containing the teams that pass to the next round.
    ///
    /// # Arguments
    /// * `teams` - A vector containing each team in the current round and the team's index or ID.
    fn simulate_round(teams: Vec<(usize, &Team)>) -> Vec<(usize, &Team)> {
        teams.into_iter()
            .chunks(2)
            .into_iter()
            .map(|chunk| {
                let teams: Vec<_> = chunk.collect();

                if teams.len() == 1 || teams[0].1.game(teams[1].1) {
                    teams[0]
                } else {
                    teams[1]
                }
            })
            .collect()
    }
}

impl FromIterator<Team> for Tournament {
    fn from_iter<T: IntoIterator<Item=Team>>(iter: T) -> Self {
        let teams: Vec<_> = iter.into_iter().collect();

        match teams.len() {
            0 => panic!("Empty tournament."),
            len if len % 2 == 0 => Self { teams },
            _ => panic!("Tournament must have an even number of teams."),
        }
    }
}

pub fn main() {
    // Opens and reads CSV file.
    let csv_filename = env::args().nth(1).expect("Missing CSV file parameter.");
    let csv_file = File::open(csv_filename).expect("Could not open CSV file.");
    let mut reader = ReaderBuilder::new().from_reader(csv_file);

    // Deserializes the csv into a tournament and simulates 1000 tournaments.
    let teams = reader.deserialize().collect::<Result<Tournament, _>>().expect("Malformed CSV.");
    let team_wins = teams.simulate(SIMULATIONS);

    let total_matches: u32 = team_wins.iter()
        .map(|(_, wins)| *wins)
        .sum();

    // Prints each team's probability to win a tournament in percent.
    for (team, wins) in team_wins {
        let percent = wins as f64 * 100.0 / total_matches as f64;
        println!("{}: {:.1}% chance of winning", team.name, percent);
    }
}