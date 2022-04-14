//match example with more detailed expression code block

enum Coin {
    Penny,
    Nickel,
    Dime, 
    Quarter
}

// Example with more detailed code under the match block after the 
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}

fn main() {
    let penny1 = Coin::Penny;
    println!("Value in cents of penny1 = {}", value_in_cents(penny1));
}