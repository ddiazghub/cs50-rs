use super::helpers;

pub mod hello {
    pub fn main() {
        let name = super::helpers::read_line("What's your name pal? ").unwrap();
        println!("Hello {}!", name);
    }
}

pub mod mario_less {
    pub fn main() {
        let height: usize = loop {
            match super::helpers::read_line("Please input the height of the pyramid: ").unwrap().parse() {
                Ok(n) if n > 0 && n < 9 => break n,
                _ => eprintln!("The height should be a positive number between 1 and 8.")
            };
        };
    
        let mut pyramid = String::new();
    
        for i in 0..height {
            pyramid.push_str(&" ".repeat(height - i - 1));
            pyramid.push_str(&"#".repeat(i + 1));
            pyramid.push('\n');
        }
    
        println!("{}", pyramid);
    }
}

pub mod mario_more {
    pub fn main() {
        let height: usize = loop {
            match super::helpers::read_line("Please input the height of the pyramid: ").unwrap().parse() {
                Ok(n) if n > 0 && n < 9 => break n,
                _ => eprintln!("The height should be a positive number between 1 and 8.")
            };
        };

        let mut pyramid = String::new();

        for i in 0..height {
            pyramid.push_str(&" ".repeat(height - i - 1));
            pyramid.push_str(&"#".repeat(i + 1));
            pyramid.push_str("  ");
            pyramid.push_str(&"#".repeat(i + 1));
            pyramid.push('\n');
        }

        println!("{}", pyramid);
    }
}

pub mod cash {
    pub fn main() {
        let mut cents = get_cents();
        let mut coins = calculate_quarters(&mut cents);
        coins += calculate_dimes(&mut cents);
        coins += calculate_nickels(&mut cents);
        coins += calculate_pennies(&mut cents);

        println!("Coins: {}", coins);
    }

    fn get_cents() -> i32 {
        let cents: i32 = loop {
            match super::helpers::read_line("Please input the number of cents: ").unwrap().parse() {
                Ok(n) if n >= 0 => break n,
                _ => eprintln!("Please input a positive integer")
            }
        };

        cents
    }

    fn coins_for_change(cents: &mut i32, coin_weight: i32) -> i32 {
        let mut coins = 0;

        while *cents >= coin_weight {
            coins += 1;
            *cents -= coin_weight;
        }

        coins
    }

    fn calculate_quarters(cents: &mut i32) -> i32 {
        coins_for_change(cents, 25)
    }

    fn calculate_dimes(cents: &mut i32) -> i32 {
        coins_for_change(cents, 10)
    }

    fn calculate_nickels(cents: &mut i32) -> i32 {
        coins_for_change(cents, 5)
    }

    fn calculate_pennies(cents: &mut i32) -> i32 {
        coins_for_change(cents, 1)
    }
}

pub mod credit {
    enum CreditCardType {
        Visa,
        MasterCard,
        Amex,
        Invalid
    }

    pub fn main() {
        let number = get_long();
        let card_type = credit_card_type(number);

        match (card_type, luhn(number)) {
            (CreditCardType::Invalid, _) | (_, false) => println!("INVALID"),
            (CreditCardType::Amex, true) => println!("AMEX"),
            (CreditCardType::MasterCard, true) => println!("MASTERCARD"),
            (CreditCardType::Visa, true) => println!("VISA")
        }
    }

    fn get_long() -> i64 {
        let number: i64 = loop {
            match super::helpers::read_line("Please input the credit card number: ").unwrap().parse() {
                Ok(n) => break n,
                _ => eprintln!("Please input a positive integer")
            };
        };

        number
    }

    fn credit_card_type(number: i64) -> CreditCardType {
        let first_digits = number / 10e11 as i64;

        let card_type = match first_digits {
            5100..=5599 => CreditCardType::MasterCard,
            340..=349 | 370..=379 => CreditCardType::Amex,
            4000..=4999 | 4 => CreditCardType::Visa,
            _ => CreditCardType::Invalid
        };

        card_type
    }

    fn luhn(mut number: i64) -> bool {
        let mut sw = true;
        let mut sum = 0;
        
        while number > 0 {
            let digit = number % 10;

            sum += if sw {
                digit
            } else {
                let digit2 = 2 * digit;

                match digit2 {
                    0..=9 => digit2,
                    _ => 1 + digit2
                }
            };

            sw = !sw;
            number /= 10;
        }

        sum % 10 == 0
    }
}