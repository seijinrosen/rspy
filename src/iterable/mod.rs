mod for_array;
mod for_str;
mod for_vec;

use std::{iter::Zip, ops::Range};

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
