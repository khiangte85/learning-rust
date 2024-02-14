

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

fn main() {
    let store  = Inventory{
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    let user1_pref = Some(ShirtColor::Red);
    let user1_got = store.giveaway(user1_pref); 
    println!("User 1 pref is {:?} and gets {:?}", user1_pref, user1_got);

    let user2_pref = None;
    let user2_got = store.giveaway(user2_pref);
    println!("User 2 pref is {:?} and gets {:?}", user2_pref, user2_got);
}
