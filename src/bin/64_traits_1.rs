// The Summary trait and Tweet struct were defined in our lib.rs files of this crate
// The crate is the name of the folder - rust_book_exs
// Now we can bring the Summary and Tweet into this files scope
// See lib.rs for details of implementation of Summary and Tweet
use rust_book_exs::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, people"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());

}