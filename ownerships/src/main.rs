fn main() {
    // assignment works fine as integer implements Copy trait
    // Copy trait is implemented for all integer types, float types, boolean, char,
    // and tuples if they only contain types that also implement Copy
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // String type is stored in heap and has no Copy trait
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("tests");

    // needs to deep copy to work as String doesn't implement Copy trait
    // if not deep copied, s1 will be moved to s2 and s1 will be invalid
    let s2 = s1.clone();

    println!("{}", s1);
    println!("{}", s2);

    // takes ownership
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // will throw error as s is no longer valid

    // makes copy
    let x = 5;
    makes_copy(x);
    println!("Still works {}", x); // works fine as x is i32 and implements Copy trait

    // return values and scope
    let s1 = gives_ownership();
    println!("{}", s1);

    let s2 = String::from("take and give back");
    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);

    // References and Borrowing
    let s1 = String::from("references");
    let len = calculate_length(&s1);
    println!("The length of `{}` is {}", s1, len);

    // Mutable References
    let mut s = String::from("mutable");
    change(&mut s);
    println!("Mutate `{}`", s);

    let mut s1: String = String::from("mutable reference");

    let _r1: &mut String = &mut s1;
    // if uncommented below, r2 mutable reference will be valid as `reference scope` will be ended here
    // println!("{}", r1);

    // will throw error as s1 is already borrowed as mutable unless r1 is out of scope
    let r2: &mut String = &mut s1;
    // println!("{} {}", r1, r2);

    println!("{}", s1);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);

    // String Slice
    let s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // error!
    println!("first word of `{}` is `{}`", s, word);
}

fn takes_ownership(string: String) {
    println!("Take ownership of `{}`", string);
}

fn makes_copy(integer: i32) {
    println!("Makes copy of `{}`", integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("give");
    println!("gives ownership of `{}`", some_string);
    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    println!("take and give back value of `{}`", some_string);
    some_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(" is mutated");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
