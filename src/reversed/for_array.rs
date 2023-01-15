use crate::Reversed;

impl<T, const N: usize> Reversed for [T; N]
where
    T: Clone,
{
    type Item = [T; N];

    fn reversed(&self) -> Self::Item {
        let mut result = self.clone();
        result.reverse();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn array() {
        let array = [1, 2, 3, 4, 5];
        let result = array.reversed();
        assert_eq!(result, [5, 4, 3, 2, 1]);
    }
}
