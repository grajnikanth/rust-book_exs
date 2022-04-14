// Rust groups errors into two major categories: recoverable and unrecoverable errors.

// Result<T, E> - Type which is for recoverable errors
// panic! macro for unrecoverable errors
// Most language use "Exceptions" for errors, not Rust 

// calling panic! macro will abort the program
// Typically panic! macro is called by Rust if it encounters a bug in the code
// Backtrace can be used to see which function is causing the panic call 
fn main() {
    panic!("Throw an error");
}
