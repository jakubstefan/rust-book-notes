#![allow(dead_code)] // do not warn that is never used
#![allow(unused_variables)]

use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

/* Basics */
fn enum_basic() {
    enum IpAddrKind {
        V4,
        V6,
    }
    fn route(_ip_kind: IpAddrKind) {}

    // instances of the variants
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn ip_definition() {
    /* We could define an IP with structs and enums */
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    /* However, it's more concise to put data directly into each enum variant */
    enum IpAddr2 {
        V4(String),
        V6(String),
    }

    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    /* IpAddr::V4() is a function call that takes a String argument and returns
    an instance of the IpAddr type */

    /* We could also have different types and amounts in each variant */
    enum IpAddr3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));

    /* But the standard library already has a difinition for IP addresses */
    enum IpAddr4 {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
}

fn message() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // we can also have methods for enums
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn matching() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alaska));
}

/* Matching with Option<T>
    enum Option<T> {
        None,
        Some(T),
    }
*/
fn matching_with_option() {
    /* if there’s a value inside, adds 1 to that value. If there isn’t a value
    inside, the function should return the None value */
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {:?}", five);
    println!("six: {:?}", six);
    println!("none: {:?}", none);
}

fn matching_other_values() {
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    #[allow(unused_variables)]
    fn move_player(num_spaces: u8) {}
    fn reroll() {}

    let dice_roll = 9;
    /* Note: The options are evaluated in order! */
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    };

    /* If we didn't use the variable in other, we should use _ instead */
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    };

    /* Finally, if we even didn't want to do anything when other: */
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // unit value (empty tuple)
    };
}

fn matching_ownership() {
    /* This will not compile:

        let opt: Option<String> = Some(String::from("Hello world"));

        match opt {
            // _ became s
            Some(s) => println!("Some: {}", s),
            None => println!("None!")
        };

        println!("{:?}", opt);

    A match on opt will move non-ignored fields like s.
     */

    /* This compiles. It matches on a reference */
    let opt: Option<String> = Some(String::from("Hello world"));

    match &opt {
        Some(s) => println!("Some: {}", s),
        None => println!("None!"),
    };

    println!("{:?}", opt);
}

/* `if let` */
fn if_let() {
    /* When we only process on variant, there a lot of boilerplate for such simple code */
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    /* We can instead make use of `if let` */
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn main() {
    enum_basic();
    ip_definition();
    message();
    matching();
    matching_with_option();
    matching_other_values();
    matching_ownership();
    if_let();
}

/* Ownership notes */

/// Makes a string to separate lines of text,
/// returning a default if the provided string is blank
fn make_separator(user_str: &str) -> String {
    if user_str == "" {
        let default = "=".repeat(10);
        default
    } else {
        user_str.to_string()
    }
}
// although it would be best to use `Cow`

/// Gets the string out of an option if it exists,
/// returning a default otherwise
fn get_or_default(arg: &Option<String>) -> String {
    match arg {
        None => String::new(),
        Some(s) => s.clone()
    }
}
