//Rust attribute usage - 
//{} inside println is std::fmt::Display format default. Structs do not implement this
// {:?} - implies display the variable in debug mode - for developers. 
// For structs, you have to opt in to use debug display using an attribute 
// attribute is #[derive(Debug)] 

#[derive(Debug)] //assigning attribute to Rectangle 
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
    println!("Print Struct fields in debug mode  {:?}", rect1);
    //print in prettier format using {:#?} 
    println!("Print Struct fields in debug mode  {:#?}", rect1);
}