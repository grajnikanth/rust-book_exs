use std::cell::RefCell;

#[derive(Debug)]
struct User {
    id: u32,
    username: String,
    active: RefCell<bool>,
}

fn main() {
    let user_1 = User {
        id: 1,
        username: "user1".to_string(),
        active: RefCell::new(true),
    };

    let mut reference1 = user_1.active.borrow_mut();
    println!("reference is {:?}", reference1);
    //let mut reference2 = user_1.active.borrow_mut();

    // Change the value at the pointer reference1 to false. This should be
    // pointing to the user_1.active field. But note that the user_1.active
    // shows that its value has been borrowed out and it does not show the 
    // value as true or false

    *reference1 = false;
    dbg!(&user_1);
    println!("reference is {:?}", reference1);
    println!("{:?}", user_1.active);

    std::mem::drop(reference1);
    dbg!(&user_1);

    // need not use the reference1 variable as intermediate variable. The 
    // active field can be updated directly using the borrow_mut and derefing it
    // and putting the new value in it
    *user_1.active.borrow_mut() = true;
    *user_1.active.borrow_mut() = false;
    dbg!(&user_1);

}