use std::fmt::Display;

use rand::{self, Rng};

static ALLELES: [char; 3] = ['A', 'B', 'O'];

pub trait RandChoice<T> {
    fn rand_choice(&self) -> &T;

    fn rand_choices(&self, n: u32) -> Vec<&T> {
        (0..n)
            .map(|_| self.rand_choice())
            .collect()
    }
}

impl<T> RandChoice<T> for [T] {
    fn rand_choice(&self) -> &T {
        let index = rand::thread_rng().gen_range(0..self.len());
        &self[index]
    }
}

pub struct Person {
    parents: Option<Box<(Person, Person)>>,
    alleles: [char; 2]
}

impl Person {
    pub fn new() -> Self {
        let alleles = ALLELES.rand_choices(2);

        Self {
            parents: None,
            alleles: [*alleles[0], *alleles[1]]
        }
    }

    pub fn with_parents(parents: (Person, Person)) -> Self {
        let alleles = [*parents.0.alleles.rand_choice(), *parents.1.alleles.rand_choice()];

        Self {
            parents: Some(Box::new(parents)),
            alleles
        }
    }

    pub fn create_family(generations: usize) -> Self {
        Self::recurse_family(generations)
    }

    fn recurse_family(gens_left: usize) -> Self {
        match gens_left {
            1 => Self::new(),
            _ => {
                let parents = (Self::recurse_family(gens_left - 1), Self::recurse_family(gens_left - 1));
                Self::with_parents(parents)
            }
        }
    }

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
    let family_tree = Person::create_family(3);
    println!("{family_tree}");
}