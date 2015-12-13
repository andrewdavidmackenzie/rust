use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            // Replaces the value at a mutable location with a new one, returning the old value,
            // without deinitializing or copying either one.
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // Replace head with empty and get the previous head value for the match
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(boxed_node) => {
                // DeRef move out of Boxed value
                let node = *boxed_node;
                // now make head point to the previously next element in the list
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

/*
    A non-tail recursive Drop deallocator for List, as it has a Boxed Node

    This would be the basic code:

    impl Drop for Box<Node> {
    fn drop(&mut self) {
        self.ptr.drop(); // uh oh, not tail recursive!
        deallocate(self.ptr);
    }
}
*/
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn pops_none_when_new() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn returns_last_pushed_value() {
        let mut list = List::new();
        list.push(1);
        assert_eq!(list.pop(), Some(1));
    }

    #[test]
    fn pops_none_when_emptied() {
        let mut list = List::new();
        list.push(1);
        list.pop();
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn pushes_pops_pushes_pops() {
        let mut list = List::new();

        // Populate list
        list.push(1);
        list.push(2);

        assert_eq!(list.pop(), Some(2));

        list.push(3);
        assert_eq!(list.pop(), Some(3));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
    }
}