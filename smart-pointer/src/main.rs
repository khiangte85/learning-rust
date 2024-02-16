use crate::List::{Cons, Nil};
use std::rc::Rc;

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct CustomerSmartPointer {
    data: String,
}

impl Drop for CustomerSmartPointer {
    fn drop(&mut self) {
        println!("drop customer smart pointer with data {}", self.data);
    }
}

fn main() {
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    println!("count after creating a = {}", Rc::strong_count(&a));

    let _b = Cons(6, Rc::clone(&a));

    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let _c = Cons(7, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let c1 = CustomerSmartPointer {
        data: String::from("Customer 1"),
    };

    let _c2 = CustomerSmartPointer {
        data: String::from("Customer 2"),
    };

    println!("Customer smart pointer created");

    // this isn't allowed
    // c1.drop();

    // Early drop
    drop(c1);


    let mut x = 5;

    let _y = &mut x;
}
