// Iterating over str slices
// chars() method and bytes() method
// chars() method retrieves the appropriate bytes for each character
// based on the UTF-8 encoded requirements to show the user the
// exact string
// bytes() retirieves each of the UTF-8 encoded number at each byte

fn main() {
    
    // chars() will give you all the letters used to represent the hindi string
    // below inlcuding the "aye" and "stha" sounds
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // bytes() will provide UTF-8 encoded number at each byte -
    // total of 18 bytes to represent the hindi word namaste
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // the chars() and bytes() function work on str slices
    let s = "नमस्ते";
    for c in s.chars() {
        println!("{}", c);
    }

    // the chars() and bytes() functions work on Strings as well
    // May be Rust coerces Strings to str slices with these function internally
    // look into this out if needed in future
    let s1 = String::from("नमस्ते");
    for c in s1.chars() {
        println!("{}", c);
    }

    for b in s1.bytes() {
        println!("{}", b);
    }
}