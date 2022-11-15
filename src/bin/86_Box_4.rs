// Implementation of MyBox<T> smart pointer. This smart pointer will be similar
// to the Box<T> smart pointer

use std::ops::Deref;

// The below syntax is defining a struct which holds a tuple of T that's it
// This struct is different than what I have typically seen. This is a struct
// which is a tuple with one element
// I guess because tuples do not need a name to be used to define name:key pair
// so this syntax works
#[derive(Debug)]
struct MyBox<T>(T);

// The new function returns a new MyBox<T> smart pointer
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// implement Deref Trait on MyBox. Need one method named deref which borrows self
// and returns a reference to the inner data
impl<T> Deref for MyBox<T> {
    // The below syntax is for associate type for Deref trait to use
    type Target = T;

    // the deref has to return a reference to the data inside the MyBox struct
    // we accomplish this by returning reference to self.0. Note self is a tuple
    // The return type is reference to MyBox T. T iin this case is set to the 
    // custom type called Target
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    // the new function gets a copy of x
    let y = MyBox::new(x);

    println!("{:?}", y);
    // *y actually internally rust does *(y.deref()). Rust internally calls 
    // the deref function on y
    println!("*y is {:?}", *y);

    // Example of Deref coercion automatically by Rust
    let m = MyBox::new(String::from("Rust"));
    // &m is reference to MyBox<String>. Rust converts &MyBox<String> to
    // &String and then &string to &str as MyBox and String both implement
    // Deref trait by making multiple calls to the deref function internally
    // So below hello() function get a &str
    hello(&m);
}

// Deref coercion example
fn hello(name: &str) {
    println!("Hello, {}", name);
}