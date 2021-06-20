use std::collections::HashMap;

struct Item {
    price: u8,
    amount: u8,
}

struct Machine<'a> {
    balance: u32,
    items: HashMap<&'a str, Item>,
}

impl Machine<'_> {
    fn new<'a>() -> Machine<'a> {
        Machine {
            balance: 0,
            items: HashMap::new(),
        }
    }

    fn add_item(&self, key: &str, amount: u8, price: Option<u8>) {
        self.items
    }
}

fn main() {
    println!("Hello, world!");
}
