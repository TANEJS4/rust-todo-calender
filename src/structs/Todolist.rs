use crate::structs::Todoitem as Todoitem;

// mod Todoitem;
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct TodoList<'a> {
    items: Vec<Todoitem::Todoitem<'a>>
}

fn _inc (mut x:u32) -> u32 {
    x = x+1;
    x
}
#[allow(dead_code)]
impl<'a> TodoList<'a> {
    pub fn new() -> TodoList<'a> {
            TodoList{ 
                items:Vec::new()
            }
    }
    pub fn add_item(&mut self, item:Todoitem::Todoitem<'a>){
            let new_item: Todoitem::Todoitem<'_> =  item;
            println!("Added: {}", new_item.title);
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
    pub fn delete_item(&mut self, id:u32) {
        let mut idx = 0;
        for item in &self.items {
            idx =  if item.id==id { idx } else { _inc(idx) };
        }
        self.items.remove(idx.try_into().unwrap());
    }
}