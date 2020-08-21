
struct _User {
    id: String
}

#[derive(Debug, PartialEq)]
pub enum Entry {
    In,
    Out
}
type ItemId = String;
#[derive(Debug)]
pub struct Item {
    pub id: ItemId,
    pub kind: Entry,
    pub amount: u32,
    pub description: Option<String>,
    pub date: String
}


pub struct Ledger {
    pub entries: Vec<Item>
}