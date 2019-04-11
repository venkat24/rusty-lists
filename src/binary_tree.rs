use std::cmp::Ordering;
use std::fmt::Display;

pub struct BinaryTree<T> {
    root: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Ord + Display> Node<T> {
    pub fn new(value: T) -> Self {
        Node {elem: value, left: None, right: None}
    }

    pub fn insert(&mut self, value: T) {
        match self.elem.cmp(&value) {
            Ordering::Greater => {
                match &mut self.right {
                    None => self.right = Some(Box::new(Node::new(value))),
                    Some(node) => (*node).insert(value),
                }
            }
            Ordering::Less | Ordering::Equal => {
                match &mut self.left {
                    None => self.left = Some(Box::new(Node::new(value))),
                    Some(node) => (*node).insert(value),
                }
            }
        }
    }

    pub fn preorder_display(&self) {
        println!("{}", self.elem);

        self.left.as_ref().map(|node| node.preorder_display());
        self.right.as_ref().map(|node| node.preorder_display());
    }

    pub fn postorder_display(&self) {
        self.left.as_ref().map(|node| node.postorder_display());
        self.right.as_ref().map(|node| node.postorder_display());

        println!("{}", self.elem);
    }

    pub fn inorder_display(&self) {
        self.left.as_ref().map(|node| node.inorder_display());
        println!("{}", self.elem);
        self.right.as_ref().map(|node| node.inorder_display());
    }
}

impl<T: Ord + Display>  BinaryTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, elem: T) {
        match &mut self.root {
            None => {
                let new_node = Box::new(Node {
                    elem: elem,
                    left: None,
                    right: None
                });
                self.root = Some(new_node);
            }
            Some(node) => {
                node.insert(elem);
            }
        }
    }

    pub fn preorder_display(&self) {
        println!("Pre-order Traversal :");
        self.root.as_ref().map(|node| node.preorder_display());
    }

    pub fn postorder_display(&self) {
        println!("Post-order Traversal :");
        self.root.as_ref().map(|node| node.postorder_display());
    }

    pub fn inorder_display(&self) {
        println!("In-order Traversal :");
        self.root.as_ref().map(|node| node.inorder_display());
    }
}

#[cfg(test)]
mod tests {
    use super::BinaryTree;

    #[test]
    fn tree_test() {
        let mut tree = BinaryTree::new();

        tree.insert(3);
        tree.insert(2);
        tree.insert(6);
        tree.insert(4);
        tree.insert(1);
        tree.insert(7);
        tree.insert(8);
        tree.insert(5);

        tree.preorder_display();
        tree.inorder_display();
        tree.postorder_display();
    }
}
