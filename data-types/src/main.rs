use std::io;

fn main() {
    // Integer
    let x = 5;
    let y: i32 = 10;
    println!("The value of x and y are: {} and {}", x, y);

    // Floating-point
    let x = 2.0;
    let y: f32 = 3.0;
    println!("The value of x and y are: {} and {}", x, y);

    // Addition
    let sum = 5 + 10;

    println!("The value of sum is: {}", sum);

    // Subtraction
    let difference = 95.5 - 4.3;

    println!("The value of difference is: {}", difference);

    // Multiplication
    let product = 4 * 30;

    println!("The value of product is: {}", product);

    // Division
    let quotient = 56.7 / 32.2;

    println!("The value of quotient is: {}", quotient);

    let quotient = -5 / 3;

    println!("The value of quotient is: {}", quotient);

    let quotient = 5.0 / 3.0;

    println!("The value of quotient is: {}", quotient);

    // Remainder
    let remainder = 43 % 5;

    println!("The value of remainder is: {}", remainder);

    // Boolean
    let x = true;

    let y: bool = false;

    println!("Boolean values are {} and {}", x, y);

    // Character
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("Character values are {}, {}, and {}", c, z, heart_eyed_cat);

    // Tuple
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple;
    println!("The value of x, y, and z are: {}, {}, and {}", x, y, z);

    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;
    println!("The value of x, y, and z are: {}, {}, and {}", x, y, z);

    // Array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);

    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("The value of months is: {:?}", months);

    println!("The first month of the year is: {:?}", months[0]);

    println!("Enter an array index: ");

    let mut index: String = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = months[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
