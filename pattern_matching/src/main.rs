struct Point {
    x: i32,
    y: i32,
}

enum Status {
    Pending,
    Confirm,
    Success,
    Error(String),
}

fn main() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("any other number"),
    }

    let x = 5;

    match x {
        1..=5 => println!("one to five"),
        _ => println!("greater than five"),
    }

    let x = 'C';

    match x {
        'a'..='j' => println!("a to j"),
        'k'..='z' => println!("k to z"),
        _ => println!("not match lowercase of alphaber"),
    }

    let p = Point { x: 12, y: 32 };

    // destructure
    let Point { x, y } = p;

    match p {
        Point { x, y: 0 } => println!("On the x axis {x}"),
        Point { x: 0, y } => println!("On the y axis {y}"),
        Point { x, y } => println!("On neither axis"),
    }

    let status = Status::Error(String::from("Invalid status"));

    match status {
        Status::Pending => println!("Pending"),
        Status::Confirm => println!("Confirm"),
        Status::Success => println!("Success"),
        Status::Error(msg) => println!("Something went wrong: {msg}"),
    }

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    struct Point1 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point1 { x: 1, y: 0, z: 0 };

    match origin {
        Point1 { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x = 4;
    let y = true;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 9 };

    match msg {
        Message::Hello {
            id: id_variable @ 1..=10,
        } => println!("Id within 10: {id_variable}"),
        Message::Hello { id } => println!("Some other id found: {id}"),
    }
}
