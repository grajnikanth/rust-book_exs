// Below is an example of how functions can be written which can eliminate duplicate 
// code

// also below is the syntax on how vectors can be passed as function arguments
// how slices of i32 can be used in a function argument and it's syntax.

// &[i32] is the syntax for defining list as a slice of integers
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    // note syntax for "for" loop where the &item is used to represent a reference
    // to an element in the slice of i32 values
    // If this was a regular for loop on a vector which was available to use 
    // in the scope the for loop would be as follows without the &

    //  for item in list {}

    for &item in list {
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

    let list2 = vec![34, 50, 25, 100, 65];
    let result = largest(&list2);

    println!("The largest number in list2 is {}", result);
}

