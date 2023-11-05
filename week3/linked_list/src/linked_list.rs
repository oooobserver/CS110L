use std::fmt;
use std::option::Option;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl <T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {value, next}
    }
}

impl <T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {head: None, size: 0}
    }
    
    pub fn get_size(&self) -> usize {
        self.size
    }
    
    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }
    
    pub fn push_front(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        let node: Box<Node<T>> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
}


impl <T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

impl <T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}


impl <T: Clone> Clone for LinkedList<T>{
    fn clone(&self) -> Self {
        let mut new_list = LinkedList::new();
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut vec_node = Vec::new();
        loop{
            match current{
                Some(node) =>{
                    vec_node.push(node.value.clone());
                    current = &node.next;
                },
                None => break,
            }
        }
        for i in vec_node.iter().rev(){
            new_list.push_front(i.clone());
        }

        new_list
    }
}

impl <T: PartialOrd> PartialEq for LinkedList<T>{
    fn eq(&self, other: &Self) -> bool {
        if self.get_size() != other.get_size() {
            println!("{},{}",self.get_size(),other.get_size());
            return false;
        }
        let mut self_node = &self.head;
        let mut other_node = &other.head;

        loop{
            match self_node{
                Some(node) =>{
                   if let Some(onode) = other_node{
                        if onode.value != node.value{
                            return false;
                        }
                        self_node = &node.next;
                        other_node = &onode.next;
                   }  
                },
                None => break,
            }

        }
        true
    }
}


 