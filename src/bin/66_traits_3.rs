// Trait Bound Syntax 
// More generic syntax in function arguments as opposed ti &impl Summary syntax
// in 65_traits_2.rs file

use rust_book_exs::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, people"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);

}

// Trait Bound Syntax
// The below syntax with generics is the actual syntax
// &impl Summary in previous file is just synctactic sugar
// We are specifying that T is of type Summary that this any type that implements
// Summary trait
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// with the trait you could have the following syntax commented out
// In the syntax below item1 and item2 can two different types of Summary trait
// implemented data types. But if you wanted item1 and item2 to of the
// same Summary type data, then you have to use Trait Bound syntax as shown
// below
// fn notify(item1: &impl Summary, item2: &impl Summary) {}


// trait bound syntax with item1 and item2 to be of the same type
// With this item1 and item2 will be of the same type of instance of data 
// which implemented Summary traits
// fn notify<T: Summary>(item1: &T, item2: &T) {}

// Specifying multiple Trait Bounds with + syntax
// In the code below, Summary and Display are the two traits we want T to implement
// to use as an arguments in this function

// pub fn notify<T: Summary + Display>(item: &T) {}

// Trait bounds can be specified using the where clause instead of putting
// them in the <> buttons. This is handy when we have multiple generic types
// below two syntax are equivalent 

// Mutliple Trait Bounds defined in signature of function
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// Trait bounds using the where syntax
// The below syntax is equivalent to the above
// fn some_function<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {}
