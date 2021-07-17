///////////////////////////////////////////////////////////////////
//// Rules of Reference
/////////////////////////////////////////////////////////////////
// 1) At any time you can have either of the following, not both
//      x) One mutable reference
//      x) Any number of immutable references
// 2) References must always be valid


fn main() {
    // clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
   
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


////////////////////////////////////////////////////////////////
// Transferring ownership of return values
////////////////////////////////////////////////////////////////

    // gives_ownership moves its return value into s1
    let _s1 = gives_ownership();

    // s2 comes into scope
    let s2 = String::from("hello");

    // s2 is moved into takes_and_gives_back, which
    // also moves its return value into s3
    let _s3 = takes_and_gives_back(s2);

//    println!("s2 is: {}", s2);


///////////////////////////////////////////////////////////////
//// References and Borrowing
///////////////////////////////////////////////////////////////

    let s1 = String::from("hello");
    // Pass by reference 
    let len = calculate_length(&s1);
    println!("The length of {} is: {}", s1, len);    

//////////////////////////////////////////////////////////////
//// Mutable References
/////////////////////////////////////////////////////////////

    let mut s = String::from("hello");
    change(&mut s);

    // Possible Data Race
    let mut r1 = &mut s;
//    let mut r2 = &mut s;
    
    // By having two mutable references to s in same 
    // scope, this can lead to a data race.
//    println!("{}, {}", r1, r2);
    
////////////////////////////////////////////////////////////
//// Dangling References
/////////////////////////////////////////////////////////////

    //let reference_to_nothing = dangle();
}


//fn dangle() -> &String {                // returns a reference to a String
//    let s = String::from("Hello");      // s is a new String
//
//    &s                                  // we return a reference to string s
//                // after the bracket s goes out of scope and is droped 
//                // It's memory goes away. DANGER!
//                // Solution: dangle() should return a string, not a ref
//}



fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}


fn calculate_length(s: &String) -> usize {
    s.len()
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
