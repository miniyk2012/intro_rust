use std::collections::HashMap;

pub fn insertDemo() {
    println!("\ninsertDemo");
    let mut book_reviews = HashMap::new();
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );
    // Check for a specific one.
    // When collections store owned values (String), they can still be
    // queried using references (&str).
    if !book_reviews.contains_key("Les Misérables") {
        println!(
            "We've got {} reviews, but Les Misérables ain't one.",
            book_reviews.len()
        );
    }

    book_reviews.remove("The Adventures of Sherlock Holmes");
    // Look up the values associated with some keys.
    let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
    for &book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{book}: {review}"),
            None => println!("{book} is unreviewed."),
        }
    }
    // Look up the value for a key (will panic if the key is not found).
    println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);
    println!();
    // Iterate over everything.
    for (book, review) in &book_reviews {
        println!("{book}: \"{review}\"");
    }
}

pub fn entryDemo() {
    println!("\nentryDemo");
    // type inference lets us omit an explicit type signature (which
    // would be `HashMap<&str, u8>` in this example).
    let mut player_stats = HashMap::new();

    fn random_stat_buff() -> u8 {
        // could actually return some random value here - let's just return
        // some fixed value for now
        42
    }

    // insert a key only if it doesn't already exist
    player_stats.entry("health").or_insert(100);

    println!("{}", player_stats["health"]);

    // insert a key using a function that provides a new value only if it
    // doesn't already exist
    player_stats
        .entry("defence")
        .or_insert_with(random_stat_buff);

    // update a key, guarding against the key possibly not being set
    let stat = player_stats.entry("attack").or_insert(100);
    *stat += random_stat_buff();

    // modify an entry before an insert with in-place mutation
    player_stats
        .entry("mana")
        .and_modify(|mana| *mana += 200)
        .or_insert(100);
}
