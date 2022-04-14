//Basic enum example
// enums can hold data as well
// enum has variants as it's field
// any instance of a enum will be of one of the type of variant. cannot be both
//example - use enum to represent IPV4 address and IPV6 address which do not overlap and
// are uniquely different from each other.

//Note the syntax, V4 and V6 are two variants
// instances of IpAddrKind can take on one of these two variants
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

// enums can be used as function parameters as shown below
// Looks like ownnership of the enum instance has been passed to the function
fn print_enum(ip_kind: IpAddrKind) {
    println!("Value = {:#?}", ip_kind);
}

fn main() {
    //"::" namespace syntax is used to access each variant
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    print_enum(four);
    print_enum(six);
    
}

