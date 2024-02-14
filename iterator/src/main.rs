
use iterator::{shoes_size};

fn main() {
    let v1 = vec![1, 2, 3];

    let v2: Vec<_> = v1
        .iter()
        .map(|x| if x % 2 == 0 { x * 2 } else { x + 2 })
        .collect();

    println!("{:#?}", v2);
}
