#[derive(Debug, PartialEq)]
pub struct Shoe {
    size: i32,
}

pub fn shoes_size(shoes: Vec<Shoe>, size: i32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_shoes() {
        let shoes = vec![
            Shoe { size: 10 },
            Shoe { size: 2 },
            Shoe { size: 8 },
            Shoe { size: 10 },
            Shoe { size: 7 },
            Shoe { size: 9 },
            Shoe { size: 6 },
        ];

        assert_eq!(
            vec![Shoe { size: 10 }, Shoe { size: 10 },],
            shoes_size(shoes, 10)
        );
    }

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut iter = v1.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1: i32 = vec![1, 2, 3, 4].iter().sum();
        // let iter = v1.iter();
        // let total: i32 = iter.sum();
        // assert_eq!(total, 10);

        assert_eq!(v1, 10);
    }
}
