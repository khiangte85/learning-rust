fn main() {
    let num = 6;

    if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 4, 3, or 2");
    }

    let mut count = 0;

    count = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };

    println!("The count is {}", count);

    let mut count = 1;

    'counting_up: loop {
        // println!("The count is {}", count);

        let mut remaining = 10;

        loop {
            println!("The count and remaining is {} and {}", count, remaining);

            if remaining == 2 {
                count += 1;
                println!("skip remaining {remaining}");
                break;
            }

            if count == remaining {
                println!("count reached remaining {} = {}", count, remaining);
                break 'counting_up;
            }

            remaining -= 1;
        }
    }

    println!("End count is {}", count);

    let number = [23, 45, 3, 56, 34];

    
    let mut i = 0;
    
    println!("Array length is: {}", number.len());
    while i < number.len() {
        println!("Array {i} element is: {}", number[i]);
        
        i += 1;
    }
    
    // number.reverse();

    for element  in number.iter().rev()  {
        println!("Array element is: {}", element);
    }

    println!("Array is: {:?}", number);


    for number in (0..=5).rev() {
        println!("{}!", number);
    }
}
