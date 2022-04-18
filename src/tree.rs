use crate::Element;
use std::cmp;

#[derive(Debug)]
pub struct Tree {
    root: Option<Box<Node>>,
}

impl Default for Tree {
    fn default() -> Self {
        Self::empty()
    }
}

impl Tree {
    pub fn empty() -> Self {
        Self { root: None }
    }

    pub fn insert_with_order(&mut self, element: Element) -> bool {
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

    pub fn insert_without_order(&mut self, element: Element) {
        let mut this = self;
        let mut reverse = false;
        loop {
            match &mut this.root {
                Some(node) => {
                    this = match (element.cmp(&node.data), reverse) {
                        (cmp::Ordering::Equal, true)
                        | (cmp::Ordering::Less, false)
                        | (cmp::Ordering::Greater, true) => &mut node.left,
                        (cmp::Ordering::Equal, false)
                        | (cmp::Ordering::Less, true)
                        | (cmp::Ordering::Greater, false) => &mut node.right,
                    };
                    reverse = !reverse;
                },
                root @ None => {
                    *root = Some(Box::new(Node {
                        data: element,
                        left: Tree::empty(),
                        right: Tree::empty(),
                    }));
                    break;
                },
            }
        }
    }

    pub fn find_with_order(&self, element: Element) -> bool {
        let mut this = self;
        while let Some(node) = &this.root {
            match element.cmp(&node.data) {
                cmp::Ordering::Equal => return true,
                cmp::Ordering::Less => this = &node.left,
                cmp::Ordering::Greater => this = &node.right,
            }
        }
        false
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

    fn inc_all(&mut self) {
        let mut nodes = vec![&mut self.root];
        while let Some(maybe_node) = nodes.pop() {
            if let Some(node) = maybe_node.as_mut() {
                node.data = node.data.wrapping_add(1);
                nodes.push(&mut node.left.root);
                nodes.push(&mut node.right.root);
            }
        }
    }

    fn remove_duplicated_max(&mut self, parent: Element) {
        let mut this = self;
        loop {
            let (is_max, is_duplicated) = match this.root.as_ref() {
                Some(node) => (node.right.root.is_none(), node.data == parent),
                None => break,
            };
            if is_max {
                if is_duplicated {
                    let mut node = this.root.take().unwrap();
                    this.root = node.left.root.take();
                }
                break;
            }
            this = &mut this.root.as_mut().unwrap().right;
        }
    }

    pub fn inc_less_than_with_order(&mut self, element: Element) {
        if let Some(mut this_node) = self.root.as_mut() {
            if this_node.data < element {
                self.inc_all();
            } else {
                loop {
                    let found_branch = match this_node.left.root.as_ref() {
                        Some(node) => node.data < element,
                        None => break,
                    };
                    if found_branch {
                        this_node.left.inc_all();
                        this_node.left.remove_duplicated_max(this_node.data);
                        break;
                    }
                    this_node = this_node.left.root.as_mut().unwrap();
                }
            }
        }
    }

    pub fn inc_less_than_without_order(&mut self, element: Element) {
        let mut nodes: Vec<&mut Option<Box<Node>>> = vec![&mut self.root];
        while let Some(maybe_node) = nodes.pop() {
            if let Some(node) = maybe_node.as_mut() {
                if node.data < element {
                    node.data = node.data.wrapping_add(1);
                }
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
    fn iterate_with_order() {
        let mut tree = Tree::empty();
        tree.insert_with_order(10);
        tree.insert_with_order(3);
        tree.insert_with_order(5);
        tree.insert_with_order(9);
        tree.insert_with_order(7);
        tree.insert_with_order(8);
        tree.insert_with_order(6);
        let collected: Vec<_> = tree.into_iter().collect();
        assert_eq!(collected, &[3, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn find() {
        let cut_element = (ELEMS_IN_PAGE * 32 + ELEMS_IN_PAGE / 2) as Element;

        let mut tree_with_order = Tree::empty();
        let mut tree_without_order = Tree::empty();
        for i in (cut_element + 1 .. cut_element * 2).step_by(2) {
            tree_with_order.insert_with_order(i + 1);
            tree_with_order.insert_with_order(i);
            tree_without_order.insert_without_order(i + 1);
            tree_without_order.insert_without_order(i);
        }
        for i in (0 .. cut_element).step_by(2) {
            tree_with_order.insert_with_order(i + 1);
            tree_with_order.insert_with_order(i);
            tree_without_order.insert_without_order(i + 1);
            tree_without_order.insert_without_order(i);
        }

        assert!(!tree_with_order.find_with_order(cut_element));
        assert!(!tree_without_order.find_without_order(cut_element));

        tree_with_order.insert_with_order(cut_element);
        tree_without_order.insert_with_order(cut_element);

        assert!(tree_with_order.find_with_order(cut_element));
        assert!(tree_without_order.find_without_order(cut_element));
    }

    #[test]
    fn inc_less_than() {
        let cut_element = (ELEMS_IN_PAGE * 4 + ELEMS_IN_PAGE / 2) as Element;

        let mut tree_with_order = Tree::empty();
        let mut tree_without_order = Tree::empty();
        for i in (cut_element .. cut_element * 2).step_by(2) {
            tree_with_order.insert_with_order(i + 1);
            tree_with_order.insert_with_order(i);
            tree_without_order.insert_without_order(i + 1);
            tree_without_order.insert_without_order(i);
        }
        for i in (0 .. cut_element).step_by(2) {
            tree_with_order.insert_with_order(i + 1);
            tree_with_order.insert_with_order(i);
            tree_without_order.insert_without_order(i + 1);
            tree_without_order.insert_without_order(i);
        }

        tree_with_order.inc_less_than_with_order(cut_element);
        tree_without_order.inc_less_than_without_order(cut_element);

        let mut with_order_iter = tree_with_order.into_iter();

        for i in 1 .. cut_element * 2 {
            assert_eq!(with_order_iter.next(), Some(i));
        }

        assert_eq!(with_order_iter.next(), None);
    }
}
