// 94_try_From.rs

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
struct EvenNumber(i32);

// type Error alias set to String. So we are going to return String as Err() for result
impl TryFrom<i32> for EvenNumber {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            return Ok(EvenNumber(value));
        } else {
            Err(String::from("The value i32 is not an even number cannot convert"))
        }

    }
}

fn main() {

    let num = 20;

    // conver num to struct EvenNumber
    let even_number = EvenNumber::try_from(num).unwrap();

    println!("num converted to EvenNumber is {:?}", even_number);

    // try_into syntax. try_into automatically provided by Rust when try_from was
    // defined
    let num2 = 22;
    let even_num_2: EvenNumber = num2.try_into().unwrap();
    println!("num2 converted to EvenNumber is {:?}", even_num_2);

    // Check when we try to convert odd i32 to EvenNumber
    let num_3 = 21;
    // let even_num_3 = EvenNumber::try_from(num_3);
    let even_num_3: Result<EvenNumber, String> = num_3.try_into();
    match even_num_3 {
        Ok(value) => println!("num_3 converted to EvenNumber {:?}", value),
        Err(e) => println!("{}", e)
    }

    

}