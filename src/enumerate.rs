use std::{slice::Iter, str::Chars};

pub struct PyEnumerate<I> {
    iter: I,
    count: i32,
}
impl<I> PyEnumerate<I> {
    fn new(iter: I, start: i32) -> PyEnumerate<I> {
        PyEnumerate { iter, count: start }
    }
}

impl<I> Iterator for PyEnumerate<I>
where
    I: Iterator,
{
    type Item = (i32, <I as Iterator>::Item);

    fn next(&mut self) -> Option<(i32, <I as Iterator>::Item)> {
        let a = self.iter.next()?;
        let i = self.count;
        self.count += 1;
        Some((i, a))
    }
}

/// `str` を拡張するためのトレイト。
pub trait PyString {
    /// インデックス付きでイテレートします。
    ///
    /// Python とは異なり、`start` は 0 の場合でも必須です。
    ///
    /// # Examples
    ///
    /// ```
    /// use rspy::PyString;
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
    fn enumerate(&self, start: i32) -> PyEnumerate<Chars>;
}

impl PyString for str {
    /// インデックス付きでイテレートします。
    fn enumerate(&self, start: i32) -> PyEnumerate<Chars> {
        PyEnumerate::new(self.chars(), start)
    }
}

pub trait PyList<T> {
    type Item;

    fn enumerate(&self, start: i32) -> PyEnumerate<Iter<T>>;
}

impl<T> PyList<T> for Vec<T> {
    type Item = T;

    fn enumerate(&self, start: i32) -> PyEnumerate<Iter<T>> {
        PyEnumerate::new(self.iter(), start)
    }
}

impl<T, const N: usize> PyList<T> for [T; N] {
    type Item = T;

    fn enumerate(&self, start: i32) -> PyEnumerate<Iter<T>> {
        PyEnumerate::new(self.iter(), start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn enumerate_works() {
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
}
