//match example with more detailed expression code block
// match with data stored in an enum and accessing that data in the match expression
// pay attention to the function type syntax in the Coin::Quarter(state) below
// See listing 6-4 in the book for explanation of the reasoning for using the below enums

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime, 
    Quarter(UsState) //this variant can store UsState enum as data for it's instances
}

// Example with more detailed code under the match block after the 
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // The state variable in the below pattern is almost like an argument in a function
        //  the code block to the right of => operator is able to access this state variable
        // So the code block may have access to other data also, let's check
        // below the state takes the value of the data inside the coin::Quater
        Coin::Quarter(state) => {
            println!("The state of the instance is = {:?}",state);
            // The below code does not work, the variable coin is not fully accessible here
            // The rust error about partial borrow is displayed. Will figure that out later
            // println!("The coin sent to this function is = {:#?}", coin);
            25
        }
    }
}

fn main() {
    //UsState::Alaska is the enum variant data I want this quarter1 instance to have 
    // as it's data
    let quarter1 = Coin::Quarter(UsState::Alaska);
    println!("Value in cents of Quarter = {}", value_in_cents(quarter1));
}