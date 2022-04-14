// HashMaps store data on the heap
// All of the keys should be same type and all of the keys have to same type



// creating new HashMap
// Create empty HashMap using the new() function
// add key value pairs using the insert() method
// HashMap is very similar to mapping in Solidity
// hash of keys is stored in the data structure
// HashMap is a type of collection

use std::collections::HashMap;

fn main() {

    // create new mutable HashMap data structure
    let mut scores = HashMap::new();

    // for scores(k,v) - k = key is a string, v = value is i32 per below
    // Rust will figure out the type of key and value when data is inserted into
    // it
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // accessing the value from the HashMap using keys
    // get() function returns a Option<&T> type value similar to Vectors
    println!("Value stored in scores HashMap at key = blue is = {:?}", scores.get("Blue"));
}