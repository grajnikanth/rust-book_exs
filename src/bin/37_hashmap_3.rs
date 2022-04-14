//Hash Map and Ownership
// i32 type data is copied into HashMap as i32 implements copy trait
// String type data is moved into the HashMap and HashMap will own this data as copy
// trait is not implemented on Strings
// Chapter 10 has details of how references work with hash maps

use std::collections::HashMap;

fn main() {

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // Both the variables field_name and field_value have lost ownership of the
    // data they were storing to the hashmap when this line of code is reached
    // The two variables are not valid anymore. Below code will error out
    println!("Field_name is = {}", field_name);
    println!("Field_name is = {}", field_value);
}
