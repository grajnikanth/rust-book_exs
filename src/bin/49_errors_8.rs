// "?" operator to propagate errors - 
// propagating errors shown in 48_errors_7.rs is common pattern so 
// Rust provides "?" opertator to simplify all the match code we wrote
// You can eliminate all the match code and instead use the "?" operator

// Below code in the read_username_from_file() function has all the same
// functionality as code in 48_errors_7.rs 


use std::fs::File;
// I think self here means std::io. 
// Since we want to use std::io::Read also we use this self syntax
use std::io::{self, Read};

fn main() {
    let ret = read_username_from_file();
    println!("Responce back from functio is {:?}", ret);
}


fn read_username_from_file() -> Result<String, io::Error> {

    // "?" operator applied to the File::open() function. Note File::open -> Result<T,E>
    // ? below will do the following:
    //  if Result value is Ok, then "?" will return the value inside ok to variable f,
    //  and the code will continue
    //  if Result value is an Err variant, then the "?" will return from this function
    //  with the Err() as the variant.
    let mut f = File::open("hello.txt")?;

    // use the "s" variable to store the string read from hello.txt
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
    
    // "?" operator does the same again, the contents are appended to s if 
    // Result is Ok variant, otherwise, Err will be returned and function
    // terminates
    f.read_to_string(&mut s)?;
    // if above is successful, "s" would have the data read from the file
    // so we can return it as shown below
    Ok(s)
}

