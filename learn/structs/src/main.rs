struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// Using the Field Init Shorthand
fn build_user_neater(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

/* Tuple Structs */
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/* Unit-Like Structs */
// it has no fields, similar to ()
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

/* Methods */
#[derive(Debug)]
struct Rectangle2 {
    width: u32,
    height: u32,
}

impl Rectangle2 {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle2) -> bool {
        self.width > other.width && self.height > other.height
    }

    // an associated function
    fn square(size: u32) -> Self { // Self referes to the type
        Self {
            width: size,
            height: size,
        }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Self) -> Self {
        Self { 
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("some@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(String::from("myemail@example.com"), String::from("anotherusername"));
    let user3 = build_user_neater(String::from("myemail@example.com"), String::from("anotherusername"));

    // Creating Instances from Other Instances with Struct Update Syntax
    // The syntax .. specifies that the remaining fields not explicitly set
    // should have the same value as the fields in the given instance.
    let user4 = User {
        email: String::from("another@example.com"),
        ..user1 // must come last
    };
    // We can no longer use user1 after creating user4 because the String in the
    // username field of user1 was moved into user4

    /* Tuple Structs */
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    /* Unit-Like Structs */
    let subject = AlwaysEqual;

    /* Borrowing Fields of a Struct */
    struct Point2 { x: i32, y: i32 }
    let mut p = Point2 { x: 0, y: 0 };
    let x = &mut p.x;
    *x += 1;
    println!("{}, {}", p.x, p.y);

    /* Rectangle example */
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1); // to print debug information
    println!("rect1 is {:#?}", rect1); // different format
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    /* dbg! macro */
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    /* Methods */
    let rect1 = Rectangle2 {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle2 {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle2 {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle2::square(3);

    /* Ownership */
    let other_rect = Rectangle2 { width: 1, height: 1 };
    let max_rect = rect1.max(other_rect); // the ownership is moved to max_rect

    let mut rect = Rectangle2 {
        width: 0,
        height: 0
    };
    rect.set_width(1); 
}
