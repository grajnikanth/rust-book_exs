//Option<T> enum of standard library
//Option enum variants - None and Some(T) are available in the scope of all Rust code
// Documentation shows all the methods which were implemented for the Option enum
// Option variant - None is present to deal with Null references in Rust code
// default variables in Rust cannot be Null. If you want to use Null, use Option enum
// and have code for the case when an value takes on this "None" variant

fn main() {
    //calling on the Some(T) of Options enum in this scope directly w/o using Option:: namespace
    
    let some_number = Some(5); //T in this case is i32 so this is Option<i32>::Some(5)
    let some_string = Some("a string"); //Option<&str>::Some
    let absent_number: Option<i32> = None; //Have to declare type for compiler to understand what type this Option will be even though none
    
    println!("some_number = {:#?}", some_number);
    println!("some_string = {:#?}", some_string);
    println!("absent_number = {:#?}", absent_number);
}