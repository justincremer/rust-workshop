#[macro_export]
macro_rules! just_saying_hi {
    () => {
        println!("Hello, world!");
    };
}

#[macro_export]
macro_rules! welcome {
    ($s:expr) => {
        println!("Welcome, {}!", $s);
    };
}

fn main() {
    just_saying_hi!();
    welcome!("justin");
}
