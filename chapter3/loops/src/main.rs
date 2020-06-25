fn main() {
    let a = [10,20,30,40,50]; 
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Value of counter is: {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!");

    for num in (1..4).rev() {
        println!("{}!", num);
    }
    println!("LIFTOFF!");

    let mut index = 0;

    while index < 5 {
        println!("The value at index {} is: {}.", index, a[index]);
        index += 1; 
    }

    for element in a.iter() {
        println!("the value is {}.", element);
    }
}
