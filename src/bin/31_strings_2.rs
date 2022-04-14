// Updating/growing a String
// Method push_str is similar to the push method on vectors
// push_str() function appends a string slice to an existing string
// push_str() method does not take ownership of the str slice passed to it
// ownership of str slice passed to push_str
// use push() method to append a single letter to a string

fn main() {

    // s is a String type value 
    let mut s = String::from("foo");
    
    // Use the push_str() method to push a string slice to append to an existing
    // string
    // This function takes in a str slice
    // push_str() takes a string slice because we don't necessarily want to take
    // ownershup of the parameter
    // note also that the original string s was modified by this method as "s" was
    // declared to be mutable
     s.push_str("bar");

    println!("Value of s after appending = {}", s);

    // s1 is a String, s2 is str slice. s2 is appended to s1 using push_str
    // s2 is still available after appending because s2 is str slice and ownership
    // stays with s2
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 value after appending the string is {}", s1);
    println!("s2 value is still accessible after it is appended = {}", s2);

    // push method using single letter
    let mut s3 = String::from("lo");
    s3.push('l');
    println!("s3 after appending l = {}", s3);
    
    // appending to str slice strings
    // str slices cannot be simply concatenated using the "+" operator
    // format! appears to be solution on stack exchange. 
    // Rust book might talk about it later
    let s4 = "Hello";
    let s5 = " World";

    let s6 = format!("{}{}", s4, s5);

    println!("concatenate using format! macro is = {}", s6);
}