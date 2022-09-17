use std::fmt;

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

trait Area {
    fn area(&self) -> f64;
}

impl Area for Circle {
    fn area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * self.radius.powf(2.0)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

#[derive(Debug, PartialEq)]
struct MyPoint {
    x: i32,
    y: i32,
}

impl fmt::Display for MyPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "custom display ({}, {})", self.x, self.y)
    }
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

    // Trait example
    let my_circle = Circle { radius: 5.0 };
    let my_rectangle = Rectangle {width: 12.00, height: 15.00};

    println!("The circle area is: {}", my_circle.area());
    println!("The rectangle area is: {}", my_rectangle.area());

    // Derive example
    let p1 = MyPoint { x: 1, y: 2 };
    let p2 = MyPoint { x: 4, y: -3 };

    if p1 == p2 { // can't compare two Point values!
        println!("equal!");
    } else {
        println!("not equal!");
    }

    println!("{}", p1);
    println!("{:?}", p1);

}
