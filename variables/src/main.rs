fn main() {
    // Variables are immutable by default
    let  x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Variables are mutable if declared as such
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    // Constants are always immutable
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // Shadowing
    let z = 5;

    println!("The value of z is: {}", z);

    {
        let z = z * 2;
        println!("The value of z in inner scope is :  {}", z);
    }

    let z = z + 1;

    println!("The value of z after shadowing is: {}", z);


    // Shadowing vs mutability
    let spaces = "     ";

    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces);

    let mut spaces = "     ";

    spaces = spaces.len();

    println!("The value of spaces is: {}", spaces);
}
