#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = vec![1,2,3,4];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("v = {:?}", v);

    let third: &i32 = &v[2];
    println!("The third element of v is: {}", third);

    match v.get(3) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let first = &v[0];
    //v.push(6);

    println!("The first element of v is: {}", first);

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue")),
    ];
    println!("row = {:?}", row);
}
