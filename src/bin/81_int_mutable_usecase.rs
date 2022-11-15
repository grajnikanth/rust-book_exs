// below is a second use case when Cell can be used to solve something we want to
// accomplish when we are dealing with immutable references

// say we have a trait know as AwesomeTrait someone else implemented with 
// lot of good methods you want to use on your struct

// Most of the functions are implemented in the trait but the return_number
// function was not implemented and we want to implement it for our case

trait AwesomeTrait: Clone {
    // fn cool_thing(&self);

    // fn cool_change(&mut self);

    // fn give_number(&self) -> i32;

    fn return_number(&self) -> u32;

}

#[derive(Clone, Debug)]
struct Book {
    name: String,
    number: u32,
}

impl AwesomeTrait for Book {

    // We implemented the return_number function but we want to mutate self.number
    // But the fn signature as defined by the trait is that, we cannot mutate the
    // self. As we are only allowed to get immutable self
    fn return_number(&mut self) -> u32 {
        self.number += 10;
        self.number
    }
}

fn main() {

}