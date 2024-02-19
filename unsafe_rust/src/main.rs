fn main() {
    let num = 5;
    let mut num2 = 5;

    // unsafe raw pointer
    let r1 = &num as *const i32;
    let r2 = &mut num2 as *mut i32;

    unsafe {
        println!("r1 is: {:?}", *r1);
        println!("r2 is: {:?}", *r2);
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    (&mut values[..mid], &mut values[mid..])
}
