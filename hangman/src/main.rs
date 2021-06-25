mod game;
use game::Session;

fn main() {
    let game = Session::new(&"Hello World", 5);
    println!(
        "{} {} {} {}",
        game.answer, game.hidden, game.letters_guessed, game.guesses_left
    )
}
