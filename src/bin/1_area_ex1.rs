//example to show when we should use structs - to group related variables
// and have descriptive fields to make code clear what we are doing
// see refactored code with structs to show when to use structs
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of rectangle is = {}", area(width1, height1));
}
//Since width and height > 0 use u32
fn area(width: u32, height: u32) -> u32 {
    width*height
}