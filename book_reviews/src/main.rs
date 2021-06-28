use std::collections::HashMap;
use std::vec::Vec;

fn main() {
    let mut book_reviews: HashMap<&str, u8> = HashMap::new();
    println!("\nDatabase initialized");

    fn add_books(map: &mut HashMap<&str, u8>) {
        insert_many(
            map,
            vec![
                ("Capital V1", 8),
                ("Capital V2", 7),
                ("Capital V3", 6),
                ("Living my Life", 9),
                ("The Communist Manifesto", 8),
            ],
        );
    }

    display(&book_reviews);
    add_books(&mut book_reviews);
    display(&book_reviews);

    book_reviews.remove("The Communist Manifesto");
    book_reviews.remove("Capital V2");
    display(&book_reviews);

    let title = "The conquest of Bread";
    println!();
    display_review(&book_reviews, title);
    book_reviews.insert(title, 8);
    display_review(&book_reviews, title);

    println!("\nDraining...");
    book_reviews.drain().for_each(|p| println!("{:?}", p));

    print!("\nFinished draining...");
    display(&book_reviews);
}

fn insert_many<'a>(map: &mut HashMap<&'a str, u8>, items: Vec<(&'a str, u8)>) {
    for i in items.iter() {
        let (k, v) = i;
        map.insert(k, *v);
    }
}

fn display(map: &HashMap<&str, u8>) {
    println!();
    if map.is_empty() {
        println!("The database is empty");
        return;
    }
    map.into_iter().for_each(|b| println!("{:?}", (b.0, b.1)));
}
fn display_review<'a>(map: &HashMap<&str, u8>, k: &str) {
    println!("{}", get_review(map, k));
}

fn get_review<'a>(map: &HashMap<&str, u8>, k: &str) -> String {
    match map.get(k) {
        None => format!("{} does not exist in our database", k),
        Some(v) => format!("{} has a rating of {}", k, v),
    }
}
