// The code below which is a copy of the code from 71_life... compiles
// because we removed the return of reference to a string slice. Now
// Rust does not have to keep track if the returned value will be in scope 
// if it is a reference. 

fn main() {
    let string1 = String::from("abcd"); // String variable
    let string2 = "xyz"; // string literal which is a string slice
    
    // as_str() function converts the string1 string type to a string slice
    // string slices are passed as references to function
    longest(string1.as_str(), string2);
    
}

// function longest take string slice references as arguments 
// Also returns a reference to a string slice
fn longest(x: &str, y: &str) {

    if x.len() > y.len() {
        println!("x is largest");
        
    } else {
        println!("y is largest");
    }

}