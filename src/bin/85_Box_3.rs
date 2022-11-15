// Typical how references work and how dereferecing works to get to the value pointed
// by the reference pointer

// Compare that with Box<T> which implements Deref and Drop trait
// These traits make Box<T> behave like typical reference type

fn main() {
    let x = 5;
    // y holds the pointer which points to the memory location of x
    // So y is the reference type variable
    let y = &x;

    assert_eq!(5, x);
    // *y dereferences the value pointed by y. That is it retrieves the value 
    // y reference is pointing to
    assert_eq!(5, *y);

    // Box implements dereference trait so we can use syntax we use with references
    // to derefernce Box<T>

    let j = 5;
    // Since j is an i32 and implements Copy trait, the below new() function
    // will make a copy of j when we pass this to the function based on Rust 
    // copy borrow rules
    let k = Box::new(j);

    // we can dereference k using *k. But note that it is pointing to 5 i32 which
    // is a copy of j. It is not pointing to j directly
    assert_eq!(5, j);
    assert_eq!(5, *k);

}