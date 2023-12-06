use std::{
    cmp::{max, min},
    ops::Range,
};

pub trait RangeExt<T> {
    fn intersect(&self, other: &Range<T>) -> Range<T>;
}

impl<T: Ord + Copy> RangeExt<T> for Range<T> {
    fn intersect(&self, other: &Range<T>) -> Range<T> {
        max(self.start, other.start)..min(self.end, other.end)
    }
}
