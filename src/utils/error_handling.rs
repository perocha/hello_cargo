#[derive(Debug)]
struct DivisionByZeroError;

// Safe division function, will return struct DivisionByZeroError when the divisor is 0
fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}

pub fn error_examples(){
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    // pick the first item:
    let first = fruits.get(0);
    println!("{:?}", first);

    // pick the third item:
    let third = fruits.get(2);
    println!("{:?}", third);

    // pick the 99th item, which is non-existent - it will return "None" because it's using the vector get method
    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);

    // Iterate the vector using match operator
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(fruit_name) => println!("index::{} It's a delicious {}!", index, fruit_name),
            None => println!("index::{} There is no fruit! :(", index),
        }
    }

    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));
}