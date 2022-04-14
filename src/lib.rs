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


