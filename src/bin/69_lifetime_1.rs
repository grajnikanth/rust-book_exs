// Listing 10-17 - Example of how refernce lifetime works

// References reference to data stored in memory. Depending on which variable
// has ownership of that data and which other variable is referencing that data
// scope length becomes relavant. For examlpe is the reference variable still in
// scope. Is the data in a variable is still in scope or was it dropped etc

fn main() {

    let r; // r in main scope

    {
        // x is in scope only in this code block
        let x = 5;
        r = &x; 
    } // x is dropped at this line, but r is still referencing x memory after it is
    // dropped

    // The below code won't compile because r is pointing to x which has
    // been dropped already
    // r is valid for longer time than x. r lifetime is longer than x

    // x does not live long enough for r to still reference the memory address of x
    println!("r: {}", r);

    // Rust does this check automatically using it's borrow checker
    // `a represents a lifetime in Rust
    // `b represents a lifetime of a different variable etc

}