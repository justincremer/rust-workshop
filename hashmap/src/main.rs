use std::collections::HashMap;
use std::vec::Vec;

fn main() {
    let mut book_reviews: HashMap<&str, u8> = HashMap::new();

    book_reviews.insert("Capital V1", 8);
    book_reviews.insert("Capital V2", 7);
    book_reviews.insert("Capital V3", 6);

    display(&book_reviews);

    insert_many(
        &mut book_reviews,
        vec![
            ("Living my Life", 9),
            ("The Communist Manifesto", 8),
            (
                "The Communist Manifesto, but Tony Hawk gets to keep all of his stuff",
                10,
            ),
        ],
    );

    display(&book_reviews);

    book_reviews.remove("The Communist Manifesto");
    book_reviews.remove("Capital V2");

    display(&book_reviews);

    assert_eq!(book_reviews.contains_key("Capital V1"), true);
}

fn insert_many<'a>(map: &mut HashMap<&'a str, u8>, items: Vec<(&'a str, u8)>) {
    for i in items.iter() {
        let (k, v) = i;
        map.insert(k, *v);
    }
}

fn display(map: &HashMap<&str, u8>) {
    println!();
    map.into_iter().for_each(|b| println!("{:?}", (b.0, b.1)));
}
