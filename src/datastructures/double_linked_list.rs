
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

/*
impl <T: Clone + PartialEq + Display> Drop for Node<T> {
    fn drop(&mut self) {
        println!("Dropping Node with value {}", self.data.clone());
    }
}
 */

impl <T: Clone + PartialEq + Display> Drop for CopyableDoubleLinkedList<T> {
    /*
     * Since the drop would happen recursively for our list (as drop is called on the smart pointers)   
     */
    fn drop(&mut self) {
        if self.tail.is_none() || self.head.is_none() {
            println!("List is empty, nothing to drop");
            return
        }
        // Take the tail node from the list
        let head = std::mem::replace(&mut self.head, Option::None);
        let head = head.unwrap();
        // Take away next pointer from head, now head will be dropped, but no recursion occurs.
        let mut next = std::mem::replace(&mut RefCell::borrow_mut(&head).child_node, None);
        // println!("after dropping head");
        while next.is_some() {
            let next_rc = next.unwrap();
            next = std::mem::replace(&mut RefCell::borrow_mut(&next_rc).child_node, None);
        }
        // tail will be dropped automatically
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

    pub fn remove_node_at(&mut self, index: u32) -> Option<T> {
        if index >= self.length  {
            return None;
        }
        return self.remove_node_from_start_at(index);
    }

    fn remove_node_from_start_at(&mut self, index: u32) -> Option<T> {
        if index == 0 {
            return self.remove_head();
        } else if index == self.length - 1 {
            return self.remove_tail();
        }
        self.length = self.length - 1;
        if let Some(head) = &self.head {
            let mut node = Rc::clone(head);
            // iterate to node
            for _i in 1..=index {
                // 
                let next_node = Rc::clone(RefCell::borrow(&node).child_node.as_ref().unwrap());
                node = next_node;
            }
            // node RC now points towards the correct node with the refcell, since it is neither the head or tail, it has to have a parent node and child node
            let node_ref = RefCell::borrow(&node);
            let child_ref = node_ref.child_node.as_ref().unwrap();
            let parent_node = node_ref.parent_node.as_ref().unwrap().upgrade().unwrap();
            parent_node.borrow_mut().child_node = Some(Rc::clone(child_ref));
            child_ref.borrow_mut().parent_node = Some(Rc::downgrade(&parent_node));
            Some(node_ref.data.clone())
            // TODO: Implement deletion of the node, be aware: we have to handle the case where we delete the first and/or last node
        } else {
            None
        }
    }

    fn remove_head(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        self.length = self.length - 1;
        let current_head = self.head.as_ref().unwrap();
        let data = RefCell::borrow(current_head).data.clone();
        let second_node = RefCell::borrow(current_head).child_node.as_ref().map(Rc::clone);
        match second_node {
            Some(second_node) => {
                self.head = Some(second_node);
            },
            None => {
                self.head = None;
            }
        }
        return Some(data);
    }


    fn remove_tail(&mut self) -> Option<T> {
        self.length = self.length - 1;
        todo!()
    }
    
    fn remove_node_from_end_at(&mut self, index: u32) -> Option<T> {
        todo!()
    }
}