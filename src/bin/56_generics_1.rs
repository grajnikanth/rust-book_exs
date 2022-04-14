// Two functions one to find the largest integer in a vector and one to find
// largest character in a character vector

// See 55_function_dupl_1 for more notes removed in this code

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    // return largest at the end of the loop
    largest
}

// Note the code in the below function is exactly the same as the code in the 
// function above where we defined the largest_i32() function
// So these two functions are perfect use case to use generic arguments in the function
// signature so that the same function can be used to find the largest i32 and largest
// char from vector list

fn largest_char(list: &[char]) -> char {

    let mut largest = list[0];

    
    for &item in list {
        // looks like in Rust code when two characters are compared, their
        // ascii value is probably compared to determine which is bigger 
        // than the other
        if item > largest {
            largest = item;
        }
    }

    // return largest at the end of the loop
    largest

}


fn main() {
    let list1 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_i32(&list1);
    println!("The largest number in list1 is {}", result);

    let list2 = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&list2);

    println!("The largest character in list2 is {}", result);
}
