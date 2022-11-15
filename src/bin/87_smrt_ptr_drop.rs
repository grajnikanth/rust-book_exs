// custom drop trait implemented to run code before a certain variable is dropped

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

}