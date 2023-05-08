use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Formatter};
use std::env;
use super::{helpers, sort};

/// Errors which may happen in a tideman election.
enum TidemanError {
    /// The given candidate does not exist.
    CandidateNotFoundError(String),
    /// Attempted to register an existing candidate.
    CandidateAlreadyExistsError(String),
    /// A graph lock created a cycle.
    LockCreatedCycleError
}

impl Debug for TidemanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let text = match self {
            TidemanError::CandidateNotFoundError(name) => format!("The candidate  \"{}\" was not found", name),
            TidemanError::CandidateAlreadyExistsError(name) => format!("Can't add candidate \"{}\" because it already exists", name),
            TidemanError::LockCreatedCycleError => String::from("The lock created a cycle in the graph")
        };

        write!(f, "{}", text)
    }
}

impl Clone for TidemanError {
    fn clone(&self) -> Self {
        match self {
            TidemanError::CandidateAlreadyExistsError(name) => TidemanError::CandidateAlreadyExistsError(name.clone()),
            TidemanError::CandidateNotFoundError(name) => TidemanError::CandidateNotFoundError(name.clone()),
            TidemanError::LockCreatedCycleError => TidemanError::LockCreatedCycleError
        }
    }
}

/// A candidate participating in a tideman election.
#[derive(Clone)]
struct Candidate {
    /// The candidate's name
    pub name: String
}

impl Candidate {
    /// Creates a new candidate with the given name.
    ///
    /// # Arguments
    /// * `name` - The candidate's name.
    pub fn new(name: String) -> Self {
        Candidate {
            name
        }
    }
}

/// A node in a tideman graph.
struct TidemanNode {
    /// The node's candidate.
    pub candidate: Candidate,
    /// The node's edges.
    pub links: Vec<usize>
}

impl TidemanNode {
    /// Creates a new tideman node containing the given candidate.
    ///
    /// # Arguments
    /// * `candidate` - The node's candidate.
    pub fn new(candidate: Candidate) -> Self {
        TidemanNode {
            candidate,
            links: Vec::new()
        }
    }

    /// Adds an edge from the this node to the specified node.
    ///
    /// # Arguments
    /// * `node_id` - The other node's id.
    pub fn link(&mut self, node_id: usize) {
        self.links.push(node_id);
    }
}

/// A pair of candidates facing each other in a tideman election.
#[derive(Debug, Clone)]
struct TidemanPair {
    /// The index of the winning candidate.
    pub winner_id: usize,
    /// The index of the losing candidate.
    pub loser_id: usize,
    /// Weight of the matchup or how impactful it is for the overall election.
    pub weight: i32
}

impl TidemanPair {
    /// Creates a new tideman pair with the supplied data.
    ///
    /// # Arguments
    /// * `winner_id` - The index of the winning candidate.
    /// * `loser_id` - The index of the losing candidate.
    /// * `weight` - Weight of the matchup or how impactful it is for the overall election.
    pub fn new(winner_id: usize, loser_id: usize, weight: i32) -> Self {
        TidemanPair {
            winner_id,
            loser_id,
            weight
        }
    }
}

/// A graph used to calculate the result of a tideman election.
struct TidemanGraph {
    /// The graph's nodes.
    nodes: Vec<TidemanNode>,
    /// A hashmap which allows indexing by candidate name.
    names_ids_map: HashMap<String, usize>,
    /// Number of votes for each candidate.
    votes: Vec<Vec<usize>>,
    /// Pairs of candidates facing each other in a tideman election.
    pairs: Vec<TidemanPair>
}

impl TidemanGraph {
    /// Creates a new empty tideman graph.
    pub fn new() -> Self {
        TidemanGraph {
            nodes: Vec::new(),
            names_ids_map: HashMap::new(),
            votes: Vec::new(),
            pairs: Vec::new()
        }
    }

    /// Gets a candidate's id by name.
    ///
    /// # Arguments
    /// * `candidate` - The candidate's name.
    pub fn get_candidate_id(&self, candidate: &str) -> Result<usize, TidemanError> {
        if self.contains(candidate) {
            Ok(self.names_ids_map[candidate])
        } else {
            Err(TidemanError::CandidateNotFoundError(candidate.to_string()))
        }
    }

    /// Checks if a candidate exists.
    ///
    /// # Arguments
    /// * `candidate` - The candidate's name.
    pub fn contains(&self, candidate: &str) -> bool {
        self.names_ids_map.contains_key(candidate)
    }

    /// Adds a candidate to the election.
    ///
    /// # Arguments
    /// * `name` - The candidate's name.
    pub fn add_candidate(&mut self, name: String) -> Result<(), TidemanError> {
        let node = TidemanNode::new(Candidate::new(name.clone()));
        
        match self.names_ids_map.insert(name.to_lowercase(), self.nodes.len()) {
            Some(_) => Err(TidemanError::CandidateAlreadyExistsError(name)),
            _ => {
                self.nodes.push(node);
                Ok(())
            }
        }
    }

    /// Checks if the graph has any cycle starting from the specified node.
    ///
    /// # Arguments
    /// * `node_id` - The node's index.
    pub fn has_cycles_from(&self, node_id: usize) -> bool {
        let mut visited: HashSet<usize> = HashSet::new();

        return self.has_cycles_dfs(node_id, &mut visited);
    }

    /// Checks if the graph has cycles using the DFS algorithm to traverse the graph.
    ///
    /// # Arguments
    /// * `node_id` - The node's index.
    fn has_cycles_dfs(&self, node_id: usize, visited: &mut HashSet<usize>) -> bool {
        if !visited.insert(node_id) {
            return true;
        };

        for link_id in self.nodes[node_id].links.iter() {
            if self.has_cycles_dfs(*link_id, visited) {
                return true;
            }
        }

        return false;
    }

    /// Locks a the tideman pair having the specified winner and loser candidates if the lock does not create a cycle.
    ///
    /// # Arguments
    /// * `winner_id` - The pair's winner's index.
    /// * `loser_id` - The pair's loser's index.
    fn lock(&mut self, winner_id: usize, loser_id: usize) -> Result<(), TidemanError> {
        let candidates: Vec<Result<usize, TidemanError>> = [winner_id, loser_id]
            .iter()
            .map(|candidate_id| {
                match self.nodes.get(*candidate_id) {
                    Some(candidate) => Ok(*candidate_id),
                    _ => Err(TidemanError::CandidateNotFoundError(candidate_id.to_string()))
                }
            })
            .collect();

        let ids = (candidates[0].clone()?, candidates[1].clone()?);
        self.nodes[ids.0].link(ids.1);

        if self.has_cycles_from(ids.0) {
            self.nodes[ids.0].links.pop();
            Err(TidemanError::LockCreatedCycleError)
        } else {
            Ok(())
        }
    }

    /// Number of candidates in the graph.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Votes the specified number of times.
    ///
    /// # Arguments
    /// * `voters` - Number of voters in the election. 1 vote for each voter.
    pub fn vote(&mut self, voters: i32) {
        for i in 0..voters {
            let mut voted: HashSet<usize> = HashSet::new();

            let voter_votes: Vec<usize> = (0..self.len())
                .fold(Vec::new(), |mut voter_votes, i| {
                    let vote = loop {
                        let vote = helpers::read_line(&format!("Rank {}: ", i + 1)).unwrap().to_lowercase();
                        
                        match self.names_ids_map.get(&vote) {
                            Some(&index) => if voted.insert(index) {
                                break index;
                            } else {
                                println!("You already voted for that candidate");
                            },
                            _ => println!("That candidate does not exist")
                        }
                    };
        
                    voter_votes.push(vote);
                    voter_votes
                });

            self.votes.push(voter_votes);
        };
    }

    /// Tabulates the election's results.
    pub fn tabulate(&mut self) {
        let mut pairs: Vec<Vec<i32>> = self.nodes
            .iter()
            .map(|_| self.nodes
                .iter()
                .map(|_| 0)
                .collect()
            )
            .collect();

        let number_of_candidates = self.nodes.len();

        for v in self.votes.iter() {
            for i in 0..(number_of_candidates - 1) {
                for j in (i + 1)..number_of_candidates {
                    pairs[v[i]][v[j]] += 1;
                    pairs[v[j]][v[i]] -= 1;
                }
            }
        }

        for i in 1..number_of_candidates {
            for j in 0..i {
                let pair = if pairs[i][j] < 0 {
                    TidemanPair::new(j, i, -pairs[i][j])
                } else {
                    TidemanPair::new(i, j, pairs[i][j])
                };

                self.pairs.push(pair);
            }
        }

        sort::quicksort_by(&mut self.pairs[..], &|bigger, smaller| smaller.weight < bigger.weight);
    }

    /// Locks tideman pairs in the election depending on their weight in order to find a winner.
    pub fn lock_pairs(&mut self) {
        for i in 0..self.pairs.len() {
            match self.lock(self.pairs[i].winner_id, self.pairs[i].loser_id) {
                Ok(_) => (),
                Err(_) => ()
            };
        }
    }

    /// Calculates the election's winner.
    pub fn get_winner(&self) -> Candidate {
        let mut possible_winners: HashSet<usize> = (0..self.len()).collect();

        for candidate in self.nodes.iter() {
            for win in candidate.links.iter() {
                possible_winners.remove(win);
            }
        }

        match possible_winners.into_iter().find(|p| self.nodes[*p].links.len() > 0) {
            Some(w) => self.nodes[w].candidate.clone(),
            _ => panic!("Could not compute winner")
        }
    }
}

pub fn main() {
    // Reads candidates from command line args.
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Usage:\n ./tideman <candidate1> <candidate2> <...> <candidateN>\nMinimun number of candidates is 2");
    }

    // Creates a tideman graph from candidates.
    let mut graph: TidemanGraph = (&args[1..])/*(&args[1..])*/
        .into_iter()
        .fold(TidemanGraph::new(), |mut graph, candidate| {
            if let Err(err) = graph.add_candidate(candidate.to_string()) {
                panic!("{:?}", err);
            }

            graph
        });

    // Reads number of voters.
    let number_of_voters: i32 = loop {
        match helpers::read_line("Number of voters: ").unwrap().parse::<i32>() {
            Ok(n) => break n,
            _ => eprintln!("The number of voters should be and integer")
        };
    };

    // Votes, tabulates results and finds winner.
    graph.vote(number_of_voters);
    graph.tabulate();
    graph.lock_pairs();
    println!("The winner is {}", graph.get_winner().name);
}