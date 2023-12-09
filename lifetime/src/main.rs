fn main() {
    let r;

    // x goes out of scope, so r points to object no longer exist
    // {
    //     let x = 5;
    //     r = &x;
    // }

    // this works
    let x = 5;
    r = &x;

    println!("{}", r);

    let first = String::from("hello");
    let second = "rust";

    let result = longest(&first.as_str(), &second);
    println!("Longest word is: {}", result);

    {
        let second = String::from("second");
        let result = longest(first.as_str(), second.as_str());
        println!("Longest word is: {}", result);
    }

    let x = String::from("hello");
    let y = String::from("world");

    let result1 = longest1(x, y);
    println!("Longest word is: {}", result1);
}

struct Excerpt<'a> {
    part: &'a str,
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//  without lifetime
fn longest1(x: String, y: String) -> String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
