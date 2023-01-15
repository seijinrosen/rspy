use super::Reversed;

impl Reversed for str {
    type Item = String;

    fn reversed(&self) -> Self::Item {
        self.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn str() {
        let s = "abcde";
        let result = s.reversed();
        assert_eq!(result, "edcba");
    }

    #[test]
    fn string() {
        let s = "abcde".to_string();
        let result = s.reversed();
        assert_eq!(result, "edcba");
    }
}
