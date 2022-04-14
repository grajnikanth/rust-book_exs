//Multiple variants with different data types
// Each enum can hold data of any type, integer, struct, enum, string etc
#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32}, // Struct data in this variant
    Write(String),
    ChangeColor(i32, i32, i32) //data holds three integers
}

//enums can have methods like structs do with &self parameter
impl Message {
    fn print_enum(&self) {
        println!("Value = {:#?}", self);
    }
}


fn main() {
    let m1 = Message::Write(String::from("Hello"));

    m1.print_enum();
}