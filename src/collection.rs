use crate::{
    array::Array as ArrayImpl,
    linked_list::LinkedList as LinkedListImpl,
    tree::Tree as TreeImpl,
    Element,
};
use std::{io, time::Instant};

#[derive(Debug, Clone, Copy, serde::Serialize)]
struct RecordRow<'mode, 'oper> {
    mode: &'mode str,
    size: usize,
    operation: &'oper str,
    collection: &'static str,
    nanoseconds: u128,
}

pub trait Collection: Sized {
    const NAME: &'static str;

    fn create(elements: &[Element]) -> Self;

    fn find(&self, element: Element) -> bool;

    fn inc_less_than(&mut self, element: Element);

    fn record_create<W>(
        elements: &[Element],
        mode_name: &str,
        oper_name: &str,
        csv_writer: &mut csv::Writer<W>,
    ) -> io::Result<Self>
    where
        W: io::Write,
    {
        let then = Instant::now();
        let this = Self::create(elements);
        let elapsed = then.elapsed();
        let row = RecordRow {
            mode: mode_name,
            size: elements.len(),
            operation: oper_name,
            collection: Self::NAME,
            nanoseconds: elapsed.as_nanos(),
        };
        csv_writer.serialize(row)?;
        Ok(this)
    }

    fn record_find<W>(
        &self,
        target_elements: &[Element],
        all_elements: &[Element],
        mode_name: &str,
        oper_name: &str,
        csv_writer: &mut csv::Writer<W>,
    ) -> io::Result<bool>
    where
        W: io::Write,
    {
        let then = Instant::now();
        let mut found_all = true;
        for &element in target_elements {
            found_all &= self.find(element);
        }
        let elapsed = then.elapsed();
        let row = RecordRow {
            mode: mode_name,
            size: all_elements.len(),
            operation: oper_name,
            collection: Self::NAME,
            nanoseconds: elapsed.as_nanos(),
        };
        csv_writer.serialize(row)?;
        Ok(found_all)
    }

    fn record_inc_less_than<W>(
        &mut self,
        target_elements: &[Element],
        all_elements: &[Element],
        mode_name: &str,
        oper_name: &str,
        csv_writer: &mut csv::Writer<W>,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        let then = Instant::now();
        for &element in target_elements {
            self.inc_less_than(element);
        }
        let elapsed = then.elapsed();
        let row = RecordRow {
            mode: mode_name,
            size: all_elements.len(),
            operation: oper_name,
            collection: Self::NAME,
            nanoseconds: elapsed.as_nanos(),
        };
        csv_writer.serialize(row)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SortedArray {
    array_impl: ArrayImpl,
}

impl Collection for SortedArray {
    const NAME: &'static str = "sorted-array";

    fn create(elements: &[Element]) -> Self {
        let mut array = ArrayImpl::empty();
        for &element in elements {
            array.append(element);
        }
        array.sort();
        Self { array_impl: array }
    }

    fn find(&self, element: Element) -> bool {
        self.array_impl.find_sorted(element)
    }

    fn inc_less_than(&mut self, element: Element) {
        self.array_impl.inc_less_than_sorted(element)
    }
}

#[derive(Debug, Clone)]
pub struct GoodLocalArray {
    array_impl: ArrayImpl,
}

impl Collection for GoodLocalArray {
    const NAME: &'static str = "good-local-array";

    fn create(elements: &[Element]) -> Self {
        let mut array = ArrayImpl::empty();
        for &element in elements {
            array.append(element);
        }
        Self { array_impl: array }
    }

    fn find(&self, element: Element) -> bool {
        self.array_impl.find_good_local(element)
    }

    fn inc_less_than(&mut self, element: Element) {
        self.array_impl.inc_less_than_good_local(element)
    }
}

#[derive(Debug, Clone)]
pub struct BadLocalArray {
    array_impl: ArrayImpl,
}

impl Collection for BadLocalArray {
    const NAME: &'static str = "bad-local-array";

    fn create(elements: &[Element]) -> Self {
        let mut array = ArrayImpl::empty();
        for &element in elements {
            array.append(element);
        }
        Self { array_impl: array }
    }

    fn find(&self, element: Element) -> bool {
        self.array_impl.find_bad_local(element)
    }

    fn inc_less_than(&mut self, element: Element) {
        self.array_impl.inc_less_than_bad_local(element)
    }
}

#[derive(Debug, Clone)]
pub struct WorseLocalArray {
    array_impl: ArrayImpl,
}

impl Collection for WorseLocalArray {
    const NAME: &'static str = "worse-local-array";

    fn create(elements: &[Element]) -> Self {
        let mut array = ArrayImpl::empty();
        for &element in elements {
            array.append(element);
        }
        Self { array_impl: array }
    }

    fn find(&self, element: Element) -> bool {
        self.array_impl.find_worse_local(element)
    }

    fn inc_less_than(&mut self, element: Element) {
        self.array_impl.inc_less_than_worse_local(element)
    }
}

#[derive(Debug, Clone)]
pub struct LinkedList {
    list_impl: LinkedListImpl,
}

impl Collection for LinkedList {
    const NAME: &'static str = "linked-list";

    fn create(elements: &[Element]) -> Self {
        let mut list = LinkedListImpl::empty();
        for &element in elements {
            list.prepend(element);
        }
        Self { list_impl: list }
    }

    fn find(&self, element: Element) -> bool {
        self.list_impl.find(element)
    }

    fn inc_less_than(&mut self, element: Element) {
        self.list_impl.inc_less_than(element)
    }
}

#[derive(Debug, Clone)]
pub struct WithOrderTree {
    tree_impl: TreeImpl,
}

impl Collection for WithOrderTree {
    const NAME: &'static str = "with-order-tree";

    fn create(elements: &[Element]) -> Self {
        let mut tree = TreeImpl::empty();
        for &element in elements {
            tree.insert(element);
        }
        Self { tree_impl: tree }
    }

    fn find(&self, element: Element) -> bool {
        self.tree_impl.find_with_order(element)
    }

    fn inc_less_than(&mut self, element: Element) {
        self.tree_impl.inc_less_than_with_order(element)
    }
}

#[derive(Debug, Clone)]
pub struct WithoutOrderTree {
    tree_impl: TreeImpl,
}

impl Collection for WithoutOrderTree {
    const NAME: &'static str = "without-order-tree";

    fn create(elements: &[Element]) -> Self {
        let mut tree = TreeImpl::empty();
        for &element in elements {
            tree.insert(element);
        }
        Self { tree_impl: tree }
    }

    fn find(&self, element: Element) -> bool {
        self.tree_impl.find_without_order(element)
    }

    fn inc_less_than(&mut self, element: Element) {
        self.tree_impl.inc_less_than_without_order(element)
    }
}
