// rust_book_exs -> 92_From_and_into.rs

// example to show how From trait is implemented to convert one type of data to other
// in the below example, I will be converting a i32 to a Number struct
// This is possible if  you implement the From<i32> trait on Number as is done below

#[derive(Debug)]
struct Number {
    value: i32,
}

// below we are implementing the From<T> trait on Number struct
// that is we will implemnt a function which will take "T" type and return us a
// Number struct. 
// So with this From trair implemented we can create a Number struct from a i32
impl From<i32> for Number { 
    fn from(value: i32) -> Self {
        Self {value}
    }
}


fn main() {

    let num: i32 = 25;
    let num_struct = Number::from(num);

    println!("Number struct from num is {:?}", num_struct);

    let num_2: i32 = 35;
    let num_struct_2: Number = num_2.into();

    println!("Number using num.into is {:?}", num_struct_2);

}