extern crate alloc;

use alloc::vec::Vec;
use std::cmp::Ordering;

type VecIntoIter<T> = alloc::vec::IntoIter<T>;

pub trait SortedByExt: Iterator {
    fn sorted_by<F>(self, cmp: F) -> VecIntoIter<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        /*
        This whole thing is stolen from itertools, with a slight modification due to clippy lint.
        In itertools Vec::from_iter() is used but we get the aforementioned clippy complaint
        and it recommends a .collect().
        But with collect rustc gets confused and requires type annotations on the var or on
        the collect() call.
        */
        let mut v = self.collect::<Vec<Self::Item>>();
        v.sort_by(cmp);
        v.into_iter()
    }
}

impl<I: Iterator> SortedByExt for I {}
