// Generics in std library
// Option<T> and Result<T, E> examples

// Below is the std library enum with Some taking a generic "T" type variable
// So Some can have any type of value integer, float, char etc
enum Option<T> {
    Some(T),
    None
}

// Similarly Result use two Generic types T and E. T for Ok variant and E for Err variant
enum Result<T, E> {
    Ok(T),
    Err(E)
}