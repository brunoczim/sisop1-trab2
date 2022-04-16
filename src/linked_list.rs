use crate::Element;

#[derive(Debug)]
pub struct LinkedList {
    top: Option<Box<Node>>,
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
        let mut new_list = LinkedList::empty();
        let mut new_list_end = &mut new_list.top;

        while let Some(node) = &self.top {
            *new_list_end = Some(Box::new(Node {
                data: node.data,
                next: LinkedList::empty(),
            }));
            new_list_end = &mut new_list_end.as_mut().unwrap().next.top;
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

#[derive(Debug)]
struct Node {
    data: Element,
    next: LinkedList,
}
