mod game;
use game::Session;

fn main() {
    let ses = Session::new("Hello World", 5);
    println!("Hello, world!");
}
