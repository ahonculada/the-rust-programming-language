struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut a = build_user("email@domain.com".to_string(), "dave".to_string());
    println!("dave's email is: {}", a.email);
    let b = User {
        username: "bobby".to_string(),
        ..a
    };
    println!("bobby's email is: {}", b.email);
    a.email = String::from("anotheremail@domain.com");
    println!("bobby's email is: {}", b.email);

    // tuple structs
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    println!("(x,y) coordinate is: ({},{})", origin.0, origin.1);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect1 is: {:#?}", rect1);
    println!("The area of the rectangle is {} square pixels.", 
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(8);

    println!("Area of sq is: {}", sq.area());

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}


