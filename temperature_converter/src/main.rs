use std::io;

fn main() {
    loop {
        println!("");
        println!("1) Fahrenheit -> Celsius");
        println!("2) Celsius -> Fahrenheit");
        println!("Please select conversion option:");
    
        let mut option = String::new();
        
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
    
        let option: u32 = match option.trim().parse() {
            Ok(num) => {
                if num >= 1 && num <= 2 { num }
                else {
                    println!("Please type a number between 1 and 2!");
                    continue;
                }
            },
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("Please input temperature in {} to convert:",
            if option == 1 {"°F"} else {"°C"});
        let mut temperature = String::new();
        
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");
    
        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        let conversion;
        if option == 1 { // Fahrenheit -> Celsius
            // Celsius = (Fahrenheit - 32) / 1.8
            conversion = (temperature - 32.0) / 1.8;
        }
        else { //Celsius -> Fahrenheit
            // Fahrenheit = (Celsius * 1.8) + 32
            conversion = (temperature * 1.8) + 32.0;
        }

        println!("=> {temperature} {} = {conversion} {}",
            if option == 1 {"°F"} else {"°C"},
            if option == 1 {"°C"} else {"°F"});
    }
}
