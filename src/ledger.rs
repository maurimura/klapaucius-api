
use std::vec::Vec;
use crate::types::{Ledger, Item, Entry};
use crate::item::{Sum};

impl Ledger {
    pub fn new(entries: Vec<Item>) -> Self {
        Ledger {
            entries
        }
    }

    pub fn add(mut self, item: Item) -> Self {
        self.entries.push(item);
        self
    }
    
    pub fn update(mut self, updated_item: Item) {
        // for 
    }

    pub fn get_total(&self) -> i64 {
        self.entries.iter().map(|item| { item.extract() }).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add(){
        let mut ledger = Ledger::new(vec![]);
        assert_eq!(ledger.get_total(), 0);
        assert_eq!(ledger.entries.len(), 0);

        let mocked_amount = 420;
        let mocked_item = Item::new(Entry::In).amount(mocked_amount);
        let expected_id = mocked_item.id.clone();
        let expected_amount = mocked_item.amount;
        ledger.add(mocked_item);

        assert_eq!(ledger.entries.len(), 1);
        assert_eq!(ledger.entries[0].id, expected_id);
        assert_eq!(ledger.entries[0].amount, expected_amount);
        assert_eq!(ledger.get_total(), expected_amount as i64);
    }
    #[test]
    fn update(){
        let mocked_item = Item::new(Entry::In).amount(420);
        let mut ledger = Ledger::new(vec![mocked_item]);
    }

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
