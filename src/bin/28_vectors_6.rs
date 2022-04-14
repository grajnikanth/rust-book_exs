// For loop iterator mutating the vector values
// Use of dereference operator to modify vector values

fn main() {
    let mut v = vec![20, 30, 50];

    // Pay attention to the &mut v syntax to reference to the vector mutable 
    // reference
    for i in &mut v {
        //* dereferences the value pointed by i reference
        // That is we can edit that value using this
        // we are derefencing and then adding 50 to it below
        *i += 50;
        println!("Value of i is {}",i);
    }
}