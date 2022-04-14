
// collect method gathers data into a number of collection types, including HashMaps
// The collect method gathers data into a number of collection types, including HashMap
// zip method and iterator of tuples
// chapter 13 of book will provide details on this

use std::collections::HashMap;

fn main() {
    
    // two vectors below will be converted into a HashMap with keys from teams
    // vector and values from corresponding initial_score vector
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    
    // collect function is used to collect the data into a HashMap type here
    // collect can arrange this data into other type of data structures as well
    // _ in the HashMap is ok because Rust can figure out type of data for keys and 
    // values based on the data in the vectors
    let mut scores: HashMap<_,_> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("Value stored in scores HashMap at key = blue is = {:?}", scores.get("Blue"));
    
}

