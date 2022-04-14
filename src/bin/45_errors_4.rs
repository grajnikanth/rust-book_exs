// Matching on different errors

// error struct and its method error.kind()
// ErrorKind::NotFound
// File::create() function

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let res = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // if hello.txt is not found call the function File::create
            // Since File::create() returns a Result enum we can use a match on this
            // to handle different conditions
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            // Not sure how this other_error works below
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };
}