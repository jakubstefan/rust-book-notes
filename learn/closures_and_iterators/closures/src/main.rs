#![allow(unused_variables)]
#![allow(dead_code)]

use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        /* `|| self.most_stocked()`is a closure expression. It takes no parameters itself,
        which would appear between the two vertical vars.
        The closure is defined here, but it is `unwrap_or_else` who will evaluate it
        later if the result is needed. */
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn store_shirt_giveaway() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

fn store_closure_in_variable_and_type_annotation() {
    /* Most of the time, it is not needed to type annotate. Some times you would
    like to be verbose, though, or in some rare cases the compiler need closure
    type annotations. */
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    /* When storing closure definitions in a variable, they look very similar to functions:
    fn  add_one_v1   (x: u32) -> u32 { x + 1 } */
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ; // the brackets are optional, as there is only one expression

    add_one_v2(1);
    add_one_v3(1);
    add_one_v4(1);

    /* The first time a closure is called, the compiler infers the type of its
    arguments and the return type. They cannot be more than one. */
}

/* The toilet closure is similar to std::mem::drop, i.e. a function that moves
an argument and causes it to be dropped. */
fn toilet_closure() {
    let f = |_| ();
    let s = String::from("Hello");
    f(s);
}

fn capturing_references_or_moving_ownwership() {
    /* Capturing an immutable reference */
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    /* Capturing a mutable reference */
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let mut borrows_mutably = || list.push(7);
    /* Can't to the following, as in previous line it caputures a mutable reference
    to list and no other borrows are allowed when there's a mutable borrow. */
    // println!("Before calling closure: {list:?}");
    borrows_mutably();
    println!("After calling closure: {list:?}");

    /* Forcing ownership with 'move' */
    /* We can force the closure to take ownership of the values it uses in the
    environment even though the body of the closure doesn't strictly need
    ownership */
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    /* sort_by_key is defined to take an FnMut closure (it calls it multiple times) */
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}

fn captured_lifetimes() {

    /* This would not compile, as we should tell Rust that the closure returned
    from `make_a_cloner`~must not live longer than `s_ref`. */
    // fn make_a_cloner(s_ref: &str) -> impl Fn() -> String {
    //     move || s_ref.to_string()
    // }

    /* `s_ref` is a string reference that lives for `'a`. Adding `+ 'a` to the return
    type’s trait bounds indicates that the closure must live no longer than `'a` */
    fn make_a_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
    /* By using the lifetime elision rules, we could re-write it this way:
    fn make_a_cloner(s_ref: &str) -> impl Fn() -> String + '_ { */
            move || s_ref.to_string()
    }

    let s_own = String::from("Hello world");
    let cloner = make_a_cloner(&s_own);
    /* Adding the following, would make it not to compile, as `s_own` cannot be
    dropped as long as `make_a_cloner` is in use */
    // drop(s_own);
    cloner();

}

fn main() {
    store_shirt_giveaway();
    store_closure_in_variable_and_type_annotation();
    toilet_closure();
    capturing_references_or_moving_ownwership();
    captured_lifetimes();
}
