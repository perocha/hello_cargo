#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

pub fn generic_data_types_examples() {
    // Declare different variables with the generic data type Point
    let boolean_var = Point {x: true, y: false};
    let int_var = Point {x: 1, y: 2};
    let string_var = Point {x: "the x", y: "the y"};
    let mix_var = Point {x: "the x", y: 123};

    println!("Boolean::{:?} {:?}", boolean_var.x, boolean_var.y);
    println!("Integer::{:?} {:?}", int_var.x, int_var.y);
    println!("String::{:?} {:?}", string_var.x, string_var.y);
    println!("Mix::{:?} {:?}", mix_var.x, mix_var.y);
 
}
