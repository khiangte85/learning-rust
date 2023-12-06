
#[derive(Debug)]
pub struct Breakfast {
    pub main: String,
    topping: String,
}

pub enum Appetizer {
    Soup,
    Salad,
}

impl Breakfast {
    pub fn new(main: String) -> Self {
        Self {
            main,
            topping: String::from("Egg"),
        }
    }
}
