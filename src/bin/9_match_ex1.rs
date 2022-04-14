//Example of how match works
enum Coin {
    Penny,
    Nickel,
    Dime, 
    Quarter
}

// match information
// => is called an operator
// code to the right of => operator is the expression
// In this case below, the match is returning u8 and then automatically that is
// being returned by the outside function. So the result of match is the return value
// of the function value_in_cents 
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}

fn main() {
    let penny1 = Coin::Penny;
    println!("Value in cents of penny1 = {}", value_in_cents(penny1));
}