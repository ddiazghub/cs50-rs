use std::io::{self, Write};

pub fn read_line(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    let mut name = String::new();
    
    if let Err(err) = io::stdout().flush() {
        eprintln!("Error: {}", err);
    };

    io::stdin().read_line(&mut name)?;

    Ok(String::from(name.trim()))
}

pub fn slice2(slice: &[u8]) -> [u8; 2] {
    slice.try_into().expect("The slice should have a length of 2")
}

pub fn slice4(slice: &[u8]) -> [u8; 4] {
    slice.try_into().expect("The slice should have a length of 4")
}