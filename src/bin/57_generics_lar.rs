
// Fixing the issues with largest function from the file 57_generics_2 using 
// Trait bounds

// Since we need to use the greater than operator in this function on the generic 
// type items, we need to make sure that the generic type sent to this function
// implements the standard trait called the "PartialOrd". Only types which implement
// this trait can use the ">" "compare" operator

// So using trait bounds on T we retrict T to PartialOrd trait

// We also are taking list[0] and list[i] and copying it to larget variable
// Since T is generic compiler does not know if T implemented Copy trait so the 
// code below will not compile unless we tell the compiler that we are only going
// to send types which implement Copy by using Trait bounds

// That is i32s and chars are types which can be sent to this function as they
// both implement the ParitalOrd and Copy traits
// You can check Rust documentation and see that both these traits were implemented
// on i32s and chars. Now the below code works

// Notice the where syntax used to define the Trait bounds for the T types 

// See book for clone and &T as return value to solve this in a different way.
// I did not fully understand how returning &T would solve the Copy issue. But
// investigate this if needed later

fn largest<T>(list: &[T]) -> T 
    where T: PartialOrd + Copy,
{

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
