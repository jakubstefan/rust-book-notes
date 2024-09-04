#![allow(dead_code)] // do not warn that is never used
#![allow(unused_variables)]

/* examples of references to an i32:
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime */

/* For some lifetime 'a, the function takes two parameters, both of which are
string slices that live at least as long as lifetime 'a. The function signature
also tells Rust that the string slice returned from the function will live at least
as long as lifetime 'a.
In practice, it means that the lifetime of the reference returned by the longest
function is the same as the smaller of the lifetimes of the values referred to by
the function arguments
! Note: it does NOT change the lifetimes, but specifies that the borrow checker
should reject any values that don’t adhere to these constraints */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/* Lifetime annotations in struct definitions */
/* We can define structs to hold references, but in that case we would need to
add a lifetime annotation on every reference in the struct’s definition. */
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    /* the static lifetime: the reference **can** live for the entire duration of
    the program */
    let s: &'static str = "I have a static lifetime.";
}

/* Lifetime Elision: some cases don't need lifetime annotations, as they were use
so often, that they were added to the compiler logic */
/* Rules that the compiler applies to automatically assign lifetimes:
1) each parameter gets its own lifetime, although for structures like ImportantExcerpt<'_>:
    fn foo(x: &'a ImportantExcerpt<'b>)
2) if there's only one input lifetime, it gets assigned to all output lifetime
    parameters
3) if one of the input lifetime parameters is `&self` or `&mut self`, its lifetime
    is assigned to all output lifetime parameters */
fn first_word(s:&str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

use std::fmt::Display;
/* Generic Type Paramenters & Trait Bounds & Lifetimes */
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where 
    T: Display,
{
    println!("Annoucement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
