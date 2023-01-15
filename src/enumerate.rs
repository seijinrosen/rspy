use std::{iter::Zip, ops::Range};

use crate::Iterable;

/// インデックス付きでイテレートします。
///
/// Python とは異なり、`start` は 0 の場合でも必須です。
///
/// # Examples
///
/// ```
/// use rspy::enumerate;
///
/// let mut index_vec = vec![];
/// let mut char_vec = vec![];
///
/// let s = "abcde";
///
/// for (i, ch) in enumerate(&s, -10) {
///     index_vec.push(i);
///     char_vec.push(ch);
/// }
///
/// assert_eq!(index_vec, [-10, -9, -8, -7, -6]);
/// for (i, ch) in s.chars().enumerate() {
///     assert_eq!(char_vec[i], ch);
/// }
/// ```
pub fn enumerate<'a, T>(iterable: &'a T, start: i32) -> Zip<Range<i32>, T::Item>
where
    T: Iterable<'a>,
{
    iterable.enumerate(start)
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn bare_enumerate() {
        let mut index_vec = vec![];
        let mut int_vec = vec![];

        let arr = [100, -100, 20, 50, -1000];

        for (i, v) in enumerate(&arr, 100) {
            index_vec.push(i);
            int_vec.push(v);
        }

        assert_eq!(index_vec, [100, 101, 102, 103, 104]);
        for (i, v) in arr.iter().enumerate() {
            assert_eq!(int_vec[i], v);
        }
    }

    #[test]
    fn bare_enumerate_str() {
        let mut index_vec = vec![];
        let mut char_vec = vec![];

        let s = "abcde";

        for (i, ch) in enumerate(&s, -10) {
            index_vec.push(i);
            char_vec.push(ch);
        }

        assert_eq!(index_vec, [-10, -9, -8, -7, -6]);
        for (i, ch) in s.chars().enumerate() {
            assert_eq!(char_vec[i], ch);
        }
    }
}
