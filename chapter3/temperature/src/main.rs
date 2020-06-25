use std::io;

fn main() {
    println!("Welcome to temperature conversion. Press Q anytime to quit.");
    
    loop {
        println!("Type F for celsius to fahrenheit. Type C for fahrenheit to celsius.");

        let mut letter = String::new();

        let mut temp = String::new();

        io::stdin().read_line(&mut letter)
            .expect("Failed to recognize input.");
        
        let letter = letter.trim();

        if letter == "Q" || letter == "q" {
            break;
        }

        println!("Type value of temperature.");

        io::stdin().read_line(&mut temp)
            .expect("Failed to recognize input.");
       
        let temp = temp.trim();

        if temp == "Q" || temp == "q" {
            break;
        }

        let temp = match temp.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        if letter == "F" || letter == "f" {
            let result: f32 = celsius_to_fahrenheit(temp);
            println!("{} degrees Celsius is {} degrees Fahrenheit.", temp, result);
        } else if letter == "C" || letter == "c" {
            let result: f32 = fahrenheit_to_celsius(temp);
            println!("{} degrees Fahrenheit is {} degrees Celsius.", temp, result);
        } else if letter == "Q" || letter == "q" {
            break;
        } else {
            println!("Improper input. Please start again.");
            continue;
        }
    }
}


fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0 
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    celsius * (9.0 / 5.0) + 32.0
}
    
