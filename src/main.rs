mod array;
mod linked_list;
mod tree;

use rand::{rngs::StdRng, Rng, SeedableRng};
use std::{collections::HashSet, mem};

use array::Array;
use linked_list::LinkedList;
use tree::Tree;

type Element = u64;

const ELEMS_IN_PAGE: usize = 0x1000 / mem::size_of::<Element>();
const TOTAL_ELEMS: usize = ELEMS_IN_PAGE * 0x1000;

fn main() {
    let mut array = Array::empty();
    let mut linked_list = LinkedList::empty();
    let mut tree = Tree::empty();
    let mut rng = StdRng::from_seed([0; 32]);
    let mut element_set = HashSet::new();

    while element_set.len() < TOTAL_ELEMS {
        let element: Element = rng.gen();
        if !element_set.contains(&element) {
            element_set.insert(element);
            array.append(element);
            linked_list.prepend(element);
            tree.insert(element);
        }
    }
}
