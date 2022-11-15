// Adding "lifetime" genreic definitions to fix code
// Below lifetime syntax fixes the compilation errors from 71_lifet... code


fn main() {
    let string1 = String::from("abcd"); // String variable
    let string2 = "xyz"; // string literal which is a string slice
    
    // as_str() function converts the string1 string type to a string slice
    // string slices are passed as references to function
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// define the <'a> as the lifetime in the function signature like a generic variable
// With this we are telling the compiler that lifetime type variable will be used in
// this code. Note the ' in front of a. The ' indicates that this is a lifetime 
// variable

// lifetime variables are not telling the length of the scope of each variable,
// it only is used to establish the lifetime of a variable based on relationships
// of the variables. 

// For example below the return value will have a lifetime of atleast the smallest
// lifetime of the two variables x or y. 

// Code will now compile

// below syntax tell rust that the two variables x and y will live atleast
// as long as the lifetime 'a. 
// returned string slice will also have a lifetime as long as 'a
// In practice it means that the lifetime 'a will be atleast as long as the 
// smallest lifetime between x and y. 

// See next example in 74_life... for more examples of how lifetimes work

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {

    if x.len() > y.len() {
        x
    } else {
        y
    }

}