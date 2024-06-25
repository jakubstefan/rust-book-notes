fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

/* Returning a Reference to the Stack */
// Move ownership of the string out of the function
fn return_a_string_1() -> String {
    let s = String::from("Hello world");
    s
}
// return a string literal, which lives forever (if we never intend to change the string, and then a heap allocation is unnecessary)
fn return_a_string_2() -> &'static str {
    "Hello world"
}
// defer borrow-checking to runtime by using garbage collection with a reference-counted pointer
use std::rc::Rc;
fn return_a_string_3() -> Rc<String> {
    let s = Rc::new(String::from("Hello world"));
    Rc::clone(&s)
}
// have the caller provide a "slot" to put the string using a mutable reference
// I don't know how to use it yet
fn return_a_string_4(output: &mut String) {
    output.replace_range(.., "Hello world");
}

fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}

fn round_in_place(v: &mut Vec<f32>) {
    for n in v {
        *n = n.round();
    }
}
// gets a reference to the largest string in a vector, and then uses it while mutating the vector
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

/* The Slice Type */
// takes a string of words separated by spaces and returns the first
// word it finds in that string
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

type Document = Vec<String>;
fn new_document(words: Vec<String>) -> Document {
    words
}
fn add_word(this: &mut Document, word: String) {
    this.push(word);
}
fn get_words(this: &Document) -> &[String] {
    this.as_slice()
}


fn main() {
    let a = 5;
    let mut b = a;
    b += 1;

    // Copies a into b
    let a = [0; 10];
    let b = a;

    // Move ownership.using a Box: used for putting data on the heap
    let a = Box::new([0; 10]);
    let b = a; // a pointer to what 'a' is pointing to. 'a' is now invalidated, as the ownership has been moved

    // Also moves ownership. 'first' cannot be used after add_suffix is called, as it is invalidated
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");

    // Clones the String first
    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");

    /* Returning a Reference to the Stack */
    let moved_string = return_a_string_1();
    println!("this is my string: '{moved_string}'");

    let string_literal = return_a_string_2();
    println!("this is my string: '{string_literal}'");

    let reference_counted_pointer_to_str = return_a_string_3();
    println!("this is my string: '{reference_counted_pointer_to_str}'");

    /* */
    let name: Vec<String> = vec![String::from("Ferris"), String::from("Jr.")];
    println!("Name + title: '{}'", stringify_name_with_title(&name));

    let mut vec: Vec<f32> = vec![1.4, 4.6];
    println!("original vector: {:?}", vec);
    round_in_place(&mut vec);
    println!("rounded: {:?}", vec);

    let mut vec: Vec<String> = vec![String::from("A"), String::from("AA"), String::from("AAA")];
    let src = [String::from("BB"), String::from("BBB"), String::from("BBBB")];
    add_big_strings(&mut vec, &src);
    println!("vec: {:?}, src: {:?}", vec, src);

    /* Access an element of a vector */
    // use immutable reference (does not take ownership of string)
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    println!("{s_ref}!");
    // clone data and leave the vector alone
    let v: Vec<String> = vec![String::from("Hello world")];
    let mut s:String = v[0].clone();
    s.push('!');
    println!("{s}");
    // use Vec::remove to move the string out of the vector
    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.len() == 0);

    /* Mutate different tuple fields  */
    let mut name = (
        String::from("Ferris"), 
        String::from("Rustacean")
    );
    let first = &name.0;
    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);

    /* Mutate different array elements */
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1];
    *x += 1;
    println!("{a:?}");

    /* String slices */
    let s = String::from("hello world");
    let hello : &str = &s[..5];
    let world : &str = &s[6..];
    let whole : &str = &s[..];
    println!("original string: {s}");
    println!("sliced 'hello': {hello}");
    println!("sliced 'world': {world}");
    println!("whole sliced string: {whole}");
    let s = String::from("hello world");
    let first_word = first_word(&s);
    println!("first word of '{s}': '{first_word}'");
    // a string literal (&str) is also a slice
    let s = "hello world"; // this is of type &str

    /* Other Slices */
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // type &[i32]
    assert_eq!(slice, &[2, 3]);

    let words = vec!["hello".to_string()];
    let d = new_document(words);
    // .to_vec() converts &[String] to Vec<String> by cloning each string
    let words_copy = get_words(&d).to_vec();
    let mut d2 = new_document(words_copy);
    add_word(&mut d2, "world".to_string());
    // The modification to `d2` does not affect `d`
    assert!(!get_words(&d).contains(&"world".into()));
}
