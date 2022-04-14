//Use sructs to add meaning by labeling the data
// group variable together which have meaning together. Sides of a rectangle here
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    //pass reference of rect1 to area function. main function retains ownership of rect1
    println!("Area of rect1 =  {}", area(&rect1));
}

//function signature says calculate area of rectangle
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width*rectangle.height
}