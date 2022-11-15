// variation 2 of previous code to illustrate how lifetimes will work

// Note string2 which is referenced to "y" will have scope only within 
// the interior code block. string2 will be out of scope or dropped at the
// end of interior code block

// The below code won't compiled by Rust because eventhough lifetimes were defined
// the result variable will be assigned a lifetime of string2 which is the smallest
// of lifetime between string1 and string2. So Rust will give compile error because
// result need to have a lifetime longer than string2 for this code to work
// In a way Rust is preventing the case where a dangling reference to value
// is created in the code at runtime by catching the error at compile time

// The code below compiled and worked even though it was supposed to error out
// per the rust lang book. 

fn main() {
    let string1 = String::from("ab"); // String variable
    let result;
    {
        let string2 = "xyz"; // string literal which is a string slice
        // as_str() function converts the string1 string type to a string slice
        // string slices are passed as references to function
        result = longest(string1.as_str(), string2);
        
    }

    // result may point to the string2 and string2 to will not be available here
    // so Rust will give a compile error
    println!("The longest string is {}", result);
    
    

}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {

    if x.len() > y.len() {
        x
    } else {
        y
    }

}