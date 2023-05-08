use std::fmt::Display;
use std::env;
use rand::{self, seq::SliceRandom};

/// Allowed alleles
static ALLELES: [char; 3] = ['A', 'B', 'O'];

/// A person with parents and 2 alleles.
pub struct Person {
    /// The person's parents. A person may not have parents.
    parents: Option<Box<(Person, Person)>>,
    /// The person's alleles.
    alleles: [char; 2]
}

impl Person {
    /// Creates a new person with no parents and random alleles.
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            parents: None,
            alleles: [*ALLELES.choose(&mut rng).unwrap(), *ALLELES.choose(&mut rng).unwrap()]
        }
    }

    /// Creates a person with the given parents. Allels are randomly chosen from parents.
    ///
    /// # Arguments
    /// * `parents` - The person's parents.
    pub fn with_parents(parents: (Person, Person)) -> Self {
        let mut rng = rand::thread_rng();
        let alleles = [*parents.0.alleles.choose(&mut rng).unwrap(), *parents.1.alleles.choose(&mut rng).unwrap()];

        Self {
            parents: Some(Box::new(parents)),
            alleles
        }
    }

    /// Creates a family tree by recursively creating generations.
    ///
    /// # Arguments
    /// * `generations` - The number of generations in the family.
    pub fn create_family(generations: usize) -> Self {
        Self::recurse_family(generations)
    }

    /// Creates a family tree by recursively creating generations.
    ///
    /// # Arguments
    /// * `generations` - The number of generations left to create.
    fn recurse_family(gens_left: usize) -> Self {
        match gens_left {
            1 => Self::new(),
            _ => {
                let parents = (Self::recurse_family(gens_left - 1), Self::recurse_family(gens_left - 1));
                Self::with_parents(parents)
            }
        }
    }

    /// Formats the person's family tree as a string.
    ///
    /// # Arguments
    /// * `generation` - The current generation's number.
    fn as_string(&self, generation: usize) -> String {
        let string = "\t".repeat(generation) + "(Generation " + &generation.to_string() + "): Blood type " + &self.alleles.into_iter().collect::<String>();

        match self.parents {
            Some(ref parents) => string + "\n" + &parents.0.as_string(generation + 1) + "\n" + &parents.1.as_string(generation + 1),
            None => string
        }
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string(0))
    }
}

pub fn main() {
    // Reads the family tree's height from command line args.
    let height: usize = env::args().nth(1)
        .unwrap_or("3".to_string())
        .parse()
        .unwrap();

    // Creates and prints the family tree.
    let family_tree = Person::create_family(height);
    println!("{family_tree}");
}