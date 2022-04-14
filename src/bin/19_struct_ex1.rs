//basic struct example - Srtuct is like an object in other languages with key
//value pairs
struct User {
    email: String,
    username: String,
    active: bool
}

fn main() {
    //Define an instance of the User struct
    let mut user1 = User {
        email: String::from("test1@test.com"),
        username: String::from("test1"),
        active: true
    };

    println!("user1.email = {}", user1.email);
    println!("user1.username = {}", user1.username);

    //updating one field in the user1 struct
    user1.email = String::from("test2@test.com");
    println!("Updated user1.email is = {}", user1.email);

    let user2 = build_user(String::from("test2@test.com"), String::from("test3"));
    println!("user2.email = {}", user2.email);
    println!("user2.username = {}", user2.username);

    //creating a new user using existing struct instance values
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("test3@test.com"),
    };

    //print user3 information
    println!("user3.username = {}", user3.username);

    //Is user1.username still valid or was it moved to user3
    // below println will not work because the user1.username is moved to user3.username
    // println!("user1.username = {}", user1.username);

    //update syntax
    // move fields from other struct (user2) but update only few fields
    // .. is like the spread operator
    let user4 = User {
        email: String::from("test4@test.com"),
        ..user2
    };

    //print user4 information
    println!("User4 struct information");
    println!("user4.username = {}", user4.username);
    println!("user4.active = {}", user4.active);
    println!("user4.email = {}", user4.email);

    //print user2 information - the fields which do not have copy traits implemented
    // will be moved to user4. user2.active implements copy trait so that property stays 
    // with user2
    //println!("user2.username = {}", user2.username); //user2.username has moved to user4
    println!("user2.active = {}", user2.active);






}

// function retuns struct of type user
fn build_user(email: String, username: String) -> User {
    User {
        //using the field init shorthand notation
        email,
        username,
        active: false
    }
}