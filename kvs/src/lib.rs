use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use std::fs::File;
// use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    items: HashMap<String, String>,
}

// trait Persist {
//     fn save(&self, file: &str);
//     fn load(&self, file: &str);
// }

// impl Persist for Store {
//     fn save(&self, file: &str) {
//         let path = Path::new(file);
//         let display = path.display();
//         let file = match File::open(&path) {
//             Err(e) => panic!("could not open {}: {}", display, e),
//             Ok(v) => v,
//         };

//         match serde_json::to_writer(file, &self.items) {
//             Err(e) => panic!("failed to write store to {} -> {}", display, e),
//             Ok(v) => v,
//         };
//     }

//     fn load(&self, file: &str) {
//         let path = Path::new(file);
//         let display = path.display();
//         let file = match File::open(&path) {
//             Err(e) => panic!("could not open {}: {}", display, e),
//             Ok(v) => v,
//         };
//         let res = match serde_json::from_reader(file) {
//             Err(e) => panic!("failed to write store to {} -> {}", display, e),
//             Ok(v) => v,
//         };
//     }
// }

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
}
