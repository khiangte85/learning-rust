fn main() {
    let y: i32 = plus_number(30);

    println!("The value of y is: {}", y);
}

fn plus_number(x: i32) -> i32 {
    println!("The value of x is: {}", x);
    x + 1
}
