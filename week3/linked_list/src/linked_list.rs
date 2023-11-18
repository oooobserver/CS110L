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



// implement for take ownership
impl <T> Iterator for LinkedList<T>{
    type Item = T;
    fn next(& mut self) -> Option<T>{
        self.pop_front()
    }
}

// implement for the borrow
pub struct LinkedListIter<'a, T> {
    current: &'a Option<Box<Node<T>>>,
}




impl <T: Clone> Iterator for LinkedListIter<'_, T>{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        match self.current {
            Some(node) => {
                let value = node.value.clone();
                self.current = &node.next;
                Some(value)
            },
            None => None,
        }
    }
}

impl<'a,T: Clone> IntoIterator for &'a LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIter<'a,T>;
    fn into_iter(self) -> LinkedListIter<'a,T> {
        LinkedListIter {current: &self.head}
    }
}

pub trait ComputeNorm {
    fn compute_norm(&self) -> f64 {
        0.0
    }
}


impl ComputeNorm for LinkedList<f64> {
    fn compute_norm(&self) -> f64 {
        let mut sq_sum = 0.0;
        for x in self {
            sq_sum += x * x;
        }
        sq_sum.sqrt()
    }
}







#[cfg(test)]
mod test
{
    use super::*;
    #[test]
    fn test_copy() {
        let mut list1: LinkedList<u32> = LinkedList::new();
        for i in 1..12 {
            list1.push_front(i);
        }
        let list2 = list1.clone();

        let equal = list1 == list2;
        assert!(equal);
    }

    #[test]

    fn test_compare_different_size(){
        let mut list1: LinkedList<u32> = LinkedList::new();
        for i in 1..12 {
            list1.push_front(i);
        }

        let mut list2: LinkedList<u32> = LinkedList::new();
        for i in 1..13 {
            list2.push_front(i);
        }

        let equal = list1 == list2;
        assert!(!equal);
    }

    #[test]

    fn test_compare_same_size(){
        let mut list1: LinkedList<u32> = LinkedList::new();
        for i in 1..12 {
            list1.push_front(i);
        }

        let mut list2: LinkedList<u32> = LinkedList::new();
        for i in 2..13 {
            list2.push_front(i);
        }

        let equal = list1 == list2;
        assert!(!equal);
    }

    #[test]
    fn test_iter_take(){
        let mut list1: LinkedList<u32> = LinkedList::new();
        for i in 1..12 {
            list1.push_front(i);
        }
        for i in list1{
            println!("{}",i);
        }
    }

    #[test]
    fn test_iter_borrow(){
        let mut list1: LinkedList<u32> = LinkedList::new();
        for i in 1..12 {
            list1.push_front(i);
        }
        for i in &list1{
            println!("{}",i);
        }
    }

    

}