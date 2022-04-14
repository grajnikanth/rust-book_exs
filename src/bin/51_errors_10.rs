// Using the fs::read_to_string
// This function opens the file, reads the string and stores it in a string
// and returns an error
// With this function we eliminate the code to open file, then read the file etc


use std::fs;
use std::io;

fn main() {
    let ret = read_username_from_file();
    println!("Responce back from functio is {:?}", ret);
}


fn read_username_from_file() -> Result<String, io::Error> {

    // We are returning the result of the below code which is the 
    // string or the error
    fs::read_to_string("hello.txt")
}
