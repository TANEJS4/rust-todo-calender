use core::fmt;

use chrono ::{DateTime, Local};
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct Todoitem<'a>{
 pub id: u32,
 pub title: &'a str,
 pub date_created : DateTime<Local>,
 pub is_completd: bool
}

impl<'a> Default for Todoitem<'a> {
    fn default() ->  Todoitem<'a> {
        Todoitem {
            id: 0,
            title: "hellow",
            date_created: Local::now(),
            is_completd : false
        }
    }
}

impl fmt::Display for Todoitem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id:{} \n title: {}\n date: {} ", self.id, self.title, self.date_created )
    }
}