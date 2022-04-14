// Generics in methods of structs

struct Point<T> {
    x: T,
    y: T
}

// function x implemented on Point Struct
// function x returns a reference to field x of the instance which it was called on

// You have to specify  <T> right after impl to indicate that <T> type will be used
// Point<T> is based on the struct type on which we are defining functions
// so <T>  after Point is automatic
// <T> after impl indicates to Rust that <T> after Point is generic type not concrete type.
// With this these function will work on any instance no matter what type the instance is

impl<T> Point<T> {

    // The function x is returning a reference to type T 
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point {x: 5, y: 6};
    println!("p.x = {}", p.x());
}