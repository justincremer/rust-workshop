use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("data.db");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(e) => panic!("could not open {}: {}", display, e),
        Ok(v) => v,
    };

    let mut res = String::new();
    match file.read_to_string(&mut res) {
        Err(e) => panic!("could not open {}: {}", display, e),
        Ok(_) => print!("{}:\n{}\n", display, res),
    }
}
