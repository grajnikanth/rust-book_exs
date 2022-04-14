// Strings are implemented as collections of bytes
// bytes interpreted as text
// str (String slice) is the only String type in Rust core
// string slices are references to some UTF-8 encoded string data stored somewhere else
// String literals are also string slices stored in program's binary
// String type is not in Rust core but implemented as a library
// String type is growable, mutable, owned, UTF-8 encoded
// Strings and str (string slices) are UTF-8 encoded
// String - owned, str - borrowed

// Rust standard library other string types are OsString, OsStr, CString, Cstr

// Creating new String
// operations are similar to that of Vec<T>
fn main() {
    // new() function to create new String like that of Vec::new()
    let mut s = String::new();

    // Create new string using to_string method
    // Used when you already have a data to store as string, like string literals
    let data = "initial contents";
    let s1 = data.to_string();

    // Create string using String::from() method by passing a string literal
    let s2 = String::from("Second comment");

    // Strings are UTF-8 encoded so we can do the following
    let hello1 = String::from("السلام عليكم");
    let hello2 = String::from("Dobrý den");
    let hello3 = String::from("Hello");
    let hello4 = String::from("שָׁלוֹם");
    let hello5 = String::from("नमस्ते");
    let hello6 = String::from("こんにちは");
    let hello7 = String::from("안녕하세요");
    let hello8 = String::from("你好");
    let hello9 = String::from("Olá");
    let hello11 = String::from("Здравствуйте");
    let hello12 = String::from("Hola");

    println!("hello1 is {}", hello1);
    println!("hello5 is {}", hello5);
    println!("s is {}", s);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

}