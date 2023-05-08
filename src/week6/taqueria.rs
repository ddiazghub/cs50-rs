use std::collections::HashMap;
use std::io;
use std::io::Write;

/// Error caused by trying to buy an item which is not in the taquería's menu.
struct InvalidItem;

/// A taquería which allow to buy items from a menu.
struct Taqueria<'a> {
    /// A hashmap where each key is the name of a taquería's item and each value is the item's price in USD.
    menu: HashMap<&'a str, f64>,
    /// The sum of all bought items.
    total: f64
}

impl <'a> Taqueria<'a> {
    /// Creates a new Taquería with the given menu.
    ///
    /// # Arguments
    /// * `menu` - The taquería's menu.
    pub fn new(menu: HashMap<&'a str, f64>) -> Self {
        Self {
            menu,
            total: 0.0
        }
    }

    /// Buys an item and adds it's price to the total. Returns an error if the item does not exist.
    ///
    /// # Arguments
    /// * `item` - The name of the item to add.
    pub fn add(&mut self, item: &str) -> Result<f64, InvalidItem> {
        match self.menu.get(item) {
            Some(&value) => {
                self.total += value;
                Ok(self.total)
            },
            _ => Err(InvalidItem)
        }
    }
}

pub fn main() {
    // Creates the taquería's menu.
    let menu = HashMap::from([
        ("baja taco", 4.00),
        ("burrito", 7.50),
        ("bowl", 8.50),
        ("nachos", 11.00),
        ("quesadilla", 8.50),
        ("super burrito", 8.50),
        ("super quesadilla", 9.50),
        ("taco", 3.00),
        ("tortilla salad", 8.00),
    ]);

    // Creates the taqueria.
    let mut taqueria = Taqueria::new(menu);

    loop {
        // Reads the name of the item from stdin until EOF.
        print!("Item: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let bytes = io::stdin().read_line(&mut input).unwrap();

        if bytes == 0 {
            break
        }

        // Adds the item's price to the total if the item exists.
        if let Ok(total) = taqueria.add(input.trim_end()) {
            println!("Total: ${total:.2}");
        }
    }
}