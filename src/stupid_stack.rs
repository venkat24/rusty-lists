use std::mem;

pub struct StupidStack {
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

impl StupidStack {
    pub fn new() -> Self {
        // Construct an empty stack
        StupidStack { head: Link::Empty }
    }

    pub fn push(&mut self, value: i32) {
        // Create a new node
        // When we deal with the Link enum, which is the type of head, need to
        // use mem::replace to ensure that we're not copying head
        let new_node = Box::new(Node {
            elem: value,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // Again, match against mem::replace of the head so as to not copy
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for StupidStack {
    fn drop(&mut self) {
        // Get the current head link from the stack
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        // Aaaaah, pattern matcherrr
        while let Link::More(mut box_node) = cur_link {
            cur_link = mem::replace(&mut box_node.next, Link::Empty);

            // ...and we let box_node go out of scope here, we're moving it in by mut value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StupidStack;

    #[test]
    fn push_pop_test() {
        let mut stack = StupidStack::new();

        assert_eq!(stack.pop(), None);

        stack.push(6);
        stack.push(5);
        stack.push(4);

        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(5));

        stack.push(8);
        stack.push(7);

        assert_eq!(stack.pop(), Some(7));
        assert_eq!(stack.pop(), Some(8));
        assert_eq!(stack.pop(), Some(6));
        assert_eq!(stack.pop(), None);
    }
}
