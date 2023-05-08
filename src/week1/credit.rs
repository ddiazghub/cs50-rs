/// Available types of credit cards.
enum CreditCardType {
    Visa,
    MasterCard,
    Amex,
    Invalid
}

pub fn main() {
    // Reads credit card number and finds which type it belongs to.
    let number = get_long();
    let card_type = credit_card_type(number);

    match (card_type, luhn(number)) {
        (CreditCardType::Invalid, _) | (_, false) => println!("INVALID"),
        (CreditCardType::Amex, true) => println!("AMEX"),
        (CreditCardType::MasterCard, true) => println!("MASTERCARD"),
        (CreditCardType::Visa, true) => println!("VISA")
    }
}

/// Reads an i64 from stdin.
fn get_long() -> i64 {
    let number: i64 = loop {
        match super::helpers::read_line("Please input the credit card number: ").unwrap().parse() {
            Ok(n) => break n,
            _ => eprintln!("Please input a positive integer")
        };
    };

    number
}

/// Finds to which type a credit card belongs to.
///
/// # Arguments
/// * `number` - The credit card's number.
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

/// Checks if a credit card is valid using luhn's algorithm.
///
/// # Arguments
/// * `number` - The credit card's number.
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