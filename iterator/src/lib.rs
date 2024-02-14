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
}
