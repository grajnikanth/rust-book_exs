// chaining of functions example
// below code returns the last character in the first line of given text
// ? operator can return an Option<> or a Result. So the function you are
// using to return shall be consistent with ? operator return types
// ? will return "None" with Option if error occurs early
// If not it ? will provide the value inside Some() and function continues

fn main() {
    let text1 = "hello\nworld";
    let s = last_char_of_first_line(&text1);
    println!("Lastword of first line in text1 is {:?}", s);
}

// Option<char> as return value because the function might find character at this position
// or might be no character
fn last_char_of_first_line(text: &str) -> Option<char> {
    
    // text is string slice
    // lines() method returns iterator over the lines
    // next() on iterator gives first vlaue from iterator
    // next() will give None if no first line exists hence we can
    // call ? at this point to stop. If not next() will return Some(value) 
    // containing the first line of the text
    // ? extracts the first line as string slice
    // chars() gives an iterator on the string slice
    // last returns the last item of the iterator as an option
    text.lines().next()?.chars().last()
}