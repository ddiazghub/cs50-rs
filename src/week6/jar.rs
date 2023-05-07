use std::fmt::{Display, Formatter};
use std::io;
use std::io::Write;

const HELP: &str = r"
Cookie jar.

Commands:
deposit <n> -- Deposit n cookies in the cookie jar.
withdraw <n> -- Withdraw n cookies from cookie jar.
size -- Number of cookies in jar.
print -- Print cookie jar in console.
help -- Show this text.
exit -- Exit program.
";

enum JarError {
    Overflow,
    Underflow
}

struct CookieJar {
    capacity: u32,
    cookies: u32
}

impl CookieJar {
    pub fn new(capacity: u32) -> Self {
        Self {
            capacity,
            cookies: 0
        }
    }

    pub fn deposit(&mut self, cookies: u32) -> Result<(), JarError> {
        if self.cookies + cookies > self.capacity {
            Err(JarError::Overflow)
        } else {
            self.cookies = self.cookies + cookies;
            Ok(())
        }
    }

    pub fn withdraw(&mut self, cookies: u32) -> Result<(), JarError>{
        if cookies > self.cookies {
            Err(JarError::Underflow)
        } else {
            self.cookies -= cookies;
            Ok(())
        }
    }

    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    pub fn size(&self) -> u32 {
        self.cookies
    }
}

impl Display for CookieJar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "🍪".repeat(self.cookies as usize))
    }
}

pub fn main() {
    let mut input = String::new();
    print!("Input the cookie jar's capacity: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let mut jar = CookieJar::new(input.trim_end().parse().unwrap());
    println!("{HELP}");

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim_end().split_whitespace();

        match (parts.next(), parts.next(), parts.next()) {
            (Some(command), Some(value), None) => match command {
                "deposit" => match jar.deposit(value.parse().unwrap()) {
                    Ok(_) => println!("Added {value} cookies to the jar."),
                    _ => println!("Number of cookies in jar exceeds capacity.")
                },
                "withdraw" => match jar.withdraw(value.parse().unwrap()) {
                    Ok(_) => println!("Withdrew {value} cookies from the jar."),
                    _ => println!("Amount of cookies in jar is less than the withdrawn amount.")
                },
                _ => println!("Unknown command.")
            },
            (Some(command), None, None) => match command {
                "size" => println!("The jar contains {} cookies.", jar.size()),
                "print" => println!("{}", jar.to_string()),
                "help" => println!("{HELP}"),
                "exit" => break,
                _ => println!("Unknown command.")
            },
            _ => println!("Invalid input.")
        }
    }
}