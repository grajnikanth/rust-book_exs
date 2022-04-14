// Overwriting an existing value in Hash Map

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    // The below would overwrite Blue: 10 to Blue: 25
    scores.insert(String::from("Blue"), 25);
    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}