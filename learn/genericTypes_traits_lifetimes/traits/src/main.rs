#![allow(unused_variables)]
#![allow(dead_code)]

/* We have multiple structs that hold various kinds and amounts of text: a
NewsArticle struct that holds a news story filed in a particular location and a
Tweet that can have, at most, 280 characters along with metadata that indicates
whether it was a new tweet, a retweet, or a reply to another tweet.

We want to make a media aggregator library crate named aggregator that can display
summaries of data that might be stored in a NewsArticle or Tweet instance. To do
this, we need a summary from each type, and we’ll request that summary by calling
a summarize method on an instance. Listing 10-12 shows the definition of a public
Summary trait that expresses this behavior.

Each type implementing this trait must provide its own custom behavior for the body of the method.
*/
pub trait Summary {
    fn summarize_author(&self) -> String;

    // fn summarize(&self) -> String; // if we didn't define a default implementation
    fn summarize(&self) -> String { // this is the default implementation
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
} // it will use the default implementation

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// This &impl Summary parameter accepts any type that implements the specified trait
pub fn notify(item: &impl Summary) {
/* this (trait bounds) would be equivalent, but more verbose
pub fn notify<T: Summary>(item: &T) { */
/* we can have two parameters
pub fn notify(item1: &impl Summary, item2: &impl Summary) { */
/* if we want to force both parameters to have the same type:
pub fn notify<T: Summary>(item1: &T, item2: &T) { */
/* multiple trait bounds (item must implement both Display and Summary)
pub fn notify(item: &(impl Summary + Display)) { */
/* also with trait bounds
pub fn notify<T: Summary + Display>(item: &T) { */
    println!("Breaking news! {}", item.summarize());
}

use std::fmt::{Display, Debug};
/* this
fn some_funtion<T:Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
can be more less cluttered like this: */
fn some_funtion<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

/* We can also use the impl Trait syntax in the return position to return a value
of some type that implements a trait */
/* Note: you can only use it if youre returning a single type
(if X Tweet, else NewsArticle wouldn't work) */
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize()); // prints `New article available! (Read more...)`
}

/* Conditionally implement methods */
/* Pair<T>:
     - always implements the `new` function to return a new instance of Pair<T>
     - only implements the `cmp_display` method if its inner type T implements the
       PartialOrd trait that enables comparison and the Display trait that enables
       printing */

       struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self{ x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

/* Conditionally implement a trait for any type that implements another trait */
/* "blanket implementation", for example this one from the standard library:
impl<T: Display> ToString for T {
    //--snip--
} */
/* For example, we can turn integers into their corresponding String values like
this because integers implement Display:
let s = 3.to_string(); */
