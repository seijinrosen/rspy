use std::io::{self, BufRead, Write};

pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    input_inner(io::stdin().lock())
}

fn input_inner(mut reader: impl BufRead) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    buf.pop();
    buf
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn input_inner_works() {
        // https://stackoverflow.com/a/72187752
        // https://stackoverflow.com/a/28370712
        let user_input = b"I'm George\n";
        let result = input_inner(&user_input[..]);
        assert_eq!(result, "I'm George");
    }
}
