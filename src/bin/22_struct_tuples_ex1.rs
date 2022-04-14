// struct tuples - tuples with a name
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    //instances of tuples
    let black = Color(10,11,12);
    let origin = Point(1,2,3);

    //accessing struct tuple with the tuple syntax
    println!("black.0 = {}", black.0);
    // syntax to destructure struct tuple - Note the "Point" in front of (x,y,z)
    // Specify that (x,y,z) is a Point struct tuple. otherwise it could be anything
    let Point(x,y,z) = origin;
    println!("x value destructured from origin tuple = {}", x);

}