use std::{env, fs};
use std::fmt::Display;
use std::io::{BufRead, BufReader, Read};
use std::fs::File;
use regex::Regex;

/// A custom singly linked list node.
#[derive(Clone)]
struct ListNode<T> {
    /// The node's data.
    data: T,
    /// Next node in the list.
    next: Option<Box<ListNode<T>>>
}

impl <T> ListNode<T> {
    /// Creates a new linked list node containing the supplied data.
    ///
    /// # Arguments
    /// * `data` - The node's data.
    pub fn new(data: T) -> Self {
        Self {
            data,
            next: None
        }
    }

    /// Adds a new item to the end of the list.
    ///
    /// # Arguments
    /// * `data` - The data to add.
    pub fn add(&mut self, data: T) {
        match self.next.as_mut() {
            Some(next) => next.add(data),
            None => self.next = Some(Box::new(ListNode::new(data)))
        }
    }
}

/// An iterator for a linked list.
struct ListIter<'a, T>(Option<&'a ListNode<T>>);

impl <'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            None => None,
            Some(node) => {
                let data = &node.data;
                self.0 = node.next.as_ref().map(|next| next.as_ref());
                Some(data)
            }
        }
    }
}

/// A custom singly linked list.
#[derive(Clone)]
struct List<T> {
    /// The first node in the list.
    head: Option<ListNode<T>>
}

impl <T> List<T> {
    /// Creates a new empty linked list.
    pub fn new() -> Self {
        Self {
            head: None
        }
    }

    /// Adds a new item to the end of the list.
    ///
    /// # Arguments
    /// * `data` - The data to add.
    pub fn add(&mut self, data: T) {
        match self.head.as_mut() {
            Some(head) => head.add(data),
            None => self.head = Some(ListNode::new(data))
        }
    }
}

impl <'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = ListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIter(self.head.as_ref())
    }
}

/// A custom hash table for quick lookup of unique data.
struct HashTable<T> {
    /// Vec which contains the actual table with data.
    table: Vec<List<T>>,
    /// Size of the table in cells.
    capacity: usize,
    /// Number of items in the table.
    len: usize
}

impl <T: Clone> HashTable<T> {
    /// Default capacity.
    const BASE_CAPACITY: usize = 19;

    /// Creates a new hashtable with the supplied capacity.
    ///
    /// # Arguments
    /// * `capacity` - The table's capacity.
    pub fn with_capacity(mut capacity: usize) -> Self {
        capacity = Self::next_capacity(capacity);

        Self {
            table: vec![List::new(); capacity],
            capacity,
            len: 0
        }
    }

    /// Creates a new hashtable with the default capacity.
    pub fn new() -> Self {
        Self::with_capacity(Self::BASE_CAPACITY)
    }

    /// Computes the next prime number at least twice as big as the table's current capacity.
    ///
    /// # Arguments
    /// * `current` - The table's current capacity.
    fn next_capacity(current: usize) -> usize {
        for i in current.. {
            if Self::is_prime(i) {
                return i
            }
        }

        return 0
    }

    /// Finds if a number is prime.
    ///
    /// # Arguments
    /// * `n` - The number.
    fn is_prime(n: usize) -> bool {
        (2..n / 2).all(|i| n % i > 0)
    }
}

impl HashTable<String> {
    /// Adds an item to the hashtable.
    ///
    /// # Arguments
    /// * `item` - The item to add.
    pub fn add(&mut self, item: String) {
        self.len += 1;
        let hash = self.hash(&item);
        self.table[hash].add(item);
    }

    /// Checks if an item is in the hashtable.
    ///
    /// # Arguments
    /// * `item` - The item.
    pub fn contains(&self, item: &str) -> bool {
        let hash = self.hash(&item);

        (&self.table[hash])
            .into_iter()
            .any(|list_item| list_item == item)
    }

    /// Computes an item's hash value.
    ///
    /// # Arguments
    /// * `item` - The item.
    fn hash(&self, item: &str) -> usize {
        let len = item.len();

        let value: usize = item.as_bytes()
            .into_iter()
            .enumerate()
            .map(|(i, &byte)| byte as usize * Self::BASE_CAPACITY * (len - i - 1))
            .sum();

        value % self.capacity
    }
}

/// Loads a dictionary file into a hashtable.
///
/// # Arguments
/// * `filename` - The dictionary's filename.
fn load_dict(filename: &str) -> HashTable<String> {
    let dict_file = BufReader::new(File::open(filename).unwrap());
    let words: Vec<_> = dict_file.lines().collect::<Result<Vec<_>, _>>().unwrap();
    let mut dictionary = HashTable::with_capacity(words.len());

    for word in words.into_iter() {
        dictionary.add(word);
    }

    dictionary
}

/// Spell checks a text file in order to find misspelled words.
///
/// # Arguments
/// * `filename` - The text file's name.
/// * `dictionary` - The dictionary to use as reference to find words.
/// * `split_regex` - Regex used to split words in the text.
fn check(filename: &str, dictionary: &HashTable<String>, split_regex: &Regex) -> (u32, u32) {
    let file = BufReader::new(File::open(filename).unwrap());
    let mut words = 0;
    let mut misspelled = 0;

    for line in file.lines() {
        for word in split_regex.split(&line.unwrap().to_lowercase()) {
            if !word.is_empty() {
                if !dictionary.contains(word) {
                    println!("{word}");
                    misspelled += 1;
                }

                words += 1;
            }
        }
    }

    (words, misspelled)
}

pub fn main() {
    // Reads filenames from command line args.
    let split_regex = Regex::new("[^a-zA-Z']+").unwrap();
    let mut args = env::args().skip(1);
    let dict_filename = args.next().unwrap();
    let filename = args.next().unwrap();

    // Loads the dictionary.
    let dictionary = load_dict(&dict_filename);

    // Spell checks text file.
    println!("MISSPELLED WORDS");
    let (words, misspelled) = check(&filename, &dictionary, &split_regex);

    println!("WORDS MISSPELLED:     {}", misspelled);
    println!("WORDS IN DICTIONARY:  {}", dictionary.len);
    println!("WORDS IN TEXT:        {}", words);
}