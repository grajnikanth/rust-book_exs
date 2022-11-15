// Fixing reference lifetime of r from example in 69_lifetime_1.rs
// define x first and then define r to reference x, so lifetime of x > r and
// 

fn main() {

    let x = 5;
    let r = &x; 
    
    println!("r: {}", r);


} // x and r go out of scope here so lifetime of r < x so r will not be a
// dangling pointer
// lifetimes of variables in Rust are referenced as 'a, 'b etc