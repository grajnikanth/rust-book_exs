// if let and else syntax
// the else in "if let" can be used to capture the rest of the cases not matching the 
// begining pattern. It's like using "_" in the match syntax

// Below is the match syntax which can be converted to "if let else"

//          let mut count = 0;
//          match coin {
//              Coin::Quarter(state) => println!("State quarter from {:?}!", state),
//              _ => count += 1,
//          }

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

fn main() {
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("state from the quarter is {:?}", state);
    } else { //else here for "if let" takes care of all other cases when coin is not a quarter
        count += 1;
    }

}