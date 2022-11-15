// Function below fails to compile because Rust does not know if the referenced
// variables will be in scope when the function executes

fn main() {
    let string1 = String::from("abcd"); // String variable
    let string2 = "xyz"; // string literal which is a string slice
    
    // as_str() function converts the string1 string type to a string slice
    // string slices are passed as references to function
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// function longest take string slice references as arguments 
// Also returns a reference to a string slice

// The code does not compile because we are returning a string slice reference in the
// function below. Rust cannot tell whether this reference will point to x or y at
// compile time
// Since we are returning a reference, Rust needs to be sure that the value at the
// memory address pointed to by the reference is still present when the reference is
// returned

// Rust requires that in these cases, the "lifetime" (scope) of the variable be given to it
// so it explicityly so it can be sure that the value exists at the memory
// See next example code in 73_life.. in which the return statement is removed
// and the code compiles because now rust does not have to know if a variable from 
// function is valid after the functions scope ends

// lifetime synyax to fix this code is in the example 74_lifetimes
fn longest(x: &str, y: &str) -> &str {

    if x.len() > y.len() {
        x
    } else {
        y
    }

}