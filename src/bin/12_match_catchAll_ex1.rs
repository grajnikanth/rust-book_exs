//catch all pattern and _ placeholder in match
// See listing below 6-5 in book for information on use case for below

fn main() {
    let dice_roll = 9;

    match dice_roll {
        // It is syntactically possible to place a function to the right of the => operator
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        //other is a Rust keyword to capture all the rest of cases for dice_roll not captured
        // by above two patterns
        //other also gets binded to the value of dice_roll and hence can be used as
        // an argument to a function as shown below
        other => move_player(other)
    }

}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}