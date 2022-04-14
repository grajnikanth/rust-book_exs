// Shortcuts to avoid using multiple matches for errors when using Result<> Enum

// Result::unwrap() function

use std::fs::File;

fn main() {
    // unwrap will return the Ok variant with file handle if no error occurs
    // when opening hello.txt
    // If error occurs, the unwrap will panic with the error
    // unwrap helps us eliminate all the match code we wrote in the previous example
    let f = File::open("hello.txt").unwrap();
}