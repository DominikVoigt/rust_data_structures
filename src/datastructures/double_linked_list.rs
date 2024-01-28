use std::borrow::Borrow;
use std::cell::Ref;
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use std::rc::Weak;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;
pub struct CopyableDoubleLinkedList<T: Clone + PartialEq  + Display> {
    head: Link<T>,
    tail: Link<T>,
    length: u32
}


pub struct Node<T: Clone + PartialEq  + Display> {
    data: T,
    child_node: Link<T>,
    parent_node: WeakLink<T>
}

impl <T: Clone + PartialEq  + Display> Node<T> {
    
    fn new(data: T) -> Node<T> {
        Node { data: data, child_node: None, parent_node: None }
    }
}

impl <T: Clone + PartialEq + Display> Drop for Node<T> {
    fn drop(&mut self) {
        println!("Dropping Node with value {}", self.data.clone());
    }
}

impl <T: Clone + PartialEq + Display> CopyableDoubleLinkedList<T> {
    
    pub fn new() -> CopyableDoubleLinkedList<T>{
        CopyableDoubleLinkedList {
            head: None,
            tail: None,
            length: 0
        }
    }

    pub fn add_tail(&mut self, value: T) {
        let node = Rc::new(RefCell::new(Node::new(value)));
        if self.length == 0 {
            self.tail = Some(Rc::clone(&node));
            self.head = Some(node);
            self.length = 1;
            return;
        }
        // Here we know that the list has a tail node and thus the option will not be none
        let current_tail = std::mem::replace(&mut self.tail, Some(node));
        if let Some(current_tail) = current_tail {
            if let Some(new_tail) = &self.tail {
                // Get a second strong pointer on the tail element from its parent
                current_tail.borrow_mut().child_node = Some(Rc::clone(new_tail));
                // Get a weak pointer from the new tail element to the old one (to its parent) -> Weak to allows cleanup (no cyclic relationships)
                new_tail.borrow_mut().parent_node = Some(Rc::downgrade(&current_tail));
            }
        }
        self.length += 1;
    }

    pub fn get_length(&self) -> u32 {
        self.length
    }

    pub fn get_value_at(&mut self, index: u32) -> Option<T> {
        if self.length < index + 1 {
            return None;
        }
        if let Some(head) = &self.head {
            let mut node = Rc::clone(head);
            for _ in 1..=index {
                let next_node = Rc::clone(RefCell::borrow(node.as_ref()).child_node.as_ref().unwrap());
                node = next_node;
            }
            return Some(RefCell::borrow(node.as_ref()).data.clone());
        } else {
            return None
        }
    }

    pub fn remove_node_at_index(&mut self, index: u32) -> Option<T> {
        if index >= self.length  {
            return None;
        }
        if index < self.length / 2 {
            self.remove_node_at_index_from_start(index)
        } else {
            self.remove_node_at_index_from_end(index)
        }
    }

    fn remove_node_at_index_from_start(&mut self, index: u32) -> Option<T> {
        if let Some(head) = &self.head {
            let mut node = Rc::clone(head);
            // iterate to node
            for i in 1..=index {
                let next_node = Rc::clone(RefCell::borrow(&node).child_node.as_ref().unwrap());
                node = next_node;
            }
            None
            // TODO: Implement deletion of the node, be aware: we have to handle the case where we delete the first and/or last node
        } else {
            None
        }
    }

    fn remove_node_at_index_from_end(&mut self, index: u32) -> Option<T> {
        None
    }
}