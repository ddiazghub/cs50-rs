use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Formatter};
use std::env;
use super::{helpers, sort};

enum TidemanError {
    CandidateNotFoundError(String),
    CandidateAlreadyExistsError(String),
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

#[derive(Clone)]
struct Candidate {
    pub name: String
}

impl Candidate {
    pub fn new(name: String) -> Self {
        Candidate {
            name
        }
    }
}

struct TidemanNode {
    pub candidate: Candidate,
    pub links: Vec<usize>
}

impl TidemanNode {
    pub fn new(candidate: Candidate) -> Self {
        TidemanNode {
            candidate,
            links: Vec::new()
        }
    }

    pub fn link(&mut self, node_id: usize) {
        self.links.push(node_id);
    }
}

#[derive(Debug, Clone)]
struct TidemanPair {
    pub winner_id: usize,
    pub loser_id: usize,
    pub weight: i32
}

impl TidemanPair {
    pub fn new(winner_id: usize, loser_id: usize, weight: i32) -> Self {
        TidemanPair {
            winner_id,
            loser_id,
            weight
        }
    }
}

struct TidemanGraph {
    nodes: Vec<TidemanNode>,
    names_ids_map: HashMap<String, usize>,
    votes: Vec<Vec<usize>>,
    pairs: Vec<TidemanPair>
}

impl TidemanGraph {
    pub fn new() -> Self {
        TidemanGraph {
            nodes: Vec::new(),
            names_ids_map: HashMap::new(),
            votes: Vec::new(),
            pairs: Vec::new()
        }
    }

    pub fn get_candidate_id(&self, candidate: &str) -> Result<usize, TidemanError> {
        if self.contains(candidate) {
            Ok(self.names_ids_map[candidate])
        } else {
            Err(TidemanError::CandidateNotFoundError(candidate.to_string()))
        }
    }

    pub fn contains(&self, candidate: &str) -> bool {
        self.names_ids_map.contains_key(candidate)
    }

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

    pub fn has_cycles_from(&self, node_id: usize) -> bool {
        let mut visited: HashSet<usize> = HashSet::new();

        return self.has_cycles_dfs(node_id, &mut visited);
    }

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

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn vote(&mut self, n: i32) {
        for i in 0..n {
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

    pub fn lock_pairs(&mut self) {
        for i in 0..self.pairs.len() {
            match self.lock(self.pairs[i].winner_id, self.pairs[i].loser_id) {
                Ok(_) => (),
                Err(_) => ()
            };
        }
    }

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
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Usage:\n ./tideman <candidate1> <candidate2> <...> <candidateN>\nMinimun number of candidates is 2");
    }

    let mut graph: TidemanGraph = (&args[1..])/*(&args[1..])*/
        .into_iter()
        .fold(TidemanGraph::new(), |mut graph, candidate| {
            if let Err(err) = graph.add_candidate(candidate.to_string()) {
                panic!("{:?}", err);
            }

            graph
        });

    let number_of_voters: i32 = loop {
        match helpers::read_line("Number of voters: ").unwrap().parse::<i32>() {
            Ok(n) => break n,
            _ => eprintln!("The number of voters should be and integer")
        };
    };

    graph.vote(number_of_voters);
    graph.tabulate();
    graph.lock_pairs();
    println!("The winner is {}", graph.get_winner().name);
}