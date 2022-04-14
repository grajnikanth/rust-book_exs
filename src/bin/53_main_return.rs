// main() function return types 
// main executable exits with 0 if succesful else exits with non zero integer
// main typical returns ()

// We can make main return Result. Main can return thos that impement the
// std::process:Termination trait

// below we are returning from main Result<>

use std::error::Error;
use std::fs::File;

// Box<dyn Error> trait type return any kind of error
fn main() -> Result<(), Box<dyn Error>> {

    // below we are using ? operator to return result
    let f = File::open("hello.txt")?;
    // In the above line does not cause return of error, we can return the 
    // Ok type with () return
    Ok(())
}