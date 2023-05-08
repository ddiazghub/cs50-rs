use std::fmt::{Display, Formatter};
use std::io;
use std::io::Write;

/// Help prompt which shows how to use the program.
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

/// An error caused by trying to modify the jar's state into an invalid state.
enum JarError {
    /// Caused by trying to deposit more cookies than the jar is capable of holding.
    Overflow,
    /// Caused by trying to withdraw more cookies than the jar currently has.
    Underflow
}

/// A jar used to hold cookies.
struct CookieJar {
    /// The max number of cookies that the jar can hold.
    capacity: u32,
    /// Number of cookies that the jar currently holds.
    cookies: u32
}

impl CookieJar {
    /// Creates a new cookie jar with the given capacity.
    ///
    /// # Arguments
    /// * `capacity` - The max number of cookies that the jar can hold.
    pub fn new(capacity: u32) -> Self {
        Self {
            capacity,
            cookies: 0
        }
    }

    /// Deposits the specified amount of cookies into the jar.
    /// Returns an error if the number of cookies in the jar exceeds the capacity after the deposit.
    ///
    /// # Arguments
    /// * `cookies` - Number of cookies to deposit.
    pub fn deposit(&mut self, cookies: u32) -> Result<(), JarError> {
        if self.cookies + cookies > self.capacity {
            Err(JarError::Overflow)
        } else {
            self.cookies = self.cookies + cookies;
            Ok(())
        }
    }

    /// Withdraws the specified amount of cookies from the jar.
    /// Returns an error if the number of cookies in the jar is smaller than the amount to withdraw.
    ///
    /// # Arguments
    /// * `cookies` - Number of cookies to withdraw.
    pub fn withdraw(&mut self, cookies: u32) -> Result<(), JarError>{
        if cookies > self.cookies {
            Err(JarError::Underflow)
        } else {
            self.cookies -= cookies;
            Ok(())
        }
    }

    /// The max number of cookies that the jar can hold.
    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    /// Number of cookies that the jar currently holds.
    pub fn size(&self) -> u32 {
        self.cookies
    }
}

impl Display for CookieJar {
    // Shows the jar as a string,
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "🍪".repeat(self.cookies as usize))
    }
}

pub fn main() {
    // Reads the jar's capacity from stdin and creates the jar.
    let mut input = String::new();
    print!("Input the cookie jar's capacity: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let mut jar = CookieJar::new(input.trim_end().parse().unwrap());
    println!("{HELP}");

    // Reads commands until exit command is inputted.
    loop {
        // Reads next command.
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim_end().split_whitespace();

        // Parses the command and acts depending on the type of command and supplied arguments.
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