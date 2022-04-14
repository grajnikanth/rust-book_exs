// Iterating over an HashMap

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // for loop takes a reference to the scores HashMap
    // this is similar to that of how we looped using vectors.
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}