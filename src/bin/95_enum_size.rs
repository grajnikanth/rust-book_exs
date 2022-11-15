// 95_enum_size.rs

use std::mem;
pub enum AccountTag {
    Uninitialized,
    Mint,
    TokenAccount,
}

pub struct Mint {
    pub tag: AccountTag,
    pub supply: u64,
}

fn main() {
    println!("Size of enum is : {:?}", mem::size_of::<AccountTag>());
    println!("Size of Mint is : {:?}", mem::size_of::<Mint>());
}


