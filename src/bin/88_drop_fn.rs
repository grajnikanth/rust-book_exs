// In this code we will use the std::mem::drop function to drop a value
// prior to it being automatically dropped by Rust

// note the std::mem::drop is in the prelude meaning we can use it directly
// without needing to import it

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("Variable c"),
    };

    let d = CustomSmartPointer {
        data: String::from("Variable d"),
    };

    println!(" Variables c and d - CustomSmartPointers created");
    drop(c);
    println!("c was dropped before end of scope. Next we will drop d");
    drop(d);
    println!("d was dropped before end of scope")

}