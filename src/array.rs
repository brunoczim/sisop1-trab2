use std::{cmp, slice};

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

    pub fn sort(&mut self) {
        self.elements.sort();
    }

    fn binary_search(&self, element: Element) -> Result<usize, usize> {
        let mut left = 0;
        let mut right = self.elements.len();

        while left < right {
            let mid = left + (right - left) / 2;
            match element.cmp(&self.elements[mid]) {
                cmp::Ordering::Less => right = mid,
                cmp::Ordering::Equal => return Ok(mid),
                cmp::Ordering::Greater => left = mid + 1,
            }
        }

        Err(left)
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

    pub fn find_sorted(&self, element: Element) -> bool {
        self.binary_search(element).is_ok()
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

    pub fn inc_less_than_sorted(&mut self, element: Element) {
        let mut index = match self.binary_search(element) {
            Ok(index) => index,
            Err(index) => index,
        };

        while index > 0 && self.elements[index] >= element {
            index -= 1;
        }

        for element in &mut self.elements[.. index] {
            *element = element.wrapping_add(1);
        }
    }
}

impl<'array> IntoIterator for &'array Array {
    type Item = Element;
    type IntoIter = Iter<'array>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { inner: self.elements.iter() }
    }
}

#[derive(Debug)]
pub struct Iter<'array> {
    inner: slice::Iter<'array, Element>,
}

impl<'array> Iterator for Iter<'array> {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().copied()
    }
}

#[cfg(test)]
mod test {
    use super::Array;
    use crate::{Element, ELEMS_IN_PAGE};

    #[test]
    fn iterate() {
        let mut array = Array::empty();
        array.append(10);
        array.append(3);
        array.append(5);
        array.append(9);
        let collected: Vec<_> = array.into_iter().collect();
        assert_eq!(collected, &[10, 3, 5, 9]);
    }

    #[test]
    fn find() {
        let mut array = Array::empty();
        for i in 0 .. ELEMS_IN_PAGE * 128 + ELEMS_IN_PAGE / 2 {
            array.append((i % 10) as Element);
        }

        let mut sorted_array = array.clone();
        sorted_array.sort();

        assert!(!array.find_good_local(11));
        assert!(!array.find_bad_local(11));
        assert!(!array.find_worse_local(11));
        assert!(!sorted_array.find_sorted(11));

        array.append(11);
        for i in 0 .. ELEMS_IN_PAGE * 128 + ELEMS_IN_PAGE / 2 {
            array.append((i % 10) as Element);
        }

        let mut sorted_array = array.clone();
        sorted_array.sort();

        assert!(array.find_good_local(11));
        assert!(array.find_bad_local(11));
        assert!(array.find_worse_local(11));
        assert!(sorted_array.find_sorted(11));
    }

    #[test]
    fn inc_less_than() {
        let cut_element = 5;

        let mut array = Array::empty();
        for i in 0 .. ELEMS_IN_PAGE * 257 + ELEMS_IN_PAGE / 2 {
            array.append((i % 10) as Element);
        }

        let mut good_array = array.clone();
        good_array.inc_less_than_good_local(cut_element);

        let mut bad_array = array.clone();
        bad_array.inc_less_than_good_local(cut_element);

        let mut worse_array = array.clone();
        worse_array.inc_less_than_good_local(cut_element);

        let mut sorted_array = array.clone();
        sorted_array.sort();
        sorted_array.inc_less_than_good_local(cut_element);

        let mut good_iter = good_array.into_iter();
        let mut bad_iter = bad_array.into_iter();
        let mut worse_iter = worse_array.into_iter();

        for i in 0 .. ELEMS_IN_PAGE * 257 + ELEMS_IN_PAGE / 2 {
            let mut expected = (i % 10) as Element;
            if expected < cut_element {
                expected += 1;
            }

            assert_eq!(good_iter.next(), Some(expected));
            assert_eq!(bad_iter.next(), Some(expected));
            assert_eq!(worse_iter.next(), Some(expected));
        }

        assert_eq!(good_iter.next(), None);
        assert_eq!(bad_iter.next(), None);
        assert_eq!(worse_iter.next(), None);

        assert!(sorted_array
            .into_iter()
            .try_fold(None, |maybe_prev, curr| match maybe_prev {
                Some(prev) if prev <= curr => Some(Some(curr)),
                Some(_) => None,
                None => Some(Some(curr)),
            })
            .is_some());
        assert!(sorted_array.into_iter().all(|element| element > 0));
    }
}
