pub struct OkayStack<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
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
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
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

/// Iterators

// IntoIter (Value)

// This is a Tuple-Struct. It's like a struct with anonymous members that you can
// access like a tuple. Here you can do into_iter.0 to get the OkayStack obj
pub struct IntoIter<T>(OkayStack<T>);

impl<T> OkayStack<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// Iter (Reference)

// Still not sure if I understand Lifetimes properly but I have it down for this
// use case here. Also worth reading well why they need to be specified explicitly
// here and cannot be inferred

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> OkayStack<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

// IterMut (mutable reference)

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>, 
}

impl<T> OkayStack<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

/// Tests

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

        stack.peek_mut().map(|value| *value = 7);
        assert_eq!(stack.peek(), Some(&7));

        assert_eq!(stack.pop(), Some(7));
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut stack = OkayStack::new();
        stack.push(4);
        stack.push(5);
        stack.push(6);

        // The IntoIter is just a move iterator that's going to give us each value
        // It's the same as popping out the elements one at a time
        let mut iter = stack.into_iter();
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut stack = OkayStack::new();
        stack.push(4);
        stack.push(5);
        stack.push(6);

        let mut iter = stack.iter();
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&4));
    }

    #[test]
    fn iter_mut() {
        let mut stack = OkayStack::new();
        stack.push(4);
        stack.push(5);
        stack.push(6);

        let mut iter = stack.iter_mut();
        assert_eq!(iter.next(), Some(&mut 6));
        assert_eq!(iter.next(), Some(&mut 5));
        assert_eq!(iter.next(), Some(&mut 4));
    }
}
