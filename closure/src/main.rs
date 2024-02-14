use std::thread;



struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_pref: Option<ShirtColor>) -> ShirtColor {
        user_pref.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red = 0;
        let mut blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => red += 1,
                ShirtColor::Blue => blue += 1,
            }
        }

        if red > blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug, Clone, Copy)] // Implement the Clone trait for ShirtColor
enum ShirtColor {
    Red,
    Blue,
}

#[derive(Debug)]
struct Rectangle{
    height: i32,
    width: i32
}


fn main() {
    let store  = Inventory{
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    let user1_pref = Some(ShirtColor::Red);
    let user1_got = store.giveaway(user1_pref); 
    println!("User 1 pref is {:?} and gets {:?}", user1_pref.unwrap(), user1_got);

    let user2_pref = None;
    let user2_got = store.giveaway(user2_pref);
    println!("User 2 pref is {:?} and gets {:?}", user2_pref, user2_got);

    // immutable multiple borrow
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || {
        println!("from thread: {:?}", list);
    }).join().unwrap();

    // let only_borrows = || println!("From closure: {:?}", list);

    // println!("Before calling closure: {:?}", list);
    // only_borrows();
    // println!("After calling closure: {:?}", list);

    // Mutabable multiple borrow is not allowed
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably(); // borrows returned here
    println!("After calling closure: {:?}", list);

    let mut rectangles = vec![
        Rectangle { height: 20, width: 30 },
        Rectangle { height: 30, width: 40 },
        Rectangle { height: 12, width: 10 },
        Rectangle { height: 14, width: 34 },
        Rectangle { height: 28, width: 60 },
    ];

    let mut sort_ops: Vec<String> = vec![];
    let value = String::from("by key called");
    
    let mut num_ops = 0;

    rectangles.sort_by_key(|r| { 
        num_ops += 1;

        // sort_ops.push(value); // not work

        r.width
    });

    println!("{:#?}", rectangles);
}
