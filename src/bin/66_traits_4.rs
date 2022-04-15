use rust_book_exs::{Summary, Tweet};

// Returning a type which implements a trait instead of returning a concrete type

// Here we are specyfing that this function will return a type which
// implements the Summary trait. Tweet type is being returned and per our
// lib.rs file we know Tweet imlpements Summary
fn returns_summarizable() -> impl Summary {

    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already"),
        reply: false,
        retweet: false
    }

}

// When returning a implemented trait type, we cannot return different types
// of objects which implement the trait. For example, as shown below
// the function may return NewsArticle or may return Tweet. Even though both
// implement Summary trait, because they are not of the same type, we cannot
// return impl Summary for this function

// compiler will give error
fn returns_summarizable2() -> impl summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins"),
            location: String::from("USA"),
            author: String::from("Unkown"),
            conent: String::from("content")
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already"),
            reply: false,
            retweet: false
        }
    }
}

