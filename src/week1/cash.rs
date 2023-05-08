pub fn main() {
    let mut cents = get_cents();
    let mut coins = calculate_quarters(&mut cents);
    coins += calculate_dimes(&mut cents);
    coins += calculate_nickels(&mut cents);
    coins += calculate_pennies(&mut cents);

    println!("Coins: {}", coins);
}

/// Reads number of cents to calculate change for from stdin.
fn get_cents() -> i32 {
    let cents: i32 = loop {
        match super::helpers::read_line("Please input the number of cents: ").unwrap().parse() {
            Ok(n) if n >= 0 => break n,
            _ => eprintln!("Please input a positive integer")
        }
    };

    cents
}

/// Finds the max number of coins with the specified weight that can be used to give change for the specified number of cents.
///
/// # Arguments
/// * `cents` - Number of cents to give change for.
/// * `coin_weight` - Value of each coin that will be given as change.
fn coins_for_change(cents: &mut i32, coin_weight: i32) -> i32 {
    let mut coins = 0;

    while *cents >= coin_weight {
        coins += 1;
        *cents -= coin_weight;
    }

    coins
}

/// Finds the max number of quarters that can be used to given change for the specified number of cents.
///
/// # Arguments
/// * `cents` - Number of cents to give change for.
#[inline]
fn calculate_quarters(cents: &mut i32) -> i32 {
    coins_for_change(cents, 25)
}

/// Finds the max number of dimes that can be used to given change for the specified number of cents.
///
/// # Arguments
/// * `cents` - Number of cents to give change for.
#[inline]
fn calculate_dimes(cents: &mut i32) -> i32 {
    coins_for_change(cents, 10)
}

/// Finds the max number of nickels that can be used to given change for the specified number of cents.
///
/// # Arguments
/// * `cents` - Number of cents to give change for.
#[inline]
fn calculate_nickels(cents: &mut i32) -> i32 {
    coins_for_change(cents, 5)
}

/// Finds the max number of pennies that can be used to given change for the specified number of cents.
///
/// # Arguments
/// * `cents` - Number of cents to give change for.
#[inline]
fn calculate_pennies(cents: &mut i32) -> i32 {
    coins_for_change(cents, 1)
}