// Concatenation with the + operator and format! macro
// "+" operator uses the add method internally, the signature of add is as follows:
// fn add(self, s: &str) -> String {}
// so string to be appended has to be a str slice
// if &String type is passed as second argument, then the compiler will "coerce" the
// &String into &str. This is done using deref coercion - &String[..]
// add() funtion does not take ownership of "s" here as it is passed as a reference
// but the first argument "self" varibale is passed to the function, and the 
// function takes ownershup of the first argument.
// Very good explanation of how add() works below listing 8-18.

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // "+" operator uses add() function behind the scenes
    // s1 ownership is passed to the add function so s1 is not in this main scope
    // after the below line
    // add method returns a String
    let s3 = s1 + &s2; 

    println!("s3 value with s1 + &s2 = {}", s3);
    
    //below line will not work because s1 ownershup was passed to the add() function
    // println!("s1 not owned by main() function anymore {}", s1);


    // More examples of "+" operator concatenation
    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");

    let s7 = s4 + "-"+ &s5 + "-" + &s6;
    
    println!("s7 with concatenated string is {}", s7);

    //alternate to "+" operator is the format! macro
    // format! macro does not take ownership of the arguments,
    // The arguments are taken as references
    let s8 = String::from("tic");
    let s9 = String::from("tac");
    let s10 = String::from("toe");

    // s1, s2 etc below were not passed as references but I guess the function 
    // internally converts them to obtain the references to these variables
    let s11 = format!("{}-{}-{}", s8, s9, s10);
    println!("s11 generated using format! function = {}", s11);

}