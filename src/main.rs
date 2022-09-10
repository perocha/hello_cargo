// Simple print function, very basic log :-)
fn my_print(function_name: &str, message: &str, counter: u32) {
    println!("{}::{}:{}", function_name, message, counter);
}

// Simple calculation
fn simple_calc(first_arg: u32, second_arg: u32) -> u32 {
    if first_arg > second_arg {
        println!("The first argument needs to be higher than the second!!");
        return 0;
    }

    first_arg * second_arg
}

fn main() {
    // Display a message saying "Hello, world!"
    println!("Hello, world!");

    // Simple arguments in println
    println!("Testing arguments {}", 'A');

    // Using simple variables (mutable and immutable)
    let mut a_number = 10;
    println!("My var number is {}", a_number);
    let a_word = "Word";
    a_number = 15;
    println!("My var number is {} and the var word is {}", a_number, a_word);

    // Var boolean
    let is_bigger = 1 > 4;
    println!("Is 1 bigger than 4? {}", is_bigger);

    // Var char and strings
    let one_char = 'C';
    let emoji_smile = ":-)";
    println!("The char {} and the string {}", one_char, emoji_smile);

    // Tuple
    let tuple_example = ('C', 5i32, true);
    println!("Print my tuple {} {} {}", tuple_example.0, tuple_example.1, tuple_example.2);

    // Structs
    struct StructStudent { name: String, level: u8, remote: bool}
    struct StructTuppleGrades (char, char, char, char, f32);
    let student_1 = StructStudent { name: String::from("Peter Rock"), remote: true, level: 2};
    let student_2: StructStudent = StructStudent { name: String::from("Another Student"), level: 4, remote: false };
    println!("Student name {}", student_1.name);
    println!("Student level {}", student_1.level);
    println!("Remote? {}", student_1.remote);
    println!("Student name {}", student_2.name);

    let mark_1 = StructTuppleGrades ('A', 'B', 'C', 'D', 123.12);
    println!("Grades: {}", mark_1.0);

    // User variable shadowing
    let shadow_number = 5;
    let shadow_number = shadow_number + 5;
    let shadow_number = shadow_number * 2;
    println!("The number is {}", shadow_number);

    // Call print function
    let func_name = "FunctionName";
    let message = "This is a message I want to print";
    let counter = 23;
    my_print(func_name, message, counter);

    // Call function
    let mut result = simple_calc (5, 6);
    println!("First result {}", result);
    result = simple_calc(4, 3);
    println!("Second result {}", result);
}