use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
pub type ForthAction = fn(&mut Forth) -> Result;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

pub enum WordAction {
    Predefined(ForthAction),
    Custom(String)
}

impl Clone for WordAction {
    fn clone(&self) -> Self {
        match self {
            Self::Predefined(f) => Self::Predefined(f.clone()),
            Self::Custom(s) => Self::Custom(s.clone())
        }
    }
}

#[derive(Clone)]
pub struct Forth {
    stack: Vec<Value>,
    dictionary: HashMap<String, WordAction>
}

impl Forth {
    pub fn new() -> Forth {
        let dictionary = HashMap::from_iter([
            ("+".to_string(), WordAction::Predefined(Self::add)),
            ("-".to_string(), WordAction::Predefined(Self::sub)),
            ("*".to_string(), WordAction::Predefined(Self::mul)),
            ("/".to_string(), WordAction::Predefined(Self::div)),
            ("dup".to_string(), WordAction::Predefined(Self::dup)),
            ("drop".to_string(), WordAction::Predefined(Self::drop)),
            ("swap".to_string(), WordAction::Predefined(Self::swap)),
            ("over".to_string(), WordAction::Predefined(Self::over)),
        ]);

        Self {
            stack: Vec::new(),
            dictionary
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack[..]
    }

    pub fn eval(&mut self, mut input: &str) -> Result {
        if input.len() == 0 {
            return Ok(());
        }

        let mut skip: usize = 0;
        let input = input.trim().to_lowercase();
        let mut i = 0;
        let split: Vec<&str> = input.split(' ').collect();

        for word in split.iter() {
            let word = *word;

            if i >= skip as usize {
                match word.parse::<Value>() {
                    Ok(n) => self.push(n),
                    _ => drop (
                        match word {
                            ":" => {
                                skip = i + self.define_word(&split[i..])? + 1;
                            },
                            _ => {
                                drop(
                                    match self.dictionary.get(word) {
                                        Some(option) => {
                                            let o = option.clone();

                                            match o {
                                                WordAction::Predefined(f) => f(self)?,
                                                WordAction::Custom(words) => self.eval(&words)?
                                            }
                                        },
                                        None => return Err(Error::UnknownWord)
                                    }
                                )
                            }
                        }
                    )
                }
            }

            i += 1;
        }

        Ok(())
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn define_word(&mut self, definition: &[&str]) -> std::result::Result<usize, Error> {
        if definition.len() < 4 {
            return Err(Error::InvalidWord);
        }

        let split = &definition[1..];
        let name = split[0];
        let mut ended = false;

        if let Ok(_) = name.parse::<i32>() {
            return Err(Error::InvalidWord);
        }

        let mut definition = String::new();
        let mut i = 2;

        for word in split[1..].iter() {
            if *word == ";" {
                ended = true;
                break;
            }

            definition.push_str(" ");
            definition.push_str(
                if let Ok(_) = word.parse::<Value>() {
                    word
                } else {
                    match self.dictionary.get(*word) {
                        Some(WordAction::Custom(def)) => def,
                        Some(WordAction::Predefined(def)) => word,
                        _ => return Err(Error::UnknownWord)
                    }
                }
            );

            i += 1;
        }

        definition = definition.trim().to_string();

        if ended {
            let mut copy = self.clone();
            
            match copy.eval(&definition)?;

            if copy.stack == self.stack {
                definition = String::from("");
            }

            self.dictionary.insert(name.to_string(), WordAction::Custom(definition));
            Ok(i)
        } else {
            Err(Error::InvalidWord)
        }
    }

    pub fn operate<F2: Fn(Value, Value) -> Value>(&mut self, operation: F2) -> Result {
        let (n1, n2) = match (self.stack.pop(), self.stack.pop()) {
            (Some(n2), Some(n1)) => (n1, n2),
            _ => return Err(Error::StackUnderflow)
        };

        self.push(operation(n1, n2));
        Ok(())
    }

    pub fn add(&mut self) -> Result {
        self.operate(Value::add)
    }

    pub fn sub(&mut self) -> Result {
        self.operate(Value::sub)
    }

    pub fn mul(&mut self) -> Result {
        self.operate(Value::mul)
    }

    pub fn div(&mut self) -> Result {
        match self.stack().last() {
            Some(0) => Err(Error::DivisionByZero),
            _ => self.operate(Value::div)
        }
    }

    pub fn dup(&mut self) -> Result {
        if let Some(&n) = self.stack.last() {
            self.push(n);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    pub fn drop(&mut self) -> Result {
        if let Some(_) = self.stack.pop() {
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    pub fn swap(&mut self) -> Result {
        let values = match (self.stack.pop(), self.stack.pop()) {
            (Some(n1), Some(n2)) => [n1, n2],
            _ => return Err(Error::StackUnderflow)
        };

        self.stack.extend_from_slice(&values);
        Ok(())
    }

    pub fn over(&mut self) -> Result {
        if self.stack.len() < 2 {
            Err(Error::StackUnderflow)
        } else if let Some(&n) = self.stack.get(self.stack.len() - 2) {
            self.push(n);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }
}



pub fn main() {
    let mut f = Forth::new();
    f.eval(": dup-twice dup dup ;");
    f.eval("1 dup-twice");
    
    println!("{:?}", f.stack());
    
}
