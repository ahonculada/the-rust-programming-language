enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let p = Coin::Penny;
    println!("The value of our coin is: {}", value_in_cents(p));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

