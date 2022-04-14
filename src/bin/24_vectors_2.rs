// Vectors continued
// Accessing elements not present in a vector
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    // Program crashes at the line below because there is nothing in memory
    // at &v[100] 
    // Index out of bounds
    let does_not_exist = &v[100]; 

}