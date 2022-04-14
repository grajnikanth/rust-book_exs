//Example of using match on the standard library Option enum

//Standard library Option enum is as follows:
//      pub enum Option<T> {
//          None,
//          Some(T),
//      }

//argument x will of the type Option with data for Some variant as i32
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        //Note "None" variant of Option is by default available in all Rust programs
        // from standard library
        None => None,
        // The "i" variable below binds to the Some(5) etc sent into the function
        // using the variable x
        // In return we add 1 to this i and send back a Some() variant with data in it
        Some(i) => Some(i+1)
    }
}

fn main() {
    // Using Option<i32> enum variant Some() directly. Note Option::Some() syntax
    // is not needed as Some is available directly
    let five = Some(5);
    let six = plus_one(five);
    //None variant of Option enum is directly available in the code
    let none = plus_one(None);

    // Note variable five was still accessible here. So it means Rust dod not move
    // the ownership of five to plus_one function. Do enums are always borrowed as references by default?
    println!("Value of Option enum five is {:?}", five);
    println!("Value of Option enum six is {:?}", six);
    println!("Value of Option enum none is {:?}", none);    
}

