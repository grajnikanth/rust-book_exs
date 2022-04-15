// Book reference listing 13-12
// Closure capture of variables of the environment next to the closure function
// definition

fn main() {
    let x = 4;
    
    // below is a closure function
    // note that variable x is being captured by the closure function to use inside
    // it's body
    // this is possible because closures implement Fn trait, which let's closure
    // do a immutable borrow in this case
    // Also FnOnce - lets you take ownership of environment variable
    // FnMut let's you mutably borrow values and change them
    // move is used if you want closure take ownership of the environment variable
    // Rust figures out based on how the environment variable was used to determine
    // if the variable was moved or not, immutable borrow or not etc
    let equal_to_x = |z| {z == x};

    let y = 4;

    assert!(equal_to_x(y));

}