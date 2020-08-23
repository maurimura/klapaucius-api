
use serde::{Serialize, Deserialize};
type UserId = String;
struct User {
    id: UserId
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize, )]
pub enum Entry {
    In,
    Out
}
type ItemId = usize;
#[derive(Debug, Clone, Serialize, Deserialize, )]
pub struct Item {
    pub id: ItemId,
    pub kind: Entry,
    pub amount: u32,
    pub description: Option<String>,
    pub date: String
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Ledger {
    pub user_id: UserId,
    pub entries: Vec<Item>
}