// hashmap example - updating a value in the Hashmap based on the old value
// In this example, a unique way of using the Rust code is described
// the entry() method is used to see if a key exists,
// then if the key exists, the entry method returns a mutable reference to 
// value at that key
// we can then use this reference to locate the value at the address and increment
// it by using the dereference "*" operator

// pay attention to the comments in the for loop

use std::collections::HashMap;

// The below code is an algorithm to create a hashmap with the keys as the words
// stored in the text and value is the number of times that word occurs in the
// text
fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // count will have the &mut V value - i.e, mutable reference to
        // the value stored at the current "word" key
        let count = map.entry(word).or_insert(0);

        // with this syntax below we are saying take the value stored at the
        // address pointed by count and increment that value by 1
        // Next time we are in the for loop, the count will point to the address
        // of the next word
        // If the word is already present, we get the address of that value 
        // again to increment
        // PRETTY AWESOME ALGORITHM

        // Note the first time, the word is encountered, we put value = 0 per above
        // and then per below we immediately increment it by 1, 
        // this ensures that we start at 0 for each word and when we encounter that
        // word for the first time, the value is incremented by 1
        *count += 1;
        println!("Address of value at key = {} is {:p}", word, count);

    }

    println!("{:?}", map);
}

