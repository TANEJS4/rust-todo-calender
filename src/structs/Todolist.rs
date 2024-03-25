use chrono::{DateTime, Local};
use crate::structs::enums as Types;
use crate::structs::Todoitem::Todoitem;
use crate::utils::_inc as _inc;

#[allow(non_snake_case,dead_code)]
// #[derive(Serialize, Deserialize)]
pub struct TodoList<'a> {
    pub items: Vec<Todoitem<'a>>,
}



impl<'a>  FromIterator<Todoitem<'a>> for TodoList<'a>  {
    fn from_iter<T: IntoIterator<Item = Todoitem<'a>> >  (iter: T) -> Self {
        let mut c =  TodoList { items: Vec::new() };
        for i in iter {
            c.add_item(i)
        }
        c
    }
}
/// id -> id of Todoitem
/// idx -> index of TodoList
#[allow(dead_code)]
impl<'a> TodoList<'a> {
    pub fn new() -> TodoList<'a> {
        TodoList { items: Vec::new() }
    }
    pub fn add_item(&mut self, item: Todoitem<'a>) {
        let new_item: Todoitem<'_> = item;
        println!("Added: {}", new_item.title);
        self.items.push(new_item);
    }

    pub fn list_items(&self) {
        if self.items.is_empty() {
            println!("Your to-do list is empty.");
        } else {
            println!("Your to-do list:");
            for item in &self.items {
                let status = if item.is_completd { "[x]" } else { "[ ]" };
                println!("{} {} - {}", status, item.id, item.title);
            }
        }
    }

    pub fn delete_item(&mut self, id: u32) {
        let mut idx = 0;
        for item in &self.items {
            idx = if item.id == id { idx } else { _inc(idx) };
        }
        self.items.remove(idx.try_into().unwrap());
    }

    pub fn get(&self, idx: usize) -> Option<&Todoitem<'_>> {
        let vec = self.items.get(idx);
        vec
    }

    pub fn update(&mut self, id: u32, new_item: Todoitem<'a>) {
        self.delete_item(id);
        self.add_item(new_item);
        print!("Item updated");
    }
    pub fn filter(
        &mut self,
        on: &str,
        target: Option<Types::Status>,
        key: Option<DateTime<Local>>,
    ) -> Option<TodoList<'a>> {
        match on {
            "status" => {
                print!("status based filtering");
                Some(self.helperFilterStatus(target.unwrap()))
            }
            "date" => {
                print!("due date based filtering");
                Some(self.helperFilterDate(key.unwrap()))
            }
            _ => {
                print!("Not valid, use 'status' or 'date'");
                None
            }
        }
    }
    fn helperFilterStatus(&mut self, target: Types::Status) ->TodoList<'a>{
         self.items
            .drain(..).filter(|x_item| x_item.status==target).collect()
        
    }
    fn helperFilterDate(&mut self, key: DateTime<Local>) ->TodoList<'a>{
        self.items
        .drain(..).filter(|x_item| x_item.due_date.unwrap()==key).collect()       
    }
}
