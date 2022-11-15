// Example for use of Rc<T> smart pointer where one item can be owned by multiple
// variables. This behaviou is not allowed typically but with Rc which keeps
// how many owners it has, this is allowed

// we will use the recursive list to show this example

use std::rc::Rc;
#[derive(Debug)]
enum List {
    // with Rc<List> multiple owners can own an instance of this list
    Bans(i32, Rc<List>),
    Nil,
}

// use crate::List::{Bans, Nil};
fn main() {

    // when declaring a, we need to specify that this is of type List
    // for Rust to know how this is being created
    // a is Bans(5), then Bans(10) and then Nil
    // a = Bans(5, 10, Nil) approximately

    // Note a, b, c are actually Rc<List> type. They are not List type
    // In original example with Box, the variables were actually of the type
    // List
    let a  = Rc::new(List::Bans(5,Rc::new(List::Bans(10,Rc::new(List::Nil)))));
    let b = Rc::new(List::Bans(3, Rc::clone(&a)));
    let c = Rc::new(List::Bans(4, Rc::clone(&a)));

    println!("a Rc<List >is {:?}", a);
    println!("b Rc<List >is {:?}", b);
    println!("c Rc<List >is {:?}", c);

}