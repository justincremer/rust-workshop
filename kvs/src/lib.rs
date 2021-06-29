use std::collections::HashMap;

pub struct Store {
    items: HashMap<String, String>,
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

impl Store {
    pub fn new() -> Self {
        Store {
            items: HashMap::new(),
        }
    }

    pub fn set(&mut self, k: &str, v: &str) {
        self.items.insert(String::from(k), String::from(v));
    }

    pub fn get(&self, k: &str) -> Option<String> {
        match self.items.get(k) {
            Some(v) => Some(v.to_string()),
            None => None,
        }
    }

    pub fn remove(&mut self, k: &str) {
        self.items.remove(k);
    }

    // pub fn display(&self, k: &str) {
    //     println!(
    //         "{}",
    //         match self.get(k) {
    //             Some(v) => v,
    //             None => String::from("Item does not exist"),
    //         }
    //     );
    // }
}
