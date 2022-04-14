// Multiple generic types in struct fields

// You can have as many generic types as you want
struct Point<T, U> {
    x: T, // x will be of type T
    y: U // y could be of different type U
}

fn main() {
    let both_integer = Point {x: 4, y: 5};
    let both_float = Point {x: 1.2, y: 4.2};

    // In the below T is integer and U is float type
    let integer_and_float = Point {x: 5, y: 4.0};
}