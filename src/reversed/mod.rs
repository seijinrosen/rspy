mod for_array;
mod for_str;
mod for_vec;

/// `reversed` を実装するトレイト。
pub trait Reversed {
    type Item;

    /// 逆順にします。
    ///
    /// # Examples
    ///
    /// ```
    /// use rspy::Reversed;
    ///
    /// let s = "abcde";
    /// let result = s.reversed();
    /// assert_eq!(result, "edcba");
    /// ```
    fn reversed(&self) -> Self::Item;
}

/// 逆順にします。
///
/// # Examples
///
/// ```
/// use rspy::reversed;
///
/// let vec = vec![1, 2, 3, 4, 5];
/// let result = reversed(vec);
/// assert_eq!(result, [5, 4, 3, 2, 1]);
/// ```
pub fn reversed<T>(reversible: T) -> T::Item
where
    T: Reversed,
{
    reversible.reversed()
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn str() {
        let s = "abcde";
        let result = reversed(s);
        assert_eq!(result, "edcba");
    }

    #[test]
    fn string() {
        let s = "abcde".to_string();
        let result = reversed(s);
        assert_eq!(result, "edcba");
    }

    #[test]
    fn array() {
        let array = [1, 2, 3, 4, 5];
        let result = reversed(array);
        assert_eq!(result, [5, 4, 3, 2, 1]);
    }

    #[test]
    fn vec() {
        let vec = vec![1, 2, 3, 4, 5];
        let result = reversed(vec);
        assert_eq!(result, [5, 4, 3, 2, 1]);
    }
}
