// variation of previous code to illustrate how lifetimes will work

// Note string2 which is referenced to "y" will have scope only within 
// the interior code block. string2 will be out of scope or dropped at the
// end of interior code block

// the below code will strill work because, result variable will point to y 
// which inturn points to string2 is whithin interior code block and hence will
// be valid 

fn main() {
    let string1 = String::from("abcd"); // String variable
    
    {
        let string2 = "xyz"; // string literal which is a string slice
        // as_str() function converts the string1 string type to a string slice
        // string slices are passed as references to function
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }
    
    

}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {

    if x.len() > y.len() {
        x
    } else {
        y
    }

}