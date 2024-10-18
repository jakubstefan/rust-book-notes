fn main() {
    /* Create an iterator and use it in a loop */
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got {val}");
    }

    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    println!("Next: {}", v1_iter.next().unwrap()); // 1
    /* The values we get from `next` are immutable references to the values in
    the vector*/

    /* To create an iterator that returns owned values: into_iter() */
    /* To create an iterator that returns mutable references: iter_mut() */

    /* `sum` method uses `next` */
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("Total: {total}"); // 6
    /* We aren't allowed to use `v1_iter` anymore, as `sum` took ownership of it */

    /* `map` does not consume the iterator. Here we'll sum one to each element */
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); //
    /* As iterators are lazy, we need to add `collect` to consume it */
    assert_eq!(v2, vec![2, 3, 4]);

    shoes_in_10();
}

/* If the closure in `filter` returns true, the value will be included in the
iteration produced by `filter` */
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// Takes ownership of the vector
fn shoes_in_size(shoes: Vec<Shoe>, shoes_size: u32) -> Vec<Shoe> {
    // `into_iter` creates an iterator that takes ownership of the vector
    shoes.into_iter().filter(|s| s.size == shoes_size).collect()
}

fn shoes_in_10() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let in_my_size = shoes_in_size(shoes, 10);
    println!("Shoes with size 10: {:?}", in_my_size);
}
