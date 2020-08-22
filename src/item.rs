

use crate::types::{Item, Entry};

impl <'a>Item<'a> {
    pub fn new(entry: Entry) -> Item<'a> {
        Item {
            amount: 0,
            description: None,
            date: "Now",
            id: "ID",
            kind: entry
        }
    }
    pub fn amount(mut self, amount: u32) -> Self {
        self.amount = amount;
        self
    }
    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }
    pub fn date(mut self, date: &'a str) -> Self {
        self.date = date;
        self
    }
}

pub trait Sum {
    fn extract(&self) -> i64;
    fn empty() -> u8;
}

impl Sum for Item<'_> {
    fn extract(&self) -> i64 {
        match &self.kind {
            Entry::In => self.amount as i64,
            Entry::Out => -(self.amount as i64)
        }
    }

    fn empty() -> u8 { 
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_item_builder() {
        let mocked_item = Item::new(Entry::In);

        let expected_item = Item {
            id: mocked_item.id,
            kind: Entry::In,
            date: "Now",
            amount: 0,
            description: None
        };

        assert_eq!(mocked_item.date, expected_item.date);
        assert_eq!(mocked_item.amount, expected_item.amount);
        assert_eq!(mocked_item.description, expected_item.description);
        assert_eq!(mocked_item.kind, expected_item.kind);
    }

    #[test]
    fn builded_item() {
        let mocked_date = "Some date";
        let mocked_amount = 4200;
        let mocked_description = "Some desc";
        let mocked_item = Item::new(Entry::Out)
                                .date(mocked_date)
                                .amount(mocked_amount)
                                .description(mocked_description);
        

        let expected_item = Item {
            id: mocked_item.id,
            kind: Entry::Out,
            date: mocked_date,
            amount: mocked_amount,
            description: Some(mocked_description)
        };

        assert_eq!(mocked_item.date, expected_item.date);
        assert_eq!(mocked_item.amount, expected_item.amount);
        assert_eq!(mocked_item.description, expected_item.description);
        assert_eq!(mocked_item.kind, expected_item.kind);
    }
}