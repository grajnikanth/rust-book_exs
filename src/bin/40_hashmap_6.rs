// Inserting into hash map only if the key already does not exist
// user the entry() function which returns enum Entry 
// on enum Entry call the or_insert() function 
// In this way we use all Rust functions to do this rather than writing this 
// logic our selves

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // Check if Yellow key exists if not insert "Yellow": 50 into the hash map 
    scores.entry(String::from("Yellow")).or_insert(50);
    // Below will not insert "Blue": 50, since "Blue": 10 already exists 
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}