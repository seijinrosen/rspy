use crate::Iterable;

/// ソートします。
///
/// 降順は `iterable.sorted().reversed()` を使用してください。
///
/// # Examples
///
/// ```
/// use rspy::sorted;
///
/// let vec = vec![100, 20, -2000, 60, 0];
/// let result = sorted(vec);
/// assert_eq!(result, [-2000, 0, 20, 60, 100]);
/// ```
pub fn sorted<'a, T>(iterable: T) -> T::SortedItems
where
    T: Iterable<'a>,
{
    iterable.sorted()
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn str() {
        let s = "cbdae";
        let result = sorted(s);
        assert_eq!(result, "abcde");
    }

    #[test]
    fn string() {
        let s = "cbdae".to_string();
        let result = sorted(s);
        assert_eq!(result, "abcde");
    }

    #[test]
    fn vec() {
        let vec = vec![100, 20, -2000, 60, 0];
        let result = sorted(vec);
        assert_eq!(result, [-2000, 0, 20, 60, 100]);
    }

    #[test]
    fn array() {
        let array = [100, 20, -2000, 60, 0];
        let result = sorted(array);
        assert_eq!(result, [-2000, 0, 20, 60, 100]);
    }
}
