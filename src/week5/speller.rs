use std::{env, fs};
use std::fmt::Display;
use std::io::{BufRead, BufReader, Read};
use std::fs::File;
use regex::Regex;

#[derive(Clone)]
struct ListNode<T> {
    data: T,
    next: Option<Box<ListNode<T>>>
}

impl <T> ListNode<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            next: None
        }
    }

    pub fn add(&mut self, data: T) {
        match self.next.as_mut() {
            Some(next) => next.add(data),
            None => self.next = Some(Box::new(ListNode::new(data)))
        }
    }
}

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

#[derive(Clone)]
struct List<T> {
    head: Option<ListNode<T>>
}

impl <T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None
        }
    }

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

struct HashTable<T> {
    table: Vec<List<T>>,
    capacity: usize,
    len: usize
}

impl <T: Clone> HashTable<T> {
    const BASE_CAPACITY: usize = 19;

    pub fn with_capacity(mut capacity: usize) -> Self {
        capacity = Self::next_capacity(capacity);

        Self {
            table: vec![List::new(); capacity],
            capacity,
            len: 0
        }
    }

    pub fn new() -> Self {
        Self::with_capacity(Self::BASE_CAPACITY)
    }

    fn next_capacity(current: usize) -> usize {
        for i in current.. {
            if Self::is_prime(i) {
                return i
            }
        }

        return 0
    }

    fn is_prime(n: usize) -> bool {
        (2..n / 2).all(|i| n % i > 0)
    }
}

impl HashTable<String> {
    pub fn add(&mut self, item: String) {
        self.len += 1;
        let hash = self.hash(&item);
        self.table[hash].add(item);
    }

    pub fn contains(&self, item: &str) -> bool {
        let hash = self.hash(&item);

        (&self.table[hash])
            .into_iter()
            .any(|list_item| list_item == item)
    }

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

fn load_dict(filename: &str) -> HashTable<String> {
    let dict_file = BufReader::new(File::open(filename).unwrap());
    let words: Vec<_> = dict_file.lines().collect::<Result<Vec<_>, _>>().unwrap();
    let mut dictionary = HashTable::with_capacity(words.len());

    for word in words.into_iter() {
        dictionary.add(word);
    }

    dictionary
}

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
    let split_regex = Regex::new("[^a-zA-Z']+").unwrap();
    let mut args = env::args();

    args.next();

    let dict_filename = args.next().unwrap();
    let filename = args.next().unwrap();
    let dictionary = load_dict(&dict_filename);

    println!("MISSPELLED WORDS");

    let (words, misspelled) = check(&filename, &dictionary, &split_regex);

    println!("WORDS MISSPELLED:     {}", misspelled);
    println!("WORDS IN DICTIONARY:  {}", dictionary.len);
    println!("WORDS IN TEXT:        {}", words);
}