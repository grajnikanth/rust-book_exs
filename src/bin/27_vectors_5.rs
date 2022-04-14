// For loops with vectors
// iterating over values in vectors
fn main() {
    //immutable vector v
    let v = vec![1, 2, 3, 4, 5, 6];

    println!("Printing the elements of v using for loop");
    // note the syntax of &v. So it appears we are iterating using the 
    // references of v. makes sense because v is stored in heap memory and
    // address of v is stored in stack. 
    //Note here i is also a immutable reference to each element
    for i in &v {
        println!("{}", i);
    }

    // Below statement leads to an error and shows that "i" in the for loop is not in the scope
    // anymore. Appears that "i" is deleted once for loop is finished
    //println!("Is i value still in scope {}", i);

    // The below for loop with using "v" also works. Is this slower
    // for loop than the one used above with "&v" because we are using the 
    // memory address with "&v"?
    println!("Loop through vector v without using &v");
    for i in v {
        println!("{}", i);
    }
}