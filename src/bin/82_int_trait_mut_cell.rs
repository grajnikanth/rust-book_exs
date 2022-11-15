// See code in 81_int.... for problem statement solved here

use std::cell::Cell;

trait AwesomeTrait: Clone {
    // fn cool_thing(&self);

    // fn cool_change(&mut self);

    // fn give_number(&self) -> i32;

    fn return_number(&self) -> u32;

}

#[derive(Clone, Debug)]
struct Book {
    name: String,
    number: Cell<u32>,
}

impl AwesomeTrait for Book {

    // with cell we can modify 
    fn return_number(&self) -> u32 {
        let new_number = self.number.get() + 10;
        self.number.set(new_number);
        self.number.get()
    }
}

fn main() {

}