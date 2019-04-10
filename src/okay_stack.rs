pub struct OkayStack<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> OkayStack<T> {
    pub fn new() -> Self {
        // Construct an empty stack
        OkayStack { head: None }
    }

    pub fn push(&mut self, value: T) {
        // Create a new node
        // Ok so turns out the whole mem::replace was total bullshit cause Option
        // gives us a .take() which just does the same thing - returns the value
        // of the object that was inside and replaces it with a None
        let new_node = Box::new(Node {
            elem: value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Drop for OkayStack<T> {
    fn drop(&mut self) {
        // Get the current head link from the stack
        let mut cur_link = self.head.take();

        // Aaaaah, pattern matcherrr
        while let Some(mut box_node) = cur_link {
            cur_link = box_node.next.take();

            // ...and we let box_node go out of scope here, we're moving it in by mut value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OkayStack;

    #[test]
    fn push_pop_test() {
        let mut stack = OkayStack::new();

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

    #[test]
    fn peek_test() {
        let mut stack = OkayStack::new();

        stack.push(4);
        stack.push(5);
        stack.push(6);

        assert_eq!(stack.peek(), Some(&6));

        stack.peek_mut().map(|value| {
            *value = 7
        });
        assert_eq!(stack.peek(), Some(&7));

        assert_eq!(stack.pop(), Some(7));
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), None);
    }
}
