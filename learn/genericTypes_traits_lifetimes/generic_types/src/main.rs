#![allow(unused_variables)]
#![allow(dead_code)]

/* To simplify
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}*/

/*fn largest<T>(list: &[T]) -> &T { this needs a trait bound) */
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct PointSameType<T> {
    x: T,
    y: T,
}

impl<T> PointSameType<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// we can restrict the types
// it won't work for types other than f32
impl PointSameType<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointMultipleTypes<X1, Y1> {
    x: X1,
    y: Y1,
}

// we can use more generic types than the ones from the struct
impl<X1, Y1> PointMultipleTypes<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointMultipleTypes<X2, Y2>) -> PointMultipleTypes<X1, Y2> {
        PointMultipleTypes {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    let integer = PointSameType { x: 5, y: 10 };
    println!("integer.x = {}", integer.x());
    let float = PointSameType { x: 1.0, y: 4.0 };

    let both_integer = PointMultipleTypes { x: 5, y: 10 };
    let both_float = PointMultipleTypes { x: 1.0, y: 4.0 };
    let integer_and_float = PointMultipleTypes { x: 5, y: 4.0 };

    let p1 = PointMultipleTypes { x: 5, y: 10.4 };
    let p2 = PointMultipleTypes { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y); // will print: p3.x = 5, p3.y = c
}
