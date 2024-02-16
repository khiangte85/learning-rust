use std::ops::Deref;

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(t: T) -> MyBox<T> {
        MyBox(t)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_ref_dereferencing() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn box_ref_dereferencing() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn custom_box_ref_dereferencing() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}

// RefCell<T>
pub trait Messenger {
    fn send(&self, message: &str) {}
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage = self.value as f64 / self.max as f64;

        if percentage >= 1.0 {
            self.messenger.send("You have reach quota limit");
        } else if percentage >= 0.9 {
            self.messenger.send("You have use 90% of your quota");
        } else if percentage >= 0.75 {
            self.messenger.send("You gave used 75% if your qouta")
        }
    }
}

#[cfg(test)]
mod ref_cell_tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        messages_count: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                messages_count: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.messages_count.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sent_over_75_percent() {
        let messenger = MockMessenger::new();

        let mut tracker = LimitTracker::new(&messenger, 100);

        tracker.set_value(80);

        assert_eq!(1, messenger.messages_count.borrow().len());
    }
}
