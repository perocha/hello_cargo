pub fn memory_examples() {
    let mascot = String::from("ferris");
    println!("mascot {}", mascot);

    let ferris = mascot;
    println!("ferris {}", ferris);

//  println!("ferris {}", mascot);  ----> This will not compile since mascot is out of scope

    let my_string = String::from("Hello!!");
    my_process_string (my_string.clone());
    my_process_string (my_string);
//  my_process_string (my_string);  ----> This will not compile, ownership transfered

    // Pass the reference of a string
    let my_new_string = String::from("Hello world ref!!");
    my_process_string_ref (&my_new_string);
    my_process_string_ref (&my_new_string);
    my_process_string_ref (&my_new_string);

    let mut my_mut_string = String::from("This is a mutable string");
    my_process_string_ref_mutable (&mut my_mut_string);

    let my_int = 32;
    // In this case my_int is a simple type, it's a "copy" type, ownership is not transfered
    my_process_int (my_int);
    my_process_int (my_int);
    my_process_int (my_int);
    my_process_int (my_int);

    // Simple use of reference var
    let x;
    let y = 42;
    {
        x = &y;
    }
    println!("x: {}", x);
    println!("y: {}", y);

    // String handling with lifetime in function my_longest_word
    let magic1 = String::from("abracadabra!");
    let magic2 = String::from("shazam!");

    let result = my_longest_word(&magic1, &magic2);
    println!("The longest magic word is {}", result);

}

// Function passing String argument will get ownership of var input
fn my_process_string (input: String) {
    println!("my_process_string:input {}", input);
}

// Function with reference string as an input, ownership is not transfered
fn my_process_string_ref (input: &String) {
    println!("my_process_string_ref:input {}", input);
}

// Function with reference string as an input, ownership is not transfered
fn my_process_string_ref_mutable (input: &mut String) {
    println!("my_process_string_ref_mutable:input {}", input);
    input.push_str(", and now it's changed!");
    println!("my_process_string_ref_mutable:input {}", input);
}

// Function passing u32 argument will *NOT* get ownership of var input, since int is a "copy" type
fn my_process_int (input: u32) {
    println!("my_process_int::input {}", input);
}

// Function that returns the longest string of 2 inputs, with lifetime annotation
fn my_longest_word<'lifetime>(first_string: &'lifetime String, second_string: &'lifetime String) -> &'lifetime String {
    if first_string.len() > second_string.len() {
        first_string
    } else {
        second_string
    }
}