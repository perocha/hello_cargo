use std::collections::HashMap;

#[derive(PartialEq, Debug)]
struct Car { color: String, motor: Transmission, roof: bool, age: (Age, u32) }

#[derive(PartialEq, Debug)]
enum Transmission { Manual, SemiAuto, Automatic }

#[derive(PartialEq, Debug)]
enum Age { New, Used }

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Return tuple with car age ("New" or "Used") and mileage
fn car_quality (miles: u32) -> (Age, u32) {

    // Check if car has accumulated miles
    // Return tuple early for Used car
    if miles > 0 {
        return (Age::Used, miles);
    }

    // Return tuple for New car, no need for "return" keyword or semicolon
    (Age::New, miles)
}

// Build "Car" using input arguments
fn build_car(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Prevent panic: Check color index for colors array, reset as needed
    // Valid color = 1, 2, 3, or 4
    // If color > 4, reduce color to valid index
    let mut color = order as usize;
    if color > 4 {        
        // color = 5 --> index 1, 6 --> 2, 7 --> 3, 8 --> 4
        color = color - 4;
    }

    // Add variety to orders for motor type and roof type
    let mut motor = Transmission::Manual;
    let mut roof = true;
    if order % 3 == 0 {          // 3, 6, 9
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {   // 2, 4, 8, 10
        motor = Transmission::SemiAuto;
        roof = false;
    }                            // 1, 5, 7, 11

    // Return requested "Car"
    Car {
        color: String::from(colors[(color-1) as usize]),
        motor: motor,
        roof: roof,
        age: car_quality(miles)
    }
}

pub fn execute() {
    // Initialize a hash map for the car orders
    use std::collections::HashMap;
    let mut orders: HashMap<i32, Car> = HashMap::new();

    // Initialize counter variable
    let mut order = 1;
    // Declare a car as mutable "Car" struct
    let mut car: Car;

    // Car order #1
    car = build_car(order, 1000);
    orders.insert(order, car);
    order = order + 1;

    // Car order #2
    car = build_car(order, 0);
    orders.insert(order, car);
    order = order + 1;

    // Car order #3
    car = build_car(order, 10);
    orders.insert(order, car);
    order = order + 1;

    // Car order #4
    car = build_car(order, 23023);
    orders.insert(order, car);
    order = order + 1;

    // Car order #5
    car = build_car(order, 0);
    orders.insert(order, car);
    order = order + 1;

    for _i in 1..order {
        println!("Car order {}: {:?}", _i, orders.get(&_i));
    }

}