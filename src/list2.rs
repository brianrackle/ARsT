type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_head = Box::new(Node::<T> {
            elem: value,
            next: self.head.take(),//std::mem::replace(&mut self.head, None) //Sets new_head.next to current List.head, and sets List.head to None
        });
        self.head = Some(new_head); //sets List head to new_head (from None)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| { &node.elem })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| { &mut node.elem })
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

    fn init_some() -> List<u8> {
        let mut list = List::<u8>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list
    }

    #[test]
    fn pop_empty() {
        let mut list = List::<u8>::new();
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn push_and_pop() {
        let mut list = init_some();
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let list = init_some();
        assert_eq!(list.peek(), Some(&3));
    }

    #[test]
    fn peek_mut() {
        let mut list = init_some();

        list.peek_mut().map(|value| { *value = 4; });
        assert_eq!(list.peek(), Some(&4));
    }
}