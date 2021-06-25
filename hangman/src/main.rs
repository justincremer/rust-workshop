mod game;
use game::Session;

fn main() {
    let mut game = Session::new("Hello World", 5);
    match game.guess('c') {
        Err(e) => std::panic::panic_any(e),
        Ok(_) => {}
    };
    game.test_display();
}
