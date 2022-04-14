//Methods on structs
// methods on structs always have &self as first parameter - represents instance of the struct
// the method is being called on

//this provides better organization of code to group functions specific to a struct

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

//all methods belonging to Rectangle can be defined in only place 
// under the impl block
impl Rectangle {
    //&self is actually "self: &self" in the parameters below
    fn area(&self) -> u32 {
        self.width*self.height
    }

    //second parameter is a reference to a different rectangle
    // method to check if the self rectangle is bigger than the second Rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//self is borrowed above, you can have "&mut self" or "self" - can take ownership to modify self 

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("Area of the rectangle is = {}", rect1.area());

    //use of the method can_hold
    let rect2 = Rectangle {
        width: 10,
        height: 20
    };

    println!("Is rect1 bigger than rect2? {}", rect1.can_hold(&rect2));

    let rect3 = Rectangle {
        width: 40,
        height: 60
    };

    println!("Is rect1 bigger than rect2? {}", rect1.can_hold(&rect3));

}

