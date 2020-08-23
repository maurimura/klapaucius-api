
use std::vec::Vec;
use crate::types::{Ledger, Item, Entry};
use crate::item::{Sum};

impl Ledger {
    
    pub fn new(entries: Vec<Item>) -> Self {
        Ledger {
            user_id: "0".to_string(),
            entries        
        }
    }

    pub fn add(mut self, item: Item) -> Self {
        self.entries.push(item);
        self
    }

    pub fn update(mut self, updated_item: Item) -> Self {
        self.entries = self.entries.into_iter().map(|entry| {
            if entry.id == updated_item.id {
                updated_item.clone()
            } else {
                entry
            }
        }).collect();
        
        self
    }

    pub fn delete(mut self, item_to_delete: Item) -> Self {
        self.entries = self.entries
                        .into_iter()
                        .filter(|item| {
                            item.id != item_to_delete.id                               
                        }).collect();
        self
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
        let ledger = Ledger::new(vec![]);
        assert_eq!(ledger.get_total(), 0);
        assert_eq!(ledger.entries.len(), 0);

        let mocked_amount = 420;
        let mocked_item = Item::new(Entry::In).amount(mocked_amount);
        let ledger = ledger.add(mocked_item.clone());
        let expected_id = mocked_item.id.clone();
        let expected_amount = mocked_item.amount;

        assert_eq!(ledger.entries[0].id, expected_id);
        assert_eq!(ledger.entries[0].amount, expected_amount);
    }
    #[test]
    fn update(){
        let mocked_item = Item::new(Entry::In).amount(420);
        let mocked_item_out = Item::new(Entry::Out).amount(20);
        let ledger = Ledger::new(vec![mocked_item, mocked_item_out.clone()]);
        assert_eq!(ledger.get_total(), 400);

        let updated_item = mocked_item_out.amount(200);
        let ledger = ledger.update(updated_item.clone());

        assert_eq!(ledger.get_total(), 220);

        ledger.entries.iter().for_each(|item| {
            if item.id == updated_item.id {
                assert_eq!(item.amount, 200)
            }
        })
    }

    #[test]
    fn delete(){
        let mocked_item = Item::new(Entry::In).amount(420);
        let mocked_item_out = Item::new(Entry::Out).amount(20);
        let ledger = Ledger::new(vec![mocked_item.clone(), mocked_item_out]);

        let ledger = ledger.delete(mocked_item.clone());

        assert_eq!(ledger.get_total(), -20);

        ledger.entries.iter().for_each(|item| {
            assert_ne!(item.id, mocked_item.id)
        })
    }
    #[test]
    fn sum() {
        let mocked_input = vec![
            Item::new(Entry::In).amount(4200).description("Some Input"),
            Item::new(Entry::Out).amount(200).description("Some Output"),
            Item::new(Entry::In).amount(2000).description("Some Input"),
        ];

        let ledger = Ledger {
            user_id: "0".to_string(),
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
            user_id: "0".to_string(),
            entries: mocked_input
        };

        let expect = -200;
        assert_eq!(ledger.get_total(), expect);
    }
}
