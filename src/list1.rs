
type Link<T> = Option<Box<Node<T>>>;

pub struct List <T> {
    head: Link<T>
}

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value : T) {
        let new_head = Box::new(Node::<T> {
            elem: value,
            next: std::mem::replace(&mut self.head, None) //Sets new_head.next to current List.head, and sets List.head to None
        });
        self.head = Some(new_head); //sets List head to new_head (from None)
    }

    pub fn pop(&mut self) -> Option<T> {
        match std::mem::replace(&mut self.head, None) { //match against head, and set head to none
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur = std::mem::replace(&mut self.head, None);

        while let Some(mut boxed_node) = cur {
            cur = std::mem::replace(&mut boxed_node.next, None);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

#[cfg(test)] //module should only be compiled for testing
mod test {
    use super::List;

    #[test]
    fn pop_empty() {
        let mut list = List::<u8>::new();
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn push_and_pop() {
        let mut list = List::<u8>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}