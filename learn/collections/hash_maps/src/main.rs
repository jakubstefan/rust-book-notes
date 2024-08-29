#![allow(unused_variables)]
fn main() {
    use std::collections::HashMap;

    /* Creating a new Hash Map */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    /* Accessing values in a Hash Map */
    let team_name = String::from("Blue)");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    /* The get method returns an Option<&V>; if there’s no value for that key in
     * the hash map, get will return None. This program handles the Option by
     * calling copied to get an Option<i32> rather than an Option<&i32>, then
     * unwrap_or to set score to zero if scores doesn't have an entry for the key.
     */

    // we can interate (in an arbitrary order) over each key/value pair 
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    /* Ownership */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    /* For owned values like String, the values will be moved and the hash map
    will be the owner of those values */
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point

    /* Overwriting a value */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores); // This code will print {"Blue": 25}

    /* Adding a Key and Value Only If a Key Isn’t Present */
    scores.entry(String::from("Yellow")).or_insert(50); // it will insert {"Yellow", 50}
    scores.entry(String::from("Blue")).or_insert(50); // it won't change Blue entry, as it exists already
    /* entry method returns an enum Entry. or_insert method on Entry returns a
    mutable reference to the value for the corresponding Entry key if that exists,
    otherwise inserts the new key/value pair */

    /* Updating a value based on the old value */
    // counts how many times a word is found
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // or_insert method returns a mutable reference (&mut V) to the value for the specified key
        *count += 1;
    }
    println!("{:?}", map); // prints {"world": 2, "hello": 1, "wonderful": 1}

    /* This program stores a vector of indexes for each occurrence of a given letter into a hashmap. Then it sums all
    the indexes for the letter 'l', which occurs at indexes 2 and 3 in the string "hello!". */
    let mut h: HashMap<char, Vec<usize>> = HashMap::new();
    for (i, c) in "hello!".chars().enumerate() {
        println!("{i} {c}");
        h.entry(c).or_insert(Vec::new()).push(i);
    }
    println!("{:?}", h);
    /* prints {'h': [0], 'e': [1], 'l': [2, 3], '!': [5], 'o': [4]}
    */
    let mut sum = 0;
    for i in h.get(&'l').unwrap() {
        sum += *i;
    }
    println!("{}", sum); // prints 5
}
