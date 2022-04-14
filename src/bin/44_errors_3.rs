// enum Result

// enum result can be used to handle errors in the code
// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }
// T and E are generic type parameters.
// T is the type of value returned in case of success in Ok variant
// E - error in case of failure

// Opening a file using File::open function
// File::open() function returns the enum Result

// In this case if success, we get back std::fs::File which is a file handle with 
// Ok variant
// In case of error opening the file we get Err() variant with std::io::Error type 
// as E

use std::fs::File;

fn main() {
    // open() function called on the file hello.txt
    // this file does not exist
    let f = File::open("hello.txt");

    // Result enum is brought into scope automatically we do not need to import it
    // So it's variants Ok and Err are accessible in this scope as shown below
    let res = match f {
        // file variable will bind to the result  we got from the varibale f
        // this block of code, will return the value "file" as shown below
        // this will be assigned to variable "res"
        Ok(file) => file,
        Err(error) => panic!("Error gotten when opening file is: {:?}", error)
    };

}


