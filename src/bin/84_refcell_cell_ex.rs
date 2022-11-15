use std::cell::{Cell, RefCell};

#[derive(Debug)]
struct Book<'a> {
    name: Cell<&'a str>,
    author: RefCell<&'a str>
}
fn main() {
    let my_book = Book {
        name: Cell::new("The Crystal Cave"),
        author: RefCell::new("Mary Stewart"),
    };

    // use the Cell::take function to take the value out of the Cell. Rust will put a default value of the
    // type inside when you take a value out of the Cell. You get the value when you take it from cell
    let name_book = my_book.name.take();
    // name_book will be "The Crystal Cave" string slice
    println!("Name_book value is {}", name_book);

    // The my_book.name will be str slice default which is "" empty string
    println!("The default value in the name Cell is {:?}", my_book.name);

    //practice with Rust default() function for various types
    println!("The bool default value in Rust is {}", bool::default());
    println!("The f32 default value in Rust is {}", f32::default());
    println!("The u8 default value in Rust is {}", u8::default());

    // RefCell and usage of try_borrow_mut() function
    // try_borrow_mut allows you to try to borrow a RefCell value and that returns a Result wrapped
    // value so you can handle the error case which occurs if the value was borrowed already 

    // so let's say you have a long program and you mutably borrowed the my_book.author value
    // in the code some where and then later somewhere else in the program you try to borrow it again
    // This time the program won't crash since we are using try_borrow_mut
    
    // let reference = my_book.author.borrow_mut();

    // r will get the reference to the mutable value if the try_borrow_mut works
    // if not we get an error which we can catch in the else block. If the 
    // code my_book.author.try_borrow_mut works you get the Ok(r). where the Ok is the 
    // first part of the Result returned by this function. If not you go into the else
    // block
    if let Ok(mut r) = my_book.author.try_borrow_mut() {
        *r = "Robert Gordon";
    } else {
        println!("my_book.author has been mutably borrowed already. Cannot borrow again");
    }

    println!("{:?}", my_book);

}