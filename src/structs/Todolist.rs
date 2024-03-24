use crate::structs::Todoitem as Todoitem;

// mod Todoitem;
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct TodoList<'a> {
    items: Vec<Todoitem::Todoitem<'a>>
}