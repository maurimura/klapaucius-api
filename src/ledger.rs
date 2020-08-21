
use std::vec::Vec;
use crate::types::{Ledger, Item, Entry};
use crate::item::{Sum};

impl Ledger {
    pub fn new(entries: Vec<Item>) -> Self {
        Ledger {
            entries
        }
    }
    pub fn get_total(&self) -> i64 {
        self.entries.iter().map(|item| { item.extract() }).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum() {
        let mocked_input = vec![
            Item::new(Entry::In).amount(4200).description("Some Input"),
            Item::new(Entry::Out).amount(200).description("Some Output"),
            Item::new(Entry::In).amount(2000).description("Some Input"),
        ];

        let ledger = Ledger {
            entries: mocked_input
        };

        let expect = 6000;
        assert_eq!(ledger.get_total(), expect);
    }
    #[test]
    fn negative_sum() {
        let mocked_input = vec![
            Item::new(Entry::Out).amount(200).description("Some Output"),
        ];

        let ledger = Ledger {
            entries: mocked_input
        };

        let expect = -200;
        assert_eq!(ledger.get_total(), expect);
    }
}
