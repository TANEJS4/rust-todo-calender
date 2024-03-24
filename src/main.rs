mod structs;
use structs::Todoitem::Todoitem as Todoitem;
use structs::Todolist::TodoList as TodoList;
mod utils;
// use std::io::{stdin, stdout, stderr};

fn main() {
    println!("Hello, world!");
    let mut todo_list = TodoList::new(); 
    let item: Todoitem<'_> = Todoitem{
        id:  utils::get_random_id(),
        title: "helloworld",
        ..Default::default()
    };
    todo_list.add_item(item);

     todo_list.list_items();
}
