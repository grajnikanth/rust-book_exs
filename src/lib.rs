// Implementing a Trait on a Type

// trait is like interfaces in other languages like solidity where we define the
// signature of a function and those functions are implemented by individual
// code
// Below is the syntax for defining trait called Summary with summarize method
// You can have multiple methods in the Summary trait with signature defined
// without implementation details here in trait definition
// signatures end with a ";"
// The trait groups common functionality

pub trait Summary {
    fn summarize(&self) -> String;
}



pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

// Here the NewsArticle Struct is implementing the trait Summary and the summarize
// method of that trait

// syntax is below. Note the keyword for
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


// Listing 10-16 - Using Trait Bounds to conditionally implement methods
// You can use trait bounds on methods of structs to specify what type of 
// variables are acceptable to send to a function of a struct. 

// Below compare_display function will accept Pair struct instances which implement
// Display and PartialOrd. This is done using the Trait bound syntax

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    // The compare_display function here will work only if the generic T 
    // implements the ParitalOrd and Display traits as defined by Trait bounds
    fn compare_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

// Conditionally implement a Trait for any type that implements another Trait

// Below the ToString trait is implemented on an generic type which also implements
// the Display Trait
// This is from standard library

// We are syaing that T which implements Display trait we will implement the
// ToString trait for such types

// These are called Blanket Implementations
// impl<T: Display> ToString for T {
//      fn to_string() {}
// }


// TRAITS REDUCE DUPLICATION 
// TELLS COMPILER THE BEHAVIOR WE WANT THE GENERIC TYPES TO HAVE
// THIS IS DONE IN RUST AT COMPILE TIME SO ALL ERRORS AT RUNTIME ARE AVAOIDED
// TYPICALLY IN DYNAMICALLY TYPED LANGUAGES THESE ERRORS SHOW AT RUNTIME
// THIS MAKES RUST'S PERFORMANCE BETTER

