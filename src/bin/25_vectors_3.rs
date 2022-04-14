// Vectors continued
// Accessing elements not present in a vector using get()
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    // Program does not crash in the line below even though we tried to access
    // an index not present in the vector
    // Get function returns a None value in this case instead of crashing
    // like what happens in 24_vectors_2.rs
    let does_not_exist = v.get(100); 
    println!("value of v.get(100) {:?}", does_not_exist);
}