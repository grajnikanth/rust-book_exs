// "?" operator and chaining
// chaining open and read_to_string
// Below code code does what the 49_errors_9.rs does but using chaining 
// of functions


use std::fs::File;
// I think self here means std::io. 
// Since we want to use std::io::Read also we use this self syntax
use std::io::{self, Read};

fn main() {
    let ret = read_username_from_file();
    println!("Responce back from functio is {:?}", ret);
}


fn read_username_from_file() -> Result<String, io::Error> {

    // chaining of the functions with "?" operator

    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

