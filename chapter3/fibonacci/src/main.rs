use std::io;

fn main() {
    loop {    
        println!("Which Fibonacci number do you want? Enter an integer.");
        
        let mut nth_digit = String::new();

        io::stdin().read_line(&mut nth_digit)
            .expect("Failed to read number.");
        
        let nth_digit = nth_digit.trim();
        if nth_digit == "Q" || nth_digit == "q" {
            break;
        }

        let nth_digit: usize = match nth_digit.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if nth_digit == 0 || nth_digit == 1 {
            println!("The {} Fibonacci number is: {}.", nth_digit, nth_digit);
            continue;
        }
        let nth_digit = nth_digit + 1;
        let memo = [0,1];
        let mut index: usize = 0;

        while index < nth_digit {
            let idx = index % 2;
            println!("{}", memo[idx]);
            index = index + 1;
        }
        println!("nth digit is: {}.", nth_digit); 
    }
}
