// Structs typicall hold owned data
// But structs can be made to hold reference data by using lifetime annotation

// The lifetime has to be ensure that the fields of structs holding reference data
// cannot outlive the reference it holds

// syntax is similar to generic definition with 'a in the <>

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    
    // novel variable holds a String
    // Strings are stored in heap memory
    let novel = String::from("Call me Ishmael. Some years ago...");

    // first_sentence will hold a string slice which comes from the novel 
    // variable above. For example, we will isolate the words "Some years ..."
    // from the novel string and store the reference to this portion of the string slice
    // in the variable first_sentence. That string slice will be assigned to the
    // struct field

    // And since the lifetime of novel > part field, using lifetimes, we can store
    // a string slice in the struct field

    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    // first_sentence string slice is assigned to part field
    let instance = ImportantExcerpt {
        part: first_sentence,
    };
}