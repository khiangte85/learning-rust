struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_number<'a, T>(x: &'a T, y: &'a T) -> T
where
    &'a T: std::ops::Add<&'a T, Output = T>,
{
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 30.0,
            height: 40.0,
        };

        let smaller = Rectangle {
            width: 20.0,
            height: 30.0,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let smaller = Rectangle {
            width: 20.0,
            height: 30.0,
        };

        let larger = Rectangle {
            width: 30.0,
            height: 40.0,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_add_two_number() {
        let a = 25;
        let b = 30;

        assert_eq!(55, add_number(&a, &b));
    }
}
