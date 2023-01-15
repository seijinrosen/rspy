use crate::Reversed;

impl<T> Reversed for Vec<T>
where
    T: Clone,
{
    type Item = Vec<T>;

    fn reversed(&self) -> Self::Item {
        let mut result = self.clone();
        result.reverse();
        result
    }
}

impl<T> Reversed for &Vec<T>
where
    T: Clone,
{
    type Item = Vec<T>;

    fn reversed(&self) -> Self::Item {
        let mut result = self.to_vec();
        result.reverse();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn vec() {
        let vec = vec![1, 2, 3, 4, 5];
        let result = vec.reversed();
        assert_eq!(result, [5, 4, 3, 2, 1]);
    }
}
