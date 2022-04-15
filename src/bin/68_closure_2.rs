// Book reference after listing 13-12 
// Closure capture of variables of the environment next to the closure function
// definition

fn main() {
    let x = vec![1, 2, 3];
    
    // We can move the ownership of environment variables to the closure by using the 
    // move keyword
    // Below ownership of x is moved to the closure 
    // Hence x is not available after the next statement in the main scope

    let equal_to_x = move |z| {z == x};

    // Below code will not compile as x is not in main scope anymore
    println!("can't use x here as it has moved to closure = {:?}", x);
    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));

}