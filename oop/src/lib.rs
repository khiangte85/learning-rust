pub struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}

impl AverageCollection {
    pub fn add(&mut self, num: i32) {
        self.list.push(num);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let item = self.list.pop();

        match item {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod average_collection {
    use super::*;

    #[test]
    fn it_add_item_to_list() {
        let mut avg_collection = AverageCollection {
            list: vec![],
            average: 0.0,
        };

        avg_collection.add(10);
        avg_collection.add(15);

        assert_eq!(2, avg_collection.list.len());
    }

    #[test]
    fn it_remove_item_from_list() {
        let mut avg_collection = AverageCollection {
            list: vec![],
            average: 0.0,
        };

        avg_collection.add(10);
        
        assert_eq!(avg_collection.remove(), Some(10));
        assert_eq!(avg_collection.remove(), None);
    }

    #[test]
    fn it_calculate_average() {
        let mut avg_collection = AverageCollection {
            list: vec![],
            average: 0.0,
        };

        avg_collection.add(15);
        avg_collection.add(20);

        assert_eq!(17.5, avg_collection.average());

        avg_collection.remove();
        avg_collection.add(17);
        avg_collection.add(12);

        assert_eq!(14.67, ((avg_collection.average() * 100.0).round()) / 100.0);
    }
}
