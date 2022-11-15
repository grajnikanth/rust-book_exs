// Code with usage of "Cell" to make part of a struct mutable even though
// the whole struct is not mutable
// This solves the error/limitation we saw in the code 79_int_mutability.rs

use std::cell::Cell;

#[derive(Debug)]
struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    // We define that the field on_sale is of the type Cell which is a struct
    // but with unique properties of mutability
    on_sale: Cell<bool>,
}

#[derive(Debug)]
struct PhoneModelNoCell {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    // We define that the field on_sale is of the type Cell which is a struct
    // but with unique properties of mutability
    on_sale: bool,
}

#[derive(Debug)]
struct Outer {
    name: String,
    inner_struct: Inner,
}

#[derive(Debug)]
struct Inner {
    name: String,
    id: u32
}

fn main() {

    // model_3000 is an instance of PhoneModel and below we are defining it
    // as immutable because most of our fields are immutable. Our program
    // design was such that this data structure should contain values which 
    // will be most typically be immutable except for one field which we want to
    // be mutable
    
    let model_3000 = PhoneModel {
        company_name: "QQQ".to_string(),
        model_name: "model3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_0000,
        date_issued: 2020,
        on_sale: Cell::new(true),
    };

    let model_3000_no_cell = PhoneModelNoCell {
        company_name: "QQQ".to_string(),
        model_name: "model3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_0000,
        date_issued: 2020,
        on_sale: true,
    };

    // Eventhough, model_3000 is immutable, on_sale value can be changed because
    // it is of type Cell which implements unsafe code with gurarantees at runtime
    // 
    model_3000.on_sale.set(false);
    dbg!(&model_3000);
    model_3000.on_sale.set(true);
    dbg!(&model_3000);

    // get() function on the std::cell::Cell returns a Copy of the variable 
    // So below code is ok as we are just getting copies of the data stored
    // at on_sale variable
    // And because it is cell, we can also change it even if the model_3000 
    // is declared to be immutable
    let mut ref1 = model_3000.on_sale.get(); //makes a copy of the bool
    println!("Ref1 borrowed from model_3000 {}", ref1);

    let mut ref2 = model_3000.on_sale.get();
    println!("Ref1 borrowed from model_3000 {}", ref2);

    let mut ref5 = model_3000.on_sale.get(); 
    let mut ref6 = model_3000.on_sale.get();
    println!("Ref1 borrowed from model_3000 {}", ref5);



    let mut ref3 = model_3000_no_cell.on_sale; // makes a copy 
    let mut ref4 = model_3000_no_cell.on_sale;
    println!("Ref1 borrowed from model_3000 {}", ref4);

    // model_3000_no_cell.on_sale = false;

    let inner1 = Inner {
        name: "inner1".to_string(),
        id: 1,
    };

    let outer1 = Outer {
        name: "outer1".to_string(),
        inner_struct: inner1,
    };

    //dbg!(inner1);
    dbg!(outer1);

}