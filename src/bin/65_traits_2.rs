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

// item shall be of any type which implements the Summary trait. So in our 
// case it can be Tweet type or NewsArticle type defined in lib.rs
// So you cannot call this function using an i32 for example because that would not
// have implemented the Summary trait
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}