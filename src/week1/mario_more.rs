pub fn main() {
    // Reads pyramid height from stdin.
    let height: usize = loop {
        match super::helpers::read_line("Please input the height of the pyramid: ").unwrap().parse() {
            Ok(n) if n > 0 && n < 9 => break n,
            _ => eprintln!("The height should be a positive number between 1 and 8.")
        };
    };

    // Builds pyramid.
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