use std::{iter::Zip, ops::Range, slice::Iter, str::Chars};

/// `enumerate` を実装するためのトレイト。
pub trait Enumerator<'a> {
    type Item;

    /// インデックス付きでイテレートします。
    ///
    /// Python とは異なり、`start` は 0 の場合でも必須です。
    fn enumerate(&'a self, start: i32) -> Zip<Range<i32>, Self::Item>;
}

impl<'a> Enumerator<'a> for str {
    type Item = Chars<'a>;

    /// # Examples
    ///
    /// ```
    /// use rspy::Enumerator;
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
        (start..self.len() as i32).zip(self.chars())
    }
}

impl<'a> Enumerator<'a> for String {
    type Item = Chars<'a>;

    fn enumerate(&'a self, start: i32) -> Zip<Range<i32>, Self::Item> {
        (start..self.len() as i32).zip(self.chars())
    }
}

impl<'a, T: 'a> Enumerator<'a> for Vec<T> {
    type Item = Iter<'a, T>;

    fn enumerate(&'a self, start: i32) -> Zip<Range<i32>, Self::Item> {
        (start..self.len() as i32).zip(self.iter())
    }
}

impl<'a, T: 'a, const N: usize> Enumerator<'a> for [T; N] {
    type Item = Iter<'a, T>;

    fn enumerate(&'a self, start: i32) -> Zip<Range<i32>, Self::Item> {
        (start..self.len() as i32).zip(self.iter())
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
}
