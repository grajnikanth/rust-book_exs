// Cons List with Rc and RefCell. This gives us a way to change the values inside the Cons List 
// while having immutable ownership/references to the outside data

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}

use crate::List::{Cons, Nil};

fn main() {

    // value is a variable of the type Rc<RefCell<i32>>
    // This type definition will allow us to store i32 10 in a RefCell which is stored
    // in a Rc. So value can be owned by many owners
    let value = Rc::new(RefCell::new(10));
    // below we take value which is a Rc<RefCell<i32>> and Rc clone it as the first element
    // of Cons() list and the second element is a Nil which is also inserted into Rc before 
    // putting in the Cons. This is ok because we defined our Cons to have two elements
    // first is a Rc<RefCell<i32>> which value is of that type. Second element is a Rc<List>. Rc::new(Nil) is
    // an enum List because 
    // a's element references value
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    // b is going to be Cons(2,&a). Where a is a Rc. So one of b's list owns a 
    let b = Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)));
    let c = Rc::new(Cons(Rc::new(RefCell::new(4)), Rc::clone(&a)));

    // value is updated by doing a borrow_mut of 10 and increment it to 20. 
    *value.borrow_mut() += 10;

    println!("value = {:?}", value);
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
    
    
}