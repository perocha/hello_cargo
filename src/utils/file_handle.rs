use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

fn read_file_contents(path: PathBuf) -> Result<String, Error> {
    let mut string = String::new();

    // Access a file at a specified path
    // - Pass variable to `file` variable on success, or
    // - Return from function early if there's an error
    let mut file: File = match File::open(path) {
        Ok(file_handle) => file_handle,
        Err(io_error) => return Err(io_error),
    };

    // Read file contents into `String` variable with `read_to_string`
    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(io_error) => return Err(io_error),
    };

    return Ok(string)
}

pub fn file_handle_example() {
    // Read the file content and print it
    let string = read_file_contents(PathBuf::from("c:\\temp\\test.txt"));
    if string.is_ok() {
        println!("The program found the main file");
        println!("File content::{:?}", string);
    }

    // Read an non-existent file
    if read_file_contents(PathBuf::from("non-existent-file.txt")).is_err() {
        println!("The program reported an error for the file that doesn't exist.");
    }
}