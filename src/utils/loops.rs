pub fn execute_loops(){
    //
    // Simple loop example
    //
    loop {
        // Keep printing, printing, printing...
        println!("We loop forever!");
        // On the other hand, maybe we should stop!
        break;                            
    }

    //
    // A more complex example of the usage of loop
    //
    let mut counter = 1;
    // stop_loop is set when loop stops
    let stop_loop = loop {
        counter *= 2;
        if counter > 256 {
            // Stop the loop and return the counter value
            break counter;
        }
    };
    // Print the value of variable stop_loop when the loop was stopped
    println!("Break the loop at {}", stop_loop);

    //
    // While examples
    //
    println!("Simple while example");
    let mut counter = 0;
    while counter < 10 {
        println!("while::{}", counter);
        counter += 1;
    }

    //
    // For examples
    //
    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
        println!("For loop::{}", bird);
    }

    for number in 0..10 {
        println!("For loop::{}::{}", number, number*3);
    }
}