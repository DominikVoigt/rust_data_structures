use std::cell::RefCell;
use std::rc::Rc;

type Link<'a, T> = Option<Rc<RefCell<Node<'a, T>>>>;
pub struct LinkedList<'a, T: Clone + PartialEq> {
    head: Link<'a, T>,
    tail: Link<'a, T>
}


#[derive(Debug)]
pub struct Node<'a, T: Clone + PartialEq> {
    data: T,
    child: Link<'a, T>,
    parent: Link<'a, T>
}

impl <'a, T: Clone + PartialEq> LinkedList<'a, T> {
    pub fn get_head_value(&'a self) -> Option<T> {
        match &self.head {
            Some(node) => {
                Some(node.borrow().data.clone())
            }
            None => None,
        }
    }

    pub fn add_value_front(&'a mut self, value: T) {
        match &mut self.head {
            Some(head) => {
                let current_head = std::mem::replace(head, Rc::new(RefCell::new(Node::new(value))));
                if let Some(new_head) = &self.head {
                    new_head.borrow_mut().child = Some(Rc::clone(&current_head));
                    current_head.borrow_mut().parent = Some(Rc::clone(&current_head))
                }
            }
            None => {
                let new_node = Rc::new(RefCell::new(Node::new(value)))
                self.tail = Some(Rc::clone(&new_node));
                self.head = Some(new_node);
            }
        }
    }

    pub fn remove_value(&'a mut self, value: T) -> bool {
        let node = self.head;
        while let Some(unpacked_node) = node {
            
        }
    }
}

impl <'a, T: Clone + PartialEq> Node<'a, T> {
    
    fn new(value: T) -> Node<'a, T> {
        Node { data: value, child: None, parent: None }
    }
}
