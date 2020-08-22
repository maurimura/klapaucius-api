

mod types;
mod item;
mod ledger;
use types::{Ledger, Item, Entry};

fn main() {
    let input = Item::new(Entry::In).amount(200);
    let output = Item::new(Entry::Out).amount(400);
    let entries = vec![input, output];
    let ledger = Ledger::new(&entries);
    println!("Entries {:?}", ledger.entries);
    println!("Total {:?}", ledger.get_total());
}
