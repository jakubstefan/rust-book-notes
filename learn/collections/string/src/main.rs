#![allow(unused_variables)]
#![allow(unused_mut)]

fn main() {
    /* Creating a new String */
    let mut s = String::new();

    // we can also create from initial data
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    /* Updating a String */
    // Appending a string slice
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    // it doesn't take ownership of the string slice
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // push method pushes a single character
    let mut s = String::from("lo");
    s.push('l'); // s now contains lol

    // Concatenation with the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    /* The + operator uses the add method, whose signature looks something like this:
    fn add(self, s: &str) -> String { */
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3; // "tic-tac-toe"

    // easier to read with the format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // it doesn't take ownwership of any of its parameters

    /* Slicing Strings */
    // indexing is not permited, but you can slice the string
    let hello = "Здравствуйте"; // every "letter" of the Cyrillic alphabet takes 2 bytes
    let s = &hello[0..4];
    // &hello[0..1] would panic at runtime (it breaks a letter in half)

    /* Methods for iterating over Strings */
    // chars method separates the string out into letters
    println!("Break it into chars (not really letters, see Devanagari Hindi script)");
    for c in "Зд".chars() {
        println!("{c}");
    }

    // the bytes method breaks it down into bytes instead
    println!("Break it into bytes");
    for b in "Зд".bytes() {
        println!("{b}");
    }

}
