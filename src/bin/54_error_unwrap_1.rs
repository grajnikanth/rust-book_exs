// When to use unwrap and expect vs when to handle errors using Result
// When you are doing testing and example code, panicking when error occurs would
// make sense. Once prototyping the code is done, then you can do error handling
// with Result, match etc

// Also if you know the code will not error out, you can use unwrap instead
// of formally handling Result. Below is an example of this

// But typically by default you woult want to return Result without panicking
// so that the function calling your function can make the call on how to 
// handle error rather than you deciding for the function caller
// panic would not give the caller a chance to handle errors

use std::net::IpAddr;

fn main() {
    // here in the code we know for sure that "127.0.0.1" will never error 
    // out because this is a valid IP. but compiler does not know what we humans
    // know. parse() will always return Result type. So here using unwrap()
    // makes sense because we know this code below with hard coded IP will never 
    // panic

    // but let's say the IP address is sent as a variable, then unwrap would
    // not make sense to use because if the variable is not valid, the code will
    // panic
    let home: IpAddr = "127.0.0.1".parse().unwrap();
}
