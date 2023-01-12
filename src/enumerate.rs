use std::str::Chars;

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
}
