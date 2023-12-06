#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn print(&self) {
        dbg!(self);
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.print();

    let m = Message::Move { x: 10, y: 20 };
    m.print();

    let m = Message::ChangeColor(255, 255, 255);
    m.print();

    let m = Message::Quit;
    m.print();

    let x: i8 = 10;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // cannot add
    let coin = Coin::Quarter;
    let cents = value_in_cents(&coin);

    println!("Coin {:#?} value in cents is {}", coin, cents);

    let config_max: Option<i32> = None; // Some(5)

    let mut count = 0;

    if let Some(max) = config_max {
        println!("config max is {}", max)
    } else {
        count += 1;
    }

    println!("first count is {}", count);

    match Some(config_max) {
        Some(max) => println!("config max {:?}", max),
        _ => count += 1,
    }

    println!("second count is {}", count);
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
