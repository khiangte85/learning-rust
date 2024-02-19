use std::{fmt::Display, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Overload Add trait
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Default type

#[derive(Debug)]
struct MilliMeters(u32);
struct Meters(u32);

impl Add<Meters> for MilliMeters {
    type Output = MilliMeters;

    fn add(self, rhs: Meters) -> Self::Output {
        MilliMeters(self.0 + (rhs.0 * 1000))
    }
}

// Same method name
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot fly");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard fly")
    }
}

impl Human {
    fn fly(&self) {
        println!("Human fly")
    }
}

//  Fully qualified name required because method doesn't have self param
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppies")
    }
}

//  Newtype pattern
struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let p1 = Point { x: 10, y: 5 };

    let p2 = Point { x: 5, y: 10 };

    let p3 = p1 + p2;

    assert_eq!(p3, p1 + p2);
    println!("{:#?}", p3);

    let total = MilliMeters(2500) + Meters(3);

    println!("Addition of mm and m: {:?}", total);

    let human = Human;

    human.fly();
    Pilot::fly(&human);
    Wizard::fly(&human);

    println!("My dog name is {}", Dog::baby_name());
    //  fully qualified syntax <Type as Trait>::function_name()
    println!("A baby dog is called {}", <Dog as Animal>::baby_name());

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);

    println!("arr = {}", w);
}
