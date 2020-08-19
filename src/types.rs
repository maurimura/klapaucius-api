use std::vec::Vec;

struct _User {
    id: String
}
#[derive(Debug)]
enum Entry {
    In,
    Out
}
#[derive(Debug)]
struct Item {
    kind: Entry,
    amount: u32,
    description: Option<String>
}

trait Sum {
    fn concat(&self) -> i64;
    fn empty() -> u8;
}

impl Sum for Item {
    fn concat(&self) -> i64 {
        match &self.kind {
            Entry::In => self.amount as i64,
            Entry::Out => -(self.amount as i64)
        }
    }

    fn empty() -> u8 { 
        0
    }
}

struct Ledger {
    entries: Vec<Item>
}

impl Ledger {
    fn get_total(&self) -> i64 {
        self.entries.iter().map(|item| { item.concat() }).sum()
    }
}

pub fn run(){
    let input = Item {
        kind: Entry::In,
        amount: 17200,
        description: Some("Some Input".to_string())
    };

    let output = Item {
        kind: Entry::Out,
        amount: 200,
        description: Some("Some Ouput".to_string())
    };
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
    fn sum() {
        let mocked_input = vec![
            Item {
                kind: Entry::In,
                amount: 4200,
                description: Some("Falopa".to_string())
            },
            Item {
                kind: Entry::Out,
                amount: 200,
                description: Some("Some Ouput".to_string())
            },
            Item {
                kind: Entry::In,
                amount: 2000,
                description: Some("Some Input".to_string())
            },
        ];

        let ledger = Ledger {
            entries: mocked_input
        };

        let expect = 6000;
        assert_eq!(ledger.get_total(), expect);
    }
}