// 7_enum_ex5.rs
// Below code is based on my attempt to understand Rust enum and how
// solana program Deserialization of enums work under the hood using borsh deserialize
// The first step is to undertand enum variants as numbers
use std::mem::size_of_val;

#[derive(Debug)]
enum Drink {
    Water, 
    Soda, 
    Juice
}

fn main() {
    let dri_1 = Drink::Water;
    let dri_2 = Drink::Soda;
    let dri_3 = Drink::Juice;

    // Print each of the variant as u8. 
    // The below should print as 0, 1, 2
    // We are casting enum fields to u8 below to show that internally
    // each field of enum is serialized by a number 
    dbg!(dri_1 as u8);
    dbg!(dri_2 as u8);
    dbg!(dri_3 as u8);

}

