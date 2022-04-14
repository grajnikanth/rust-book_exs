//use of dbg! macro to print expressions anywhere in the code
// appears to be an awesome tool to debug code by checking the values in the code anywhere
// debug - dgb! macro is a Trait

// Below directive allows the println! to work with structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        //dbg! macro takes ownership and gives it back once it prints to stderror stream
        // this debug even prints text of the expression to terminal and code line number
        //- GREAT!
        width: dbg!(30*scale),
        height: 50
    };

    //below we used reference to rect1 because we are not assigning to any
    //variable to take back ownership of rect1 from dbg!. So just passing
    // reference to keep ownership of rect1 in main
    dbg!(&rect1);

}