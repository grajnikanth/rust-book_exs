// Propagating errors from one function to another function (main) etc

// Returning Result<T,E> from a function
// Using match to return the Result<> Type explicitly
// When propagating an error, we can customize the Result<> to whatever
// we want

use std::fs::File;
// I think self here means std::io. 
// Since we want to use std::io::Read also we use this self syntax
use std::io::{self, Read};

fn main() {
    let ret = read_username_from_file();
    println!("Responce back from functio is {:?}", ret);
}

// We are customizing the return of the below function saying it
// will return a Result<> type variable 
// In the Result we are saying we will retun a Ok(String) if we are
// successful in whatever we are doing in this function. In this case
// we will return variant Ok() with username read from file
// If we encounter an error we will return a Err(io::Error) 
// io::Error is the std::io::Error
// Rust book is saying that both open() and read_to_string() functions will retturn
// an error of type io:Error hence that was used here in the return
fn read_username_from_file() -> Result<String, io::Error> {

    // note f here is going to get a Result<> type value
    let f = File::open("hello.txt");

    // Here f will be assigned a file handle if the first line of code
    // is matched. 
    let mut f = match f {
        // same as before return file handle to res variable if f
        // variable above returned Result Ok() variant
        Ok(file) => file,
        // if variable f got a Result Err() variant, then 
        // return Err(e) and get out of this function
        // Note Err(e) is a Result Err() variant. The value "e"
        // will be of type io::Error 
        Err(e) => return Err(e),
    };

    // if above res = file, then code continue to here

    // we will extract the username from the file handle using the 
    // variable f
    let mut s = String::new();

    // since "f" is a file handle, we can call the read_to_string function
    // if "f" were a Result type variable, you couldn't call the read_to_string function
    // f.read_to_string() function of std::fs::File will read the file contents
    // and append it to the mut s variable sent in as an argument. 
    // If successful, the return value of this function is number of bytes
    // which were read
    // if not an error returned
    match f.read_to_string(&mut s) {
        // In the Ok(_) - it appears that we mean that whatever the value
        // which was returned. In this case it is the bytes read
        // I will print it to verify this
        Ok(bytes) => {
            println!("Number of bytes read is {}",bytes);
            // The below line is a return statement so this entire function
            // read_username_from.. will return when it reaches below line of 
            // code 
            // When we reach the line below we are ready to return from 
            // this function, so again we are using Ok(s) without the ";"
            // meaning we are returning this value
            // Ok(s) is a Result type variant with "s" as the data
            // which matches the String in our return statement
            // In this case this is the username
            Ok(s)
        },
        // Below we are returning the error from this entire function,
        // since this is the last line, we do not have to use the 
        // return keyword. But it is interesting how Rust works
        // this block of code 
        Err(e) => Err(e),
    }
}

