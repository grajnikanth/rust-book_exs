//Chapter 15 Smart Pointers Box<>
// smart pointers are pointers which have additional capabilities and metadata
// A typical pointer like reference only has a reference
// String and Vec<> are smart pointers

// Box<T> is used to store data on Heap instead of on stack. So that it can be refereced
// by other variables without doing expensive copy operation

// Example to store i32 on heap. Typically i32 is stored on stack by default

fn main() {
    // 5 is an i32 type value but now will be stored in heap
    let b = Box::new(5);

    // data stored on heap using Box can retrieved similar to that stored on stack
    // by simply using the variable directly
    println!("b Box Type = {}", b);
} // b is de-allocated when b goes out of scope


