use linked_list::LinkedList;
pub mod linked_list;

fn main() {
    // let mut list: LinkedList<u32> = LinkedList::new();
    // for i in 1..12 {
    //     list.push_front(i);
    // }
    // println!("{}", list);
    // println!("list size: {}", list.get_size());
    // println!("top element: {}", list.pop_front().unwrap());
    // println!("{}", list);
    // println!("size: {}", list.get_size());
    // println!("{}", list.to_string()); // ToString impl for anything impl Display

    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}

    let mut list1: LinkedList<u32> = LinkedList::new();
    for i in 1..12 {
        list1.push_front(i);
    }

    let list2 = list1.clone();

    println!("{}", list1);
    println!("{}", list2);

    let equal = list1 == list2;
    println!("{}",equal);


#[cfg(test)]
mod test{
    use super::*;
    #[test]

    fn test_copy_compare() {
        let mut list1: LinkedList<u32> = LinkedList::new();
        for i in 1..12 {
            list1.push_front(i);
        }
    
        let mut list2: LinkedList<u32> = LinkedList::new();
        for i in 2..13 {
            list2.push_front(i);
        }

        println!("{}", list1);
        println!("{}", list2);

        let equal = list1 == list2;
        assert!(!equal);
    }

}
    
}
