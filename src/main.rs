use ::datastructures::datastructures::double_linked_list::CopyableDoubleLinkedList;
type DoubleLinkedList<T> = CopyableDoubleLinkedList<T>;

mod datastructures;

fn main() {
    /*
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
    */
    let mut list: DoubleLinkedList<u32> = DoubleLinkedList::new();
    for i in 1..=6 {
        let number_elements: u32 = 4000000 * i;
        let now = std::time::Instant::now();
        // let mut list: Vec<usize> = Vec::with_capacity(number_elements);
        (0..number_elements).for_each(|i: u32| {list.add_tail(i)});
        println!("Elapsed time to insert {number_elements} into linked list: {:.5?}", now.elapsed());
        let mut vec: Vec<u32> = Vec::with_capacity(usize::try_from(number_elements).unwrap());
        let now = std::time::Instant::now();
        // let mut list: Vec<usize> = Vec::with_capacity(number_elements);
        (0..number_elements).for_each(|i| {vec.push(i)});
        println!("Elapsed time to insert {number_elements} into vector: {:.5?}", now.elapsed());
    }        
}