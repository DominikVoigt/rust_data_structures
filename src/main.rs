use datastructures::double_linked_list::CopyableDoubleLinkedList;
type DoubleLinkedList<T> = CopyableDoubleLinkedList<T>;

mod datastructures;

fn main() {
    let mut list: DoubleLinkedList<i32> = DoubleLinkedList::new();
    list.add_tail(0);
    list.add_tail(1);
    list.add_tail(2);
    println!("Length of list after inserts: {}", list.get_length());
    println!("Value at index 1: {}", list.get_value_at(1).unwrap());
    println!("Value at index 2: {}", list.get_value_at(2).unwrap());
    println!("Value of removed node: {}", list.remove_node_at_index(1).unwrap());
    println!("Length after remove: {}", list.get_length());
    println!("value of head before removing head: {}", list.get_value_at(0).unwrap());
    println!("Value of removed head: {}", list.remove_node_at_index(0).unwrap());
    println!("Length after remove: {}", list.get_length());
    println!("value of head after removing old head: {}", list.get_value_at(0).unwrap());
}