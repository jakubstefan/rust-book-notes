#![allow(unused_variables)]
#![allow(dead_code)]

use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::fs;

fn main() {
    /*
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
    */

    /* Matching on different errors */
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    /* Closures */
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    /* unwrap */
    /* If the Result value is the Ok variant, it will return the value inside
    the Ok. If the Result is the Err variant, unwrap will call the panic! macro */
    let greeting_file = File::open("hello.txt").unwrap();

    /* expect */
    // it let us choose an error message
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

/* Propagating the error */
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

/* Alternative for propagating: the ? operator */
// does exactly the same as in read_username_from_file
fn read_username_from_file_alt1() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// and even simpler:
fn read_username_from_file_alt2() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// even simpler using fs::read_to_string
fn read_username_from_file_alt3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

/* If `text` is the empty string, this call to `next` will return `None`, in which
case we use `?` to stop and return `None`. If `text` is not the empty string, `next`
will return a `Some` value containing a string slice of the first line in text. */
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

/* Main can also return a `Result<(), E>`. The executable will exit with a value
of 0 if main returns `Ok(())` and will exit with a nonzero value if main returns
an `Err` value */
/*
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
*/

/* We can make a new type and put the validations in a function to create an
instance of the type rather than repeating the validations everywhere. That way,
it’s safe for functions to use the new type in their signatures and confidently
use the values they receive. One way to define a Guess type that will only create
an instance of Guess if the new function receives a value between 1 and 100. */
/* !! The conditions in which Guess::new might panic should be discussed in its
public-facing API documentation */
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            println!("The secret number will be between ")
        }

        Guess { value }
    }

    // getter
    pub fn value(&self) -> i32 {
        self.value
    }
}
