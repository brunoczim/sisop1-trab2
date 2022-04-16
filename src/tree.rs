use crate::Element;
use std::cmp;

#[derive(Debug)]
pub struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    pub fn empty() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, element: Element) -> bool {
        let mut this = self;
        loop {
            match &mut this.root {
                Some(node) => match element.cmp(&node.data) {
                    cmp::Ordering::Equal => return false,
                    cmp::Ordering::Less => this = &mut node.left,
                    cmp::Ordering::Greater => this = &mut node.right,
                },
                root @ None => {
                    *root = Some(Box::new(Node {
                        data: element,
                        left: Tree::empty(),
                        right: Tree::empty(),
                    }));
                    return true;
                },
            }
        }
    }

    pub fn find_with_order(&self, element: Element) -> bool {
        let mut this = self;
        loop {
            match &this.root {
                Some(node) => match element.cmp(&node.data) {
                    cmp::Ordering::Equal => return true,
                    cmp::Ordering::Less => this = &node.left,
                    cmp::Ordering::Greater => this = &node.right,
                },
                None => return false,
            }
        }
    }

    pub fn find_without_order(&self, element: Element) -> bool {
        let mut nodes: Vec<&Option<Box<Node>>> = vec![&self.root];
        while let Some(maybe_node) = nodes.pop() {
            if let Some(node) = maybe_node {
                if node.data == element {
                    return true;
                }
                nodes.push(&node.left.root);
                nodes.push(&node.right.root);
            }
        }
        false
    }

    pub fn inc_less_than_with_order(&mut self, element: Element) {
        let mut this = &mut *self;
        loop {
            let mut found_branch = false;
            if let Some(node) = &mut this.root {
                if node.data < element {
                    found_branch = true;
                }
            }
            if found_branch {
                let mut nodes = vec![this.root.take()];
                while let Some(maybe_node) = nodes.pop() {
                    if let Some(mut node) = maybe_node {
                        self.insert(node.data.wrapping_add(1));
                        nodes.push(node.left.root.take());
                        nodes.push(node.right.root.take());
                    }
                }
                break;
            }
            match &mut this.root {
                Some(node) => {
                    this = &mut node.left;
                },
                None => break,
            }
        }
    }

    pub fn inc_less_than_without_order(&mut self, element: Element) {
        let mut nodes: Vec<&mut Option<Box<Node>>> = vec![&mut self.root];
        while let Some(mut maybe_node) = nodes.pop() {
            let mut found_branch = false;
            if let Some(node) = &mut maybe_node {
                if node.data < element {
                    found_branch = true;
                }
            }

            if found_branch {
                let mut nodes = vec![maybe_node.take()];
                while let Some(maybe_node) = nodes.pop() {
                    if let Some(mut node) = maybe_node {
                        self.insert(node.data.wrapping_add(1));
                        nodes.push(node.left.root.take());
                        nodes.push(node.right.root.take());
                    }
                }
                break;
            }
            if let Some(node) = maybe_node {
                nodes.push(&mut node.left.root);
                nodes.push(&mut node.right.root);
            }
        }
    }
}

impl Clone for Tree {
    fn clone(&self) -> Self {
        let mut new_tree = Tree::empty();
        let mut nodes = vec![(self, &mut new_tree.root)];

        while let Some((maybe_old_node, new_node)) = nodes.pop() {
            if let Some(old_node) = &maybe_old_node.root {
                *new_node = Some(Box::new(Node {
                    data: old_node.data,
                    left: Tree::empty(),
                    right: Tree::empty(),
                }));
                let new_node_unwrapped = new_node.as_mut().unwrap();
                nodes.push((&old_node.left, &mut new_node_unwrapped.left.root));
                nodes.push((
                    &old_node.right,
                    &mut new_node_unwrapped.right.root,
                ));
            }
        }

        new_tree
    }
}

impl Drop for Tree {
    fn drop(&mut self) {
        let mut nodes: Vec<Option<Box<Node>>> = vec![self.root.take()];

        while let Some(maybe_node) = nodes.pop() {
            if let Some(mut node) = maybe_node {
                nodes.push(node.left.root.take());
                nodes.push(node.right.root.take());
            }
        }
    }
}

impl<'tree> IntoIterator for &'tree Tree {
    type Item = Element;
    type IntoIter = Iter<'tree>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            entries: vec![IterEntry {
                left_processed: false,
                node: &self.root,
            }],
        }
    }
}

#[derive(Debug)]
struct IterEntry<'tree> {
    left_processed: bool,
    node: &'tree Option<Box<Node>>,
}

#[derive(Debug)]
pub struct Iter<'tree> {
    entries: Vec<IterEntry<'tree>>,
}

impl<'tree> Iterator for Iter<'tree> {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut entry = self.entries.pop()?;
            if let Some(node) = entry.node.as_ref() {
                match &node.left.root {
                    Some(_) if !entry.left_processed => {
                        entry.left_processed = true;
                        self.entries.push(entry);
                        self.entries.push(IterEntry {
                            left_processed: false,
                            node: &node.left.root,
                        });
                    },
                    _ => {
                        self.entries.push(IterEntry {
                            left_processed: false,
                            node: &node.right.root,
                        });
                        return Some(node.data);
                    },
                }
            }
        }
    }
}

#[derive(Debug)]
struct Node {
    data: Element,
    left: Tree,
    right: Tree,
}

#[cfg(test)]
mod test {
    use super::Tree;
    use crate::{Element, ELEMS_IN_PAGE};

    #[test]
    fn iterate() {
        let mut tree = Tree::empty();
        tree.insert(10);
        tree.insert(3);
        tree.insert(5);
        tree.insert(9);
        tree.insert(7);
        tree.insert(8);
        tree.insert(6);
        let collected: Vec<_> = tree.into_iter().collect();
        assert_eq!(collected, &[3, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn find() {
        let cut_element = (ELEMS_IN_PAGE * 32 + ELEMS_IN_PAGE / 2) as Element;

        let mut tree = Tree::empty();
        for i in (cut_element + 1 .. cut_element * 2).step_by(2) {
            tree.insert(i + 1);
            tree.insert(i);
        }
        for i in (0 .. cut_element).step_by(2) {
            tree.insert(i + 1);
            tree.insert(i);
        }

        assert!(!tree.find_with_order(cut_element));
        assert!(!tree.find_without_order(cut_element));

        tree.insert(cut_element);

        assert!(tree.find_with_order(cut_element));
        assert!(tree.find_without_order(cut_element));
    }

    #[test]
    fn inc_less_than() {
        let cut_element = (ELEMS_IN_PAGE * 32 + ELEMS_IN_PAGE / 2) as Element;

        let mut tree = Tree::empty();
        for i in (cut_element .. cut_element * 2).step_by(2) {
            tree.insert(i + 1);
            tree.insert(i);
        }
        for i in (0 .. cut_element).step_by(2) {
            tree.insert(i + 1);
            tree.insert(i);
        }

        let mut with_order_tree = tree.clone();
        with_order_tree.inc_less_than_with_order(cut_element);

        let mut without_order_tree = tree.clone();
        without_order_tree.inc_less_than_without_order(cut_element);

        let mut with_order_iter = with_order_tree.into_iter();
        let mut without_order_iter = without_order_tree.into_iter();

        for i in 1 .. cut_element * 2 {
            assert_eq!(with_order_iter.next(), Some(i));
            assert_eq!(without_order_iter.next(), Some(i));
        }

        assert_eq!(with_order_iter.next(), None);
        assert_eq!(without_order_iter.next(), None);
    }
}
