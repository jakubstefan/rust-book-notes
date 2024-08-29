use std::collections::HashMap;

fn main() {
    exercise_median_and_mode();
    exercise_convert_to_pig_latin();
    exercise_employees();
}

/// Given a list of integers, use a vector and return the median (when sorted,
/// the value in the middle position) and mode (the value that occurs most often;
/// a hash map will be helpful here) of the list
fn exercise_median_and_mode() {
    println!("\n** Median an mode **");
    
    let list = [1, 2, 3, 4];
    println!("{:?} => median: {}, mode: {}", list, median(&list), mode(&list));
    
    let list = [1, 48, 72, 82, 77, 91, 4, 92, 100, 9, 56, 54, 92, 97, 2];
    println!("{:?} => median: {}, mode: {}", list, median(&list), mode(&list));
    
    let list = [94, 22, 1, 49, 1, 35, 49, 95, 22, 73, 46, 1];
    println!("{:?} => median: {}, mode: {}", list, median(&list), mode(&list));
}

fn median(list: &[i32]) -> i32 {
    if list.len() == 0 {
        panic!("Empty list");
    }

    let mut v: Vec<i32> = Vec::new();
    for num in list {
        v.push(*num);
    }
    v.sort();

    let median_position = v.len() / 2;
    v[median_position]
}

fn mode(list: &[i32]) -> i32 {
    if list.len() == 0 {
        panic!("Empty list");
    }

    let mut repetitions = HashMap::new();
    for num in list {
        let count = repetitions.entry(num).or_insert(0);
        *count += 1;
    }

    let mut max = i32::MIN;
    let mut mode: i32 = 0;
    for (key, value) in &repetitions {
        if *value >= max {
            max = *value;
            mode = **key;
        }
    }

    mode
}

/// Convert strings to pig latin. The first consonant of each word is moved to
/// the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words
/// that start with a vowel have “hay” added to the end instead (“apple” becomes
/// “apple-hay”). Keep in mind the details about UTF-8 encoding!
fn exercise_convert_to_pig_latin() {
    println!("\n** Convert to pig latin **");
    
    let string = String::from("first");
    println!("{string} => {}", covert_to_pig_latin(&string));
    
    let string = String::from("apple");
    println!("{string} => {}", covert_to_pig_latin(&string));
}

fn covert_to_pig_latin(s: &String) -> String {
    let mut converted_s: String = String::new();

    let first_letter = s.chars().nth(0).unwrap(); // will panic if no first element

    if "aeiouAEIOU".contains(first_letter) {
        // it's a vowel, so just add '-hay' to the end
        converted_s = format!("{s}-hay");
    }
    else {
        // a consonant, so move the first letter to the end + 'ay'
        let mut count = 0;
        for c in s.chars() {
            if count > 0 {
                converted_s.push(c);
            }
            count += 1;
        }
        converted_s = format!("{converted_s}-{first_letter}ay");
    }

    converted_s
}

/// Using a hash map and vectors, create a text interface to allow a user to add
/// employee names to a department in a company. For example, “Add Sally to
/// Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all
/// people in a department or all people in the company by department, sorted alphabetically.
fn exercise_employees() {
    println!("\n** Employees **");

    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    list_employees_by_department(&employees);
    add_employee(&mut employees, String::from("Sally"), String::from("Engineering"));
    add_employee(&mut employees, String::from("Amir"), String::from("Sales"));
    add_employee(&mut employees, String::from("Billy"), String::from("Engineering"));
    list_employees_by_department(&employees);
    list_employees_in_department(&employees, &String::from("Sales"));
}

fn add_employee(employees: &mut HashMap<String, Vec<String>>, name: String, department: String) {
    println!("Adding {name} to {department}...");
    let names_in_department = employees.entry(department).or_insert(Vec::new());
    names_in_department.push(name);
}

fn list_employees_in_department(employees: &HashMap<String, Vec<String>>, department: &String) {
    println!("Listing employees in department {department}...");
    let employees_in_department = employees.get(department);
    match employees_in_department {
        Some(employees_in_department) => println!("Employees in {department}: {:?}", employees_in_department),
        None => println!("Department {department} doesn't exist"),
    }
}

fn list_employees_by_department(employees: &HashMap<String, Vec<String>>) {
    println!("Listing employees by department...");

    if employees.is_empty() {
        println!("! There are no employees");
        return
    }

    for (department, names) in employees {
        println!("Employees in {department}: {:?}", names);
    }
}
