pub fn main() {
    // Reads name and prints hello message.
    let name = super::helpers::read_line("What's your name pal? ").unwrap();
    println!("Hello {}!", name);
}