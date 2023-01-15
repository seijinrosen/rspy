use std::{iter::Zip, ops::Range, slice::Iter};

use crate::Iterable;

impl<'a, T: 'a, const N: usize> Iterable<'a> for [T; N]
where
    T: Clone + Ord,
{
    type Item = Iter<'a, T>;
    type SortedItems = [T; N];

    fn enumerate(&'a self, start: i32) -> Zip<Range<i32>, Self::Item> {
        (start..start + self.len() as i32).zip(self.iter())
    }

    fn sorted(&self) -> Self::SortedItems {
        let mut result = self.clone();
        result.sort();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn arr_enumerate_works() {
        let mut index_vec = vec![];
        let mut int_vec = vec![];

        let arr = [100, -100, 20, 50, -1000];

        for (i, v) in arr.enumerate(-3) {
            index_vec.push(i);
            int_vec.push(v);
        }

        assert_eq!(index_vec, [-3, -2, -1, 0, 1]);
        for (i, v) in arr.iter().enumerate() {
            assert_eq!(int_vec[i], v);
        }
    }

    #[test]
    fn arr_enumerate_works2() {
        let mut index_vec = vec![];
        let mut int_vec = vec![];

        let arr = [100, -100, 20, 50, -1000];

        for (i, v) in arr.enumerate(3) {
            index_vec.push(i);
            int_vec.push(v);
        }

        assert_eq!(index_vec, [3, 4, 5, 6, 7]);
        for (i, v) in arr.iter().enumerate() {
            assert_eq!(int_vec[i], v);
        }
    }

    #[test]
    fn sorted_works() {
        let array = [100, 20, -2000, 60, 0];
        let result = array.sorted();
        assert_eq!(result, [-2000, 0, 20, 60, 100]);
    }
}
