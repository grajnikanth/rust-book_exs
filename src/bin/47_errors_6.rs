// Shortcuts to avoid using multiple matches for errors when using Result<> Enum

// Result::expect() function

use std::fs::File;

fn main() {
    // expect will return the Ok variant with file handle if no error occurs
    // when opening hello.txt
    // If error occurs, the expect will panic with the error message
    // we send it in the argument. It will easier to debug our code if 
    // we can provide the error message
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}