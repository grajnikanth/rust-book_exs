// Vector collection example
// vector data is stored in heap memory. All vector data is stored together
// All vector elements have to be of the same type


fn main() {
    //v1 has to be declared mutable if we want to push elements to it later
    // otherwise we get an error that we are trying to change a immutable value
    let mut v1: Vec<i32> = Vec::new();

    // defining a vector using vec! macro
    // Type of data is internally figured out by Rust based on data stored
    let v2 = vec![1,2,3]; 

    // inserting data into a Vector using push function 
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    // Option 1 - Reading vector values
    // Syntax to read the values stored in a vector
    // Below we are getting the reference to the v1[2] value and storing
    // it in a immutable variable
    // So now third points to the memory where v1[2] is stored
    // If v1 elements are moved after this, then Rust will not compile
    // v1 elements could be moved to new memory if more elements are pushed
    // into it. And if the current memory where v1 is not sufficient to add
    // new elements
    // If no element exists at the index, then the below will cause the code to abort 
    // So be careful if you assign the memory address of one vector element to a new variable and
    // then that data moves, then the new variable is now pointing to empty data address
    let third = &v1[2];
    println!("The third element of v1 vector is {}", third);

    // Instead of storing v1[2] into a new variable, we can just directly print it
    // as shown below
    println!("&v1[2] is = {}", &v1[2]);

    // vector data can be accessed using the simple array type syntax as shown 
    // below v1[2]
    println!("v1[2] is = {}", v1[2]);

    // Option 2 - reading vector values using get() method
    // get method resturns the standard enum option<&T> type. Where &T represents that
    // the data returned is a reference of type T is returned
    // if no value at the index specified, we get Option type "None"
    match v2.get(2) {
        // if value exists we can bind it to "third" here as shown with type some
        Some(third) => println!("Value obtained from v2.get(2) is {}", third),
        // With the Option<&T> returned we can avoid the program from terminating abruptly
        // we can handle the case when an the index requested does not exist
        None => println!("There is no third element")
    }

}