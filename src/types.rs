use std::vec::Vec;

struct _User<'a> {
    id: &'a str
}
#[derive(Debug)]
enum Entry {
    In,
    Out
}
type ItemId = String;
#[derive(Debug)]
struct Item {
    id: ItemId,
    kind: Entry,
    amount: u32,
    description: Option<String>,
    date: String
}

trait Sum {
    fn extract(&self) -> i64;
    fn empty() -> u8;
}

impl Sum for Item {
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


impl Item {
    pub fn new(entry: Entry) -> Item {
        Item {
            amount: 0,
            description: None,
            date: "Now".to_string(),
            id: "ID".to_string(),
            kind: entry
        }
    }
    pub fn amount(mut self, amount: u32) -> Self {
        self.amount = amount;
        self
    }
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
    pub fn date(mut self, date: &str) -> Self {
        self.date = date.to_string();
        self
    }
}
struct Ledger {
    entries: Vec<Item>
}

impl Ledger {
    fn get_total(&self) -> i64 {
        self.entries.iter().map(|item| { item.extract() }).sum()
    }
}


pub fn run(){
    let input = Item::new(Entry::In).amount(17200).description("Some Input");
    let output = Item::new(Entry::Out).amount(200).description("Some Output");
    let ledger = Ledger {
        entries: vec![input, output]
    };
    println!("{:?}", ledger.entries);
    println!("Platita {:?}", ledger.get_total());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn build_item_without_description() {

    }
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
}