// Use of Backtrace
// Backtrace is used to identify where in your code the bug occurs
// A backtrace is a list of all the functions that have been called 
// to get to this point.
// buffer overread security vulnerability in "C" language was explained

fn main() {
    let v = vec![1, 2, 3];
    v[99];
}