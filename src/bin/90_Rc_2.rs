// See code in 89_Rc_1.rs for original code for the below
//Example to check on the counter of Rc<List>

use std::rc::Rc;
#[derive(Debug)]
enum List {
    // with Rc<List> multiple owners can own an instance of this list
    Bans(i32, Rc<List>),
    Nil,
}

// use crate::List::{Bans, Nil};
fn main() {

    let a  = Rc::new(List::Bans(5,Rc::new(List::Bans(10,Rc::new(List::Nil)))));
    println!("RC Count of a after creating a = {}", Rc::strong_count(&a));
    let b = Rc::new(List::Bans(3, Rc::clone(&a)));
    println!("RC Count of a after creating b = {}", Rc::strong_count(&a));
    {
        let c = Rc::new(List::Bans(4, Rc::clone(&a)));
        println!("RC Count of a after creating c = {}", Rc::strong_count(&a));
    } // c will be out of scope here and hence c will be dropped
    println!("RC Count of a after dropping c = {}", Rc::strong_count(&a));

}