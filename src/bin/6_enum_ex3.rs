//enum example with data in the enum with different types of data for each variant
// below enum variants can take in data as well for each of the instance
// Each instance can store a string data into one of the variants
#[derive(Debug)]
enum IpAddr {
    //V4() is like a constructor function taking arguments
    V4(u8, u8, u8, u8),
    V6(String)
}

// enums can be used as function parameters as shown below
// Looks like ownnership of the enum instance has been passed to the function
fn print_enum(ip_kind: IpAddr) {
    println!("Value = {:#?}", ip_kind);
}

fn main() {
    //"::" namespace syntax is used to access each variant
    //V4() is like  a function
    let four = IpAddr::V4(127, 0, 0, 1);
    let six = IpAddr::V6(String::from("::1"));

    print_enum(four);
    print_enum(six);
    
}

