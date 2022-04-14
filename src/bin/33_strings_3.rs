// Slicing Strings
// Can use range index with str slice to obtain specific bytes
// With this approach the returned string can be customized to return say 2 bytes 
// or 3 bytes based on how many ever bytes it took to encode character in UTF-8 numbers

fn main() {
    
    // The below is cryllic script and each character needs two bytes to store the
    // utf-8 encoded number
    // below is defined as a string slice
    let hello = "Здравствуйте";

    // Retrieve two bytes at a time using str slice with range operator
    // [0..2] is 1st and 2nd byte
    // hello is a string slice reference and below indexing type syntax with
    // range is allowed in rust but not index at specific index. 
    let s1 = &hello[0..2];
    println!("s1 representing 2 bytes which is first character in string = {}", s1);

    // [2..4] is 3rd and 4th byte
    // These two bytes represent the 2nd character in the hello string
    let s2 = &hello[2..4];
    println!("s2 with 3rd and 4th byte = 2nd character is = {}", s2);

    // [3] try single number in index
    // below code will give compilation error as the index has to be a range in 
    // Rust not a single integer
    // let s3 = &hello[3];
    // println!("s3 = &hello[3] is = {}", s3);

    // Lets try printing the first byte of the string slice for above and
    // see what the output is
    // the below code also does not work. It does not crash the program but
    // we get an error at runtime saying byte 1 is not a character boundary consistent
    // with what we need 2 bytes for each character
    // str slices with range index should be used with caution to avoid runtime
    // errors
    let s4 = &hello[0..1];
    println!("s4 = &hello[0..1]= {}", s4);

}