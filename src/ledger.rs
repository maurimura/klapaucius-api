
use std::vec::Vec;
use crate::types::{Ledger, Item, Entry};
use crate::item::{Sum};

impl <'a>Ledger<'a> {
    pub fn new(entries: &'a Vec<Item<'a>>) -> Self {
        Ledger {
            entries        
        }
    }

    pub fn add(mut self, item: Item<'a>) -> Self {
        let mut new_entries = self.entries.clone();
        new_entries.push(item);
        self.entries = &new_entries;
        self
    }
    
    pub fn update(mut self, updated_item: Item) {
        self.entries = self.entries 
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
        let ledger = Ledger::new(&vec![]);
        assert_eq!(ledger.get_total(), 0);
        assert_eq!(ledger.entries.len(), 0);

        let mocked_amount = 420;
        let mocked_item = Item::new(Entry::In).amount(mocked_amount);
        let expected_id = mocked_item.id.clone();
        let expected_amount = mocked_item.amount;
        ledger.add(mocked_item);
        let expected_item = ledger.entries[0];
        assert_eq!(expected_item.id, expected_id);
        assert_eq!(expected_item.amount, expected_amount);
        assert_eq!(ledger.get_total(), expected_amount as i64);
    }
    #[test]
    fn update(){
        let mocked_item = Item::new(Entry::In).amount(420);
        let mut ledger = Ledger::new(&vec![mocked_item]);
    }

    #[test]
    fn sum() {
        let mocked_input = &vec![
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
        let mocked_input = &vec![
            Item::new(Entry::Out).amount(200).description("Some Output"),
        ];

        let ledger = Ledger {
            entries: mocked_input
        };

        let expect = -200;
        assert_eq!(ledger.get_total(), expect);
    }
}
