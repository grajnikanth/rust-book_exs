// Syntax where we use generics with function signature so that function can
// work with multiple different types of data types

// integer, string, char data types are called concrete data types in Rust

//First declare the function signature with Generic type after the function
// name. This tells the compiler the type which will be used inside the function
// so this syntax <T> is placed after the name of the function and before the ()

// Also "T" is the parameter used to specify the generic type. Use of "T" in Rust just the
// convention 

// We read this definition as: the function largest is generic over some type T. 
// This function has one parameter named list, which is a slice of values of 
// type T. The largest function will return a value of the same type T.

fn largest<T>(list: &[T]) -> T {

    let mut largest = list[0];

    
    for &item in list {
       
        // Currently this code fails to compile because compiler does not know
        // if the "list" sent to this function is of type which can do comparison
        // like that done below

        // For this comparison to work, we can only use "T" types which implemented
        // the trait type std::cmp::PartialOrd. This is done for types whose values
        // can be ordered for example integers and characters

        // We have to restrict our Type "T" before this code can compile
        if item > largest {
            largest = item;
        }
    }

    // return largest at the end of the loop
    largest

}


fn main() {
    let list1 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&list1);
    println!("The largest number in list1 is {}", result);

    let list2 = vec!['y', 'm', 'a', 'q'];
    let result = largest(&list2);

    println!("The largest character in list2 is {}", result);
}
