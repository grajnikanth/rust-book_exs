// Listing 8-7 - Ownership of data from Vectors
// Below code won't run as explained below
fn main() {
    // mutable v
    let mut v = vec![1, 2, 3, 4, 5];

    // first is an immutable variable
    // It will now have the memory address to v[0] element
    // Now we cannot push data to v anymore as it may change location of
    // v and inturn location of v[0]. If that happens, first will no longer
    // point to v[0]. So Rust will not allow this to happen
    let first = &v[0]; // immutable borrow occurs here

    // Trying to add data to "v" may need to relocate the memory location
    // of "v", hence causing issues with address stored in "first" variable
    // above. So below not allowed
    v.push(6); // mutable borrow here
    println!("The first element is: {}", first);

}
