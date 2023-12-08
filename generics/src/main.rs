fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct MixPoint<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T: Clone, U> MixPoint<T, U> {
    fn mix_points<V, W: Clone>(&self, other: &MixPoint<V, W>) -> MixPoint<T, W> {
        MixPoint {
            x: self.x.clone(),
            y: other.y.clone(),
        }
    }
}

fn main() {
    let numbers = vec![2, 5, 99, 34, 0];
    let largest = find_largest(&numbers);
    println!("Largest number is: {}", largest);

    let mut chars = vec!['z', 'c', 'd', 's'];
    let largest = find_largest(&mut chars);
    println!("Largest character is: {}", largest);

    let p1 = Point { x: 20, y: 40 };
    println!("x value is: {}", p1.x());
    println!("Point value is: {:?}", p1);

    let p2 = MixPoint { x: 12, y: 5.9 };
    let p3 = MixPoint { x: "x", y: "x" };
    let p4 = p2.mix_points(&p3);

    println!("p2: {:?}", p2);
    println!("p3: {:?}", p3);
    println!("p4: {:?}", p4);
}
