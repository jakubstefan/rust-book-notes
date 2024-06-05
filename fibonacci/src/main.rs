use std::io;

fn main() {
    println!("Generate the nth Fibonacci number.");

    loop {
        println!("Input n:");
        let mut n = String::new();
    
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");
    
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type an unsigned number!");
                continue;
            }
        };
    
        let mut f0 = 0;
        let mut f1 = 1;
        let mut res = 0;
    
        for _ in 0..n - 1 {
            res = f0 + f1;
            f0 = f1;
            f1 = res;
        }
    
        println!("The Fibonacci number of order {n} is {res}");
    }
}
