//associated functions without self parameter - These are not called methods
//They are accessed using the namespace :: syntax not object.method() syntax
//These methods though associated with struct may not be a function needed by most
// structs of this type 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    //square is like a constructor function, doesn't need self parameter
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    //notice the namespace access of function syntax "::"
    let square1 = Rectangle::square(10);

    println!("Square1 Rectangle is {:#?}", square1);
}