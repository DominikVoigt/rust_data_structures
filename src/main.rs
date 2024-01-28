use std::{cell::RefCell, rc::Rc};
use datastructures::double_linked_list::DoubleLinkedList;

mod datastructures;

fn main() {
    let mut list: DoubleLinkedList<i32> = DoubleLinkedList::new();
    list.add_tail(0);
    list.add_tail(1);
    list.add_tail(2);
    println!("{}", list.get_length());
    println!("{}", list.get_value_at(1).unwrap());
}