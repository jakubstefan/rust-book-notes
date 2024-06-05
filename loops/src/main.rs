fn main() {
    // Repeating Code with loop
    {
        let mut counter = 0;

        loop {
            println!("again!");
            counter += 1;

            if counter == 10 {
                break;
            }
        }
    }
    // Returning Values from Loops
    {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {result}"); // 20
    }
    // Loop Labels to Disambiguate Between Multiple Loops
    {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;
    
            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }
    
            count += 1;
        }
        println!("End count = {count}"); // 2
    }
    {
        let mut x = 0;
        'a: loop {
            x += 1;
            'b: loop {
                if x > 10 {
                    continue 'a;
                } else {
                    break 'b;
                }
            }
            break;
        }
    }
    // Conditional Loops with while
    {
        let mut number = 3;

        while number != 0 {
            println!("{number}!");
    
            number -= 1;
        }
    
        println!("LIFTOFF!!!");
    }
    // Looping Through a Collection with for
    {
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;
        while index < 5 {
            println!("the value is: {}", a[index]);
            index += 1;
        }
    }
    {
        let a = [10, 20, 30, 40, 50];
        for element in a {
            println!("the value is: {element}");
        }
    }
    {
        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }
}
