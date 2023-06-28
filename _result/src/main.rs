use std::io::{self, Read};
use std::io::ErrorKind;
use std::fs::{File, self};

// T refers to the type of the value to be returned in the success case
// E refers to the type of error to be returned in the failure case
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    handle_with_match();
    handle_with_closure();
    handle_with_unwrap();
    handle_with_expect();
    handle_and_propogate();
    handle_and_propogate_short();
}

fn handle_with_match() {
    // `File::open` returns either the file wrapped in a `Ok` variant or
    // a `io::Error` wrapped in a `Err` variant.
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // `kind` method is implemented by struct `io::Error`
        Err(err) => match err.kind() {
            // if `ErrorKind::NotFound`, create the file
            // if file can not be created, panic
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem opening the file {:?}", e),
            }
            // if error is any type other than `NotFound`, panic
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            }
        }
    };
}

fn handle_with_closure() {
// This is an example from the book and I honestly have no clue how it works
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file {:?}", error);
        }
    });
}

fn handle_with_unwrap() {
    // `unwrap` will return the value contained in the `Ok` if the result
    // is success. If the result is a failure, it will call `panic!`
    let greeting_file = File::open("hello.txt").unwrap();
}

fn handle_with_expect() {
    // `expect` will return the value contained in the `Ok` if the result
    // is success. If the result is a failure, it will call `panic!` and
    // pass the value passed to `expect` along to the `panic!` call.
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

fn handle_and_propogate() {
    let result = read_username_from_file();
}

// Return type `Result<String, io::Error>` works for errors produced by
// both operations.
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    // `username_file` will be mutated later by `read_to_string`
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    // `username` will also be mutated by `read_to_string`
    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_)  => Ok(username),
        Err(e) => Err(e),
    }
}

fn handle_and_propogate_short() {
    let result = read_username_from_file_short();
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    // usage of `?` operator calls the `from` function, defined in the `From`
    // trait in the standard library. The from function converts the error
    // type received to the return type of the function and returns it.
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn handle_and_propogate_shorter() {
    let result = read_username_from_file_shorter();
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    // this is simply a reduction of the last `read_username*` function
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn handle_and_propogate_shortest() {
    let result = read_username_from_file_shortest();
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    // This function implements `fs::read_to_string` which is functionally
    // identical to the other `read_username*` functions.
    fs::read_to_string("hello.txt")
}
