mod structs;
use structs::Todoitem::Todoitem;
use structs::Todolist::TodoList;
mod utils;
// use std::io::{stdin, stdout, stderr};

fn main() {
    println!("Hello, world!");
    let mut todo_list = TodoList::new();
    let item: Todoitem<'_> = Todoitem {
        id: utils::get_random_id(),
        title: "helloworld",
        ..Default::default()
    };
    todo_list.add_item(item);
    println!("Printing table - I");
    todo_list.list_items();
    println!("get first idx - II");

    let x = todo_list.get(0);
    match x {
        Some(x) => println!("Found {}", x),
        None => println!("empty"),
    }
    println!("delete first idx - II");

    todo_list.delete_item(x.unwrap().id);
    todo_list.list_items();
}
