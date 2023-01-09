pub mod string;

use std::io::{self, BufRead, Write};

pub fn input(prompt: &str) -> String {
    input_inner_writer(io::stdout(), prompt);
    input_inner_reader(io::stdin().lock())
}

fn input_inner_writer(mut writer: impl Write, prompt: &str) {
    write!(&mut writer, "{}", prompt).unwrap();
    writer.flush().unwrap();
}

fn input_inner_reader(mut reader: impl BufRead) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    buf.pop();
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::string::{ASCII_LOWERCASE, ASCII_UPPERCASE};

    #[test]
    fn input_inner_writer_works() {
        let prompt = "プロンプトメッセージ: ";
        let mut output = Vec::new();
        input_inner_writer(&mut output, prompt);
        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, prompt);
    }

    #[test]
    fn input_inner_reader_works() {
        // https://stackoverflow.com/a/72187752
        // https://stackoverflow.com/a/28370712
        let user_input = b"I'm George\n";
        let result = input_inner_reader(&user_input[..]);
        assert_eq!(result, "I'm George");
    }

    #[test]
    fn ascii_lowercase_exists() {
        let result = ASCII_LOWERCASE;
        assert_eq!(result, "abcdefghijklmnopqrstuvwxyz");
    }

    #[test]
    fn ascii_uppercase_exists() {
        let result = ASCII_UPPERCASE;
        assert_eq!(result, "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
}
