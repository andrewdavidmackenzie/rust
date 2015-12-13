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