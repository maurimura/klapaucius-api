
struct _User {
    id: String
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Entry {
    In,
    Out
}
type ItemId = usize;
#[derive(Debug, Clone)]
pub struct Item {
    pub id: ItemId,
    pub kind: Entry,
    pub amount: u32,
    pub description: Option<String>,
    pub date: String
}

#[derive(Clone)]
pub struct Ledger {
    pub entries: Vec<Item>
}