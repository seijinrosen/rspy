use std::{iter::Zip, ops::Range, str::Chars};

use crate::Iterable;

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

impl<'a> Iterable<'a> for &String {
    type Item = Chars<'a>;
    type SortedItems = String;

    fn enumerate(&'a self, start: i32) -> Zip<Range<i32>, Self::Item> {
        (start..start + self.len() as i32).zip(self.chars())
    }

    fn sorted(&self) -> Self::SortedItems {
        inner_sort_str(self)
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
    fn str_sorted_works() {
        let s = "cbdae";
        let result = s.sorted();
        assert_eq!(result, "abcde");
    }

    #[test]
    fn string_sorted_works() {
        let s = "cbdae".to_string();
        let result = s.sorted();
        assert_eq!(result, "abcde");
    }
}
