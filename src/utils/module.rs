struct Rectangle {
    width: u32,
    height: u32,
}

pub fn test_function() {
    println!("test_module::test_function");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

// Area
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
