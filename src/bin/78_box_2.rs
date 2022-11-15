// Cons is a recursive type
// Rust does not know how much memory to allocate if Cons(i32, List) is used below
// instead of Cons(i32, Box<List>)
// This is because with Box it only has to allocate enough memory to hold i32
// and pointer data. So it is not infinite memory.

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("list = {:?}", list);
}