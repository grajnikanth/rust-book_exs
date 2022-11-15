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

    // I changed the 3u8 to 25 to make this example simpler to follow
    // 
    let config_max = Some(25);

    // if let syntax : pattern is to the left of "=". 
    // to the right of "=" is the variable we want to match
    // the data in config_max variable is binded to max in the example below

    // config_max is Some(25) so below syntax is going to take config_max which is Some(25) 
    // So on the left hand side of "=" we are comparing that to Some() pattern. Which in this case
    // it matches. So now since it matches, Rust takes the value 25 of the Some(25) and binds that 25 to 
    // the variable max.
    
    // So now the below will print that the value of max which is 25 to the terminal
    // Else with if let will take care of of rest of the exhaustive types config_max can 
    // take. But below with only if let we are saying that we do not want to do an exhaustive 
    // match on all possible values of config_max. 
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    //if let synyax is useful if you want to only check for the specific pattern
    // with this you lose the exhaustive nature of the match syntax
}