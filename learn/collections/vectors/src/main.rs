#![allow(unused_variables)]

fn main() {
    /* Creating a vector */
    // Creating a new empty vector
    let v: Vec<i32> = Vec::new();

    // Creating a new vector with some values
    let v = vec![1, 2, 3];

    /* Updating a vector */
    let mut v = Vec::new();
    v.push(5);
    v.push(6);

    /* Reading elements of vectors */
    let v = vec![1, 2, 3, 4, 5];

    // via indexing
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // using the get method
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    /* Iterating */
    let v = vec![100, 32, 57];
    for n_ref in &v {
        // n_ref has type &i32
        let n_plus_one: i32 = *n_ref + 1;
        println!( "{n_plus_one}");
    }

    // iterate over mutable references
    let mut v = vec![100, 32, 57];
    for n_ref in &mut v {
        // n_ref has type &mut i32
        *n_ref += 50;
    }

    // Safely Using Iterators
    let mut v: Vec<i32>         = vec![1, 2];
    let mut iter: Iter<'_, i32> = v.iter();
    let n1: &i32                = iter.next().unwrap(); // will point to first element
    let n2: &i32                = iter.next().unwrap(); // will point to second element
    let end: Option<&i32>       = iter.next(); // None

    // Iterate over a vector using a range
    let mut v: Vec<i32>        = vec![1, 2];
    let mut iter: Range<usize> = 0 .. v.len();
    let i1: usize              = iter.next().unwrap();
    let n1: &i32               = &v[i1];

    /* Using an Enum to store multiple types in a vector */
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    // All the elements will be considered to be of the same type (enum Spreadsheetcell) by the vector
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ]
}
