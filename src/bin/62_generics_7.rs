// Functions on specific instances of structs
// Below we can implement a method which only works with a Point<f32>. That is instance
// which only have f32 data in them. So this funciton will not work with instance
// which have Point<i32>

struct Point<T> {
    x: T,
    y: T
}

// No generics defined here on impl so the functions work only on Point<f32> types
impl Point<f32> {

    fn distance_from_origin(&self) -> f32 {
        // some math functions powi(2) which only work on floats
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p1 = Point {x: 10.0, y: 10.0};
    println!("Distance of P1 from origin is = {}", p1.distance_from_origin());

    
    let integer_p2 = Point {x: 10, y: 10};
    // Since integer_p2 is Point<i32>, below will give an error because
    // the function distance_from_origin does not exist on Point<{integer}>
    println!("Distance of p2 from origin is = {}", integer_p2.distance_from_origin());
}