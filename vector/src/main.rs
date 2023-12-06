fn main() {
    let w = vec![2, 4, 1, 5];
    println!("vector {:#?}", w);

    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(11);
    v.push(12);
    v.push(13);

    println!("vector {:#?}", v);

    v.pop();

    v.push(14);

    println!("vector after pop {:#?}", v);

    let mut pos = 3;
    let mut v1 = v[pos];
    
    println!("{} pos of vector is {}", pos, v1);
    v1 = 20;
    println!("{}", v1);
    println!("{:?}", v);
    
    pos =2;
    let v2 = v.get(pos);

    match v2 {
        Some(v) => println!("{} pos is {}", pos, v),
        None => println!("No value at {}", pos)
    } 
    
    let mut pos = 3;
    let v = [String::from("Jan"), String::from("Feb"), String::from("Mar"), String::from("Apr"), String::from("May")];
    let v1 = &v[pos];
    
    println!("{:?} pos of vector is {}", pos, v1);
    println!("{:?}", v);
    
    pos =2;
    let v2 = v.get(pos);

    match v2 {
        Some(v) => println!("{} month is {}", pos, v),
        None => println!("No value at {}", pos)
    }

    for m in &v {
        println!("{} ", m);
    }

    println!("{:?}", v);
}
