use crate::structs::enums as Types;
use chrono::{DateTime, Local};
use core::fmt;

#[allow(non_snake_case)]
pub struct Todoitem<'a> {
    pub id: u32,
    pub title: &'a str,
    //TODO: make this field private but assign default value. might have to desugar it.
    pub _date_created: DateTime<Local>,
    pub date_modified: DateTime<Local>,
    pub is_completd: bool,
    pub due_date: Option<DateTime<Local>>,
    pub status: Option<Types::Status>,
    //TODO: need to link another todoitem but recursive structs are not allowed in rust.
    pub link: Option<u32>,
    pub linkRel: Option<Types::LinkRelation>,
}

impl<'a> Default for Todoitem<'a> {
    fn default() -> Todoitem<'a> {
        Todoitem {
            id: 0,
            title: "hellow",
            _date_created: Local::now(),
            date_modified: Local::now(),
            is_completd: false,
            due_date: None,
            status: Some(Types::Status::Todo),
            link: None,
            linkRel: None,
        }
    }
}

impl fmt::Display for Todoitem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id:{} \n title: {}\n date: {} ",
            self.id, self.title, self.date_modified
        )
    }
}
