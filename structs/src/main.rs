#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    address: String,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.area() > rect.area()
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let user1 = User {
        username: String::from("khiangte"),
        email: String::from("khiangte85@gmail.com"),
        address: String::from("Aizawl"),
        active: true,
    };

    let user2 = User {
        username: String::from("awmtea"),
        ..user1
    };

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 30,
        height: 45,
    };

    let square = Rectangle::square(40);

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));
    println!("rect can hold square: {}", rect.can_hold(&square));

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let rect1 = Rectangle {
        width: 0,
        height: 50,
    };

    if rect1.width() {
        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    } else {
        println!("Invalid width");
    }

    // dbg!(&rect);
    // dbg!(&user1); // error because partial move of address String which doesn't implement Copy trait
    // dbg!(&user2);
}
