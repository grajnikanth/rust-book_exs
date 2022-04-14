// Generics on method signature are independent of generics on structs
// below we define a function whose signature uses a different type of data than
// what is defined on the impl<>. The function signatures can work with any 
// other type of generic types

// x and y could be of different types X1 and Y1
struct Point<X1, Y1> {
    x: X1,
    y: Y1
}

// X1, Y1 are the generic types on the Struct
impl<X1, Y1> Point<X1, Y1> {

    // mixup function can take a different type of data as opposed to what the 
    // instance of Point was originally defined on
    // This function will take a Point<X1,Y1> and convert it into a Point<X1, Y2>

    // Note here should self be &self in function arguments
    // Note X2, Y2 generics only go with mixup so this works. Functions
    // can have their own generics different from the Struct types
    // This is because functions can do other computation which may not 
    // be the same type as the struct
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y
        }
    }

}

fn main() {
    // p1 is Point<i32, f32>
    let p1 = Point { x: 5, y: 10.4 };

    // p2 is Point<&str, char>
    let p2 = Point {x : "hello", y: 'c'};

    // p3 will be Point<i32, char>
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}