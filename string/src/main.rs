fn main() {
    let mut a = String::new();

    a = "mutate and add content to string".to_string();

    let b = " and initial content".to_string();

    let mut c = String::from(" and string from");

    c.push_str(" and push to string");

    println!("string: {}, {}, {}", a, b, c);

    let mut s1 = String::from("tes");
    let s2 = "t";
    s1.push_str(s2);
    println!("S1 is {s1}");
    println!("s2 is {s2}");

    let s1 = String::from("s1 concat");
    let s2 = String::from("s2");
    let s3 = s1 + &s2;

    println!("string concat: {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = s1 + &s2 + &s3;

    println!("string concat: {}", s4);

    let s1 = String::from("tic"); // shadowing s1 as it already moved in previous concat op

    let s5 = format!("{s1}{s2}{s3}");

    println!("string concat: {s5}");

    let s6 = &s5[0..1];

    println!("{s6}");

    let s7 = "Здравствуйте"; 
    let s8 = &s7[0..2];
    println!("{s8}");

    for c in s7.chars(){
        println!("{c}");
    }
}
