
struct _User {
    id: String
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Entry {
    In,
    Out
}
type ItemId = str;
#[derive(Debug, Copy, Clone)]
pub struct Item<'a> {
    pub id: &'a ItemId,
    pub kind: Entry,
    pub amount: u32,
    pub description: Option<&'a str>,
    pub date: &'a str
}

#[derive(Copy, Clone)]
pub struct Ledger<'a> {
    pub entries: &'a Vec<Item<'a>>
}