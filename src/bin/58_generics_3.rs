// Generics in the definition of a struct
// one or more fields in a struct can be of the type generic as shown below

// syntax is similar to functions the <T> appears after the name of the struct
// Below both x and y are defined to be of Generic type T
// So you can define a Point instance with x and y to be integers or floats or chars 
// etc as shown below
struct Point<T> {
    x: T,
    y: T
}

fn main() {
    // both x and y here are integers so the Generic type for this struct "integer"
    // becomes integer type 
    let integer = Point {x: 5, y: 10}; 
    
    // The struct below has float as the Generic type
    let float = Point {x: 1.0, y: 4.2};

    // The below code won't compile if you used different types for x and y
    // Since we specified x and y will be of Type T. T type has to be same for 
    // both
    let wont_work = Point {x: 3, y: 4.0};
}

