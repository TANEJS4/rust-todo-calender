mod structs;
use structs::Todoitem as Todoitem;
mod utils;
// use std::io::{stdin, stdout, stderr};

struct TodoList<'a> {
    items: Vec<Todoitem::Todoitem<'a>>
}

impl<'a> TodoList<'a> {
    fn new() -> TodoList<'a> {
            TodoList{ 
                items:Vec::new()
            }
    }
    pub fn add_item(&mut self, item:Todoitem::Todoitem<'a>){
            let new_item: Todoitem::Todoitem<'_> =  item;
            println!("Added: {}", item.title);
            self.items.push(new_item);
    }
    pub fn list_items(&self) {
        if self.items.is_empty() {
            println!("Your to-do list is empty.");
        } else {
            println!("Your to-do list:");
            for item in &self.items {
                let status =  if item.is_completd { "[x]" } else { "[ ]"};
                println!("{} {} - {}",status,  item.id, item.title);
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
    let mut todo_list: TodoList<'_> = TodoList::new(); 
    let item: Todoitem::Todoitem<'_> = Todoitem::Todoitem{
        id:  utils::get_random_id(),
        title: "helloworld",
        ..Default::default()
    };
    todo_list.add_item(item);

     todo_list.list_items();
}
