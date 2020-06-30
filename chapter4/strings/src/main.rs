fn main() {
    // clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}", s2);
   
    // copy
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");
    
    // after function 's' is no longer valid
    takes_ownership(s);

    let x = 5;
    
    // after function 'x' is still valid
    makes_copy(x);
    
    // gives_ownership moves its return value into s1
    let _s1 = gives_ownership();

    // s2 comes into scope
    let s2 = String::from("hello");

    // s2 is moved into takes_and_gives_back, which
    // also moves its return value into s3
    let _s3 = takes_and_gives_back(s2);

//    println!("s2 is: {}", s2);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello!");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
