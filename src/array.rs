use crate::{Element, ELEMS_IN_PAGE};

#[derive(Debug, Clone)]
pub struct Array {
    elements: Vec<Element>,
}

impl Array {
    pub fn empty() -> Self {
        Self { elements: Vec::new() }
    }

    pub fn append(&mut self, element: Element) {
        self.elements.push(element);
    }

    pub fn find_good_local(&self, element: Element) -> bool {
        let mut i = 0;

        while i < self.elements.len() {
            if self.elements[i] == element {
                return true;
            }
            i += 1;
        }

        false
    }

    pub fn find_bad_local(&self, element: Element) -> bool {
        let jump_pages = 16;
        let jump_elements = jump_pages * ELEMS_IN_PAGE;

        let mut offset = 0;

        while offset < jump_elements {
            let mut jump_page = 0;
            let mut index = jump_page * jump_pages + offset;
            while index < self.elements.len() {
                if self.elements[index] == element {
                    return true;
                }
                jump_page += 1;
                index = jump_page * jump_pages + offset;
            }
            offset += 1;
        }

        false
    }

    pub fn find_worse_local(&self, element: Element) -> bool {
        let rounded = self.elements.len() + ELEMS_IN_PAGE - 1;
        let pages = rounded / ELEMS_IN_PAGE;
        let half_size = pages / 2 * ELEMS_IN_PAGE;

        let jump_pages = 16;
        let jump_elements = jump_pages * ELEMS_IN_PAGE;

        let mut offset = 0;

        while offset < jump_elements {
            let mut jump_page = 0;
            let mut in_bounds = true;
            let mut index = jump_page * jump_pages + offset;
            while in_bounds {
                in_bounds = false;
                let lower_index = index;
                if lower_index < half_size {
                    if self.elements[lower_index] == element {
                        return true;
                    }
                    in_bounds = true;
                }
                let upper_index = index + half_size;
                if upper_index < self.elements.len() {
                    if self.elements[upper_index] == element {
                        return true;
                    }
                    in_bounds = true;
                }
                jump_page += 1;
                index = jump_page * jump_pages + offset;
            }
            offset += 1;
        }

        false
    }

    pub fn inc_less_than_good_local(&mut self, element: Element) {
        let mut i = 0;

        while i < self.elements.len() {
            if self.elements[i] < element {
                self.elements[i] = self.elements[i].wrapping_add(1);
            }
            i += 1;
        }
    }

    pub fn inc_less_than_bad_local(&mut self, element: Element) {
        let jump_pages = 16;
        let jump_elements = jump_pages * ELEMS_IN_PAGE;

        let mut offset = 0;

        while offset < jump_elements {
            let mut jump_page = 0;
            let mut index = jump_page * jump_pages + offset;
            while index < self.elements.len() {
                if self.elements[index] < element {
                    self.elements[index] = self.elements[index].wrapping_add(1);
                }
                jump_page += 1;
                index = jump_page * jump_pages + offset;
            }
            offset += 1;
        }
    }

    pub fn inc_less_than_worse_local(&mut self, element: Element) {
        let rounded = self.elements.len() + ELEMS_IN_PAGE - 1;
        let pages = rounded / ELEMS_IN_PAGE;
        let half_size = pages / 2 * ELEMS_IN_PAGE;

        let jump_pages = 16;
        let jump_elements = jump_pages * ELEMS_IN_PAGE;

        let mut offset = 0;

        while offset < jump_elements {
            let mut jump_page = 0;
            let mut in_bounds = true;
            let mut index = jump_page * jump_pages + offset;
            while in_bounds {
                in_bounds = false;
                let lower_index = index;
                if lower_index < half_size {
                    if self.elements[lower_index] < element {
                        self.elements[index] =
                            self.elements[index].wrapping_add(1);
                    }
                    in_bounds = true;
                }
                let upper_index = index + half_size;
                if upper_index < self.elements.len() {
                    if self.elements[upper_index] < element {
                        self.elements[index] =
                            self.elements[index].wrapping_add(1);
                    }
                    in_bounds = true;
                }
                jump_page += 1;
                index = jump_page * jump_pages + offset;
            }
            offset += 1;
        }
    }
}
