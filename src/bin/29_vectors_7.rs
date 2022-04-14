// Vector to store enum type
// Vector elements have to be all of same type. So Rust knows how much memory to
// allocate at compile time
// Enums allow vectors to store different types of elements 
// Enum can have variants of different types
// So each vector element can end up being a different variant of the enum
// but since they are all the same enum type, it is allowed to be stored in the
// vector
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}


fn main() {
    // Below in the vector all three elements are acutally of different type
    // for practical purposes, but they are of the same enum type
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue"))
    ];

    //The below print prints out Int(3), Float(10.12), etc
    for i in &row {
        println!("{:?}", i);
    }
    
} 