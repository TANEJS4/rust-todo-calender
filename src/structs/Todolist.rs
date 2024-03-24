// use crate::structs::Todoitem as Todoitem;
use crate::structs::Todoitem::Todoitem;

// mod Todoitem;
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct TodoList<'a> {
    pub items: Vec<Todoitem<'a>>,
}

fn _inc(mut x: u32) -> u32 {
    x = x + 1;
    x
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

    pub fn update(&mut self, id:u32, new_item: Todoitem<'a>){
        self.delete_item(id);
        self.add_item(new_item);
        print!("Item updated");
    }
}
