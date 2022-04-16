use crate::Element;

#[derive(Debug)]
pub struct LinkedList {
    top: Option<Box<Node>>,
}

impl Default for LinkedList {
    fn default() -> Self {
        Self::empty()
    }
}

impl LinkedList {
    pub fn empty() -> Self {
        LinkedList { top: None }
    }

    pub fn prepend(&mut self, element: Element) {
        let next = LinkedList { top: self.top.take() };
        self.top = Some(Box::new(Node { data: element, next }));
    }

    pub fn find(&self, element: Element) -> bool {
        let mut this = self;
        loop {
            match &this.top {
                Some(top) if top.data == element => {
                    return true;
                },
                Some(top) => this = &top.next,
                None => return false,
            }
        }
    }

    pub fn inc_less_than(&mut self, element: Element) {
        let mut this = self;
        loop {
            match &mut this.top {
                Some(top) => {
                    if top.data < element {
                        top.data = top.data.wrapping_add(1);
                    }
                    this = &mut top.next;
                },
                None => break,
            }
        }
    }
}

impl Clone for LinkedList {
    fn clone(&self) -> Self {
        let mut source_list = self;
        let mut new_list = LinkedList::empty();
        let mut new_list_end = &mut new_list.top;

        while let Some(node) = &source_list.top {
            *new_list_end = Some(Box::new(Node {
                data: node.data,
                next: LinkedList::empty(),
            }));
            new_list_end = &mut new_list_end.as_mut().unwrap().next.top;
            source_list = &node.next;
        }

        new_list
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        while let Some(top) = self.top.take() {
            *self = top.next;
        }
    }
}

impl<'list> IntoIterator for &'list LinkedList {
    type Item = Element;
    type IntoIter = Iter<'list>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { list: self }
    }
}

pub struct Iter<'list> {
    list: &'list LinkedList,
}

impl<'list> Iterator for Iter<'list> {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.list.top {
            Some(node) => {
                let element = node.data;
                self.list = &node.next;
                Some(element)
            },
            None => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    data: Element,
    next: LinkedList,
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn iterate() {
        let mut list = LinkedList::empty();
        list.prepend(10);
        list.prepend(3);
        list.prepend(5);
        list.prepend(9);

        let collected: Vec<_> = list.into_iter().collect();
        assert_eq!(collected, &[9, 5, 3, 10]);

        let cloned = list.clone();

        let collected: Vec<_> = cloned.into_iter().collect();
        assert_eq!(collected, &[9, 5, 3, 10]);
    }

    #[test]
    fn find() {
        let mut list = LinkedList::empty();
        list.prepend(10);
        list.prepend(3);
        list.prepend(5);
        list.prepend(9);

        assert!(list.find(10));
        assert!(list.find(3));
        assert!(list.find(5));
        assert!(list.find(9));
        assert!(!list.find(11));
    }

    #[test]
    fn inc_less_than() {
        let mut list = LinkedList::empty();
        list.prepend(10);
        list.prepend(3);
        list.prepend(5);
        list.prepend(9);

        let cut_element = 7;
        list.inc_less_than(cut_element);

        let collected: Vec<_> = list.into_iter().collect();
        assert_eq!(collected, &[9, 6, 4, 10]);
    }
}
