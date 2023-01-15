use std::{iter::Zip, ops::Range, slice::Iter, str::Chars};

/// `enumerate`, `sorted` を実装するためのトレイト。
pub trait Iterable<'a> {
    type Item: Iterator;
    type SortedItems;

    /// インデックス付きでイテレートします。
    ///
    /// Python とは異なり、`start` は 0 の場合でも必須です。
    fn enumerate(&'a self, start: i32) -> Zip<Range<i32>, Self::Item>;

    fn sorted(&self) -> Self::SortedItems;
}

fn inner_sort_str(s: &str) -> String {
    let mut vec: Vec<char> = s.chars().collect();
    vec.sort();
    vec.iter().collect()
}

impl<'a> Iterable<'a> for str {
    type Item = Chars<'a>;
    type SortedItems = String;

    /// # Examples
    ///
    /// ```
    /// use rspy::Iterable;
    ///
    /// let mut index_vec = vec![];
    /// let mut char_vec = vec![];
    ///
    /// for (i, ch) in "abcde".enumerate(-3) {
    ///     index_vec.push(i);
    ///     char_vec.push(ch);
    /// }
    ///
    /// assert_eq!(index_vec, [-3, -2, -1, 0, 1]);
    /// assert_eq!(char_vec, ['a', 'b', 'c', 'd', 'e']);
    /// ```
    fn enumerate(&'a self, start: i32) -> Zip<Range<i32>, Self::Item> {
        (start..start + self.len() as i32).zip(self.chars())
    }

    fn sorted(&self) -> Self::SortedItems {
        inner_sort_str(self)
    }
}

impl<'a> Iterable<'a> for &str {
    type Item = Chars<'a>;
    type SortedItems = String;

    fn enumerate(&'a self, start: i32) -> Zip<Range<i32>, Self::Item> {
        (start..start + self.len() as i32).zip(self.chars())
    }

    fn sorted(&self) -> Self::SortedItems {
        inner_sort_str(self)
    }
}

impl<'a> Iterable<'a> for String {
    type Item = Chars<'a>;
    type SortedItems = String;

    fn enumerate(&'a self, start: i32) -> Zip<Range<i32>, Self::Item> {
        (start..start + self.len() as i32).zip(self.chars())
    }

    fn sorted(&self) -> Self::SortedItems {
        inner_sort_str(self)
    }
}

impl<'a, T: 'a> Iterable<'a> for Vec<T>
where
    T: Clone + Ord,
{
    type Item = Iter<'a, T>;
    type SortedItems = Vec<T>;

    fn enumerate(&'a self, start: i32) -> Zip<Range<i32>, Self::Item> {
        (start..start + self.len() as i32).zip(self.iter())
    }

    fn sorted(&self) -> Self::SortedItems {
        let mut result = self.to_vec();
        result.sort();
        result
    }
}

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
    fn str_enumerate_works() {
        let mut index_vec = vec![];
        let mut char_vec = vec![];

        for (i, ch) in "abcde".enumerate(-3) {
            index_vec.push(i);
            char_vec.push(ch);
        }

        assert_eq!(index_vec, [-3, -2, -1, 0, 1]);
        assert_eq!(char_vec, ['a', 'b', 'c', 'd', 'e']);
    }

    #[test]
    fn str_enumerate_works2() {
        let mut index_vec = vec![];
        let mut char_vec = vec![];

        for (i, ch) in "abcde".enumerate(3) {
            index_vec.push(i);
            char_vec.push(ch);
        }

        assert_eq!(index_vec, [3, 4, 5, 6, 7]);
        assert_eq!(char_vec, ['a', 'b', 'c', 'd', 'e']);
    }

    #[test]
    fn string_enumerate_works() {
        let mut index_vec = vec![];
        let mut char_vec = vec![];

        let s = "abcde".to_string();

        for (i, ch) in s.enumerate(-3) {
            index_vec.push(i);
            char_vec.push(ch);
        }

        assert_eq!(index_vec, [-3, -2, -1, 0, 1]);
        assert_eq!(char_vec, ['a', 'b', 'c', 'd', 'e']);
    }

    #[test]
    fn string_enumerate_works2() {
        let mut index_vec = vec![];
        let mut char_vec = vec![];

        let s = "abcde".to_string();

        for (i, ch) in s.enumerate(3) {
            index_vec.push(i);
            char_vec.push(ch);
        }

        assert_eq!(index_vec, [3, 4, 5, 6, 7]);
        assert_eq!(char_vec, ['a', 'b', 'c', 'd', 'e']);
    }

    #[test]
    fn vec_enumerate_works() {
        let mut index_vec = vec![];
        let mut int_vec = vec![];

        let vec = vec![100, -100, 20, 50, -1000];

        for (i, v) in vec.enumerate(-3) {
            index_vec.push(i);
            int_vec.push(v);
        }

        assert_eq!(index_vec, [-3, -2, -1, 0, 1]);
        for (i, v) in vec.iter().enumerate() {
            assert_eq!(int_vec[i], v);
        }
    }

    #[test]
    fn vec_enumerate_works2() {
        let mut index_vec = vec![];
        let mut int_vec = vec![];

        let vec = vec![100, -100, 20, 50, -1000];

        for (i, v) in vec.enumerate(3) {
            index_vec.push(i);
            int_vec.push(v);
        }

        assert_eq!(index_vec, [3, 4, 5, 6, 7]);
        for (i, v) in vec.iter().enumerate() {
            assert_eq!(int_vec[i], v);
        }
    }

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
}
