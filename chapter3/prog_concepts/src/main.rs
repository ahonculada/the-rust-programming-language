fn main() {
    // Variables and Mutability
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6; 

    println!("The value of x is: {}", x);

   // Shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
      
    println!("The value of x is: {}", x);
    
    // Functions and parameters
    another_function(5,6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
