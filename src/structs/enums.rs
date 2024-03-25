#[derive(PartialEq)]
#[allow(dead_code)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}
#[derive(PartialEq)]
#[allow(dead_code)]
pub enum LinkRelation {
    BlockedBy,
    LinkedTo,
    DuplicateOf,
    PartOf,
}
