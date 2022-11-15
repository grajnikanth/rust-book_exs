// Below is a struct which has lot of fields. All of the fields except the "on_sale"
// field have to be immutable once defined initially.
struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: bool,
}

fn main() {

    // model_3000 is an instance of PhoneModel and below we are defining it
    // as immutable because most of our fields are immutable.
    
    let model_3000 = PhoneModel {
        company_name: "QQQ".to_string(),
        model_name: "model3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_0000,
        date_issued: 2020,
        on_sale: true
    };

    // But we want model_3000.on_sale value to change and we cannot change
    // it with the code above because model_3000 is defined as an immutable 
    // variable

    // With the below line we get an error saying model_3000 is not mutable
    // hence we cannot change on_sale field. This is where the interior mutability
    // can be used to solve our problem
    model_3000.on_sale = false;

    // in the next piece of code we will solve the above issue using Cells
    


}