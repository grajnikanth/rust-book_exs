//if let syntax to lessen match type of code
// below is an example of match code which we can simplify using if let syntax
//          let config_max = Some(3u8);
//          match config_max {
//              Some(max) => println!("The maximum is configured to be {}", max),
//              _ => (),
//          }

// The above is code which we want to execute only if config_max has some value 

fn main() {
    // Haven't seen the 3u8 syntax before but must be something which describes what
    // 3 is about that is we do not want Rust to infere i32 by default instead we 
    // want it to be u8 in this case
    let config_max = Some(3u8);

    // if let syntax : pattern is to the left of "=". 
    // to the right of "=" is the variable we want to match
    // the data in config_max variable is binded to max in the example below

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    //if let synyax is useful if you want to only check for the specific pattern
    // with this you lose the exhaustive nature of the match syntax
}