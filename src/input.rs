use std::io::{self, BufRead, Result, Write};

/// `prompt` を表示して、ユーザーに入力を促します。入力された文字列は、変数に保存されることが期待されます。
///
/// # Examples
///
/// ```
/// let user_input = rspy::input("type here: ");
/// // "type here: " が端末に表示されます。
/// // ユーザーが入力した文字列は、`user_input` に保存されます。
///
/// let user_input = rspy::input("");
/// // Python とは異なり、引数は必須です。
/// // `prompt` が空白で良い場合は、空文字列を渡してください。
/// ```
pub fn input(prompt: &str) -> String {
    input_inner(prompt, io::stdout(), io::stdin().lock()).unwrap()
}

fn input_inner(prompt: &str, writer: impl Write, reader: impl BufRead) -> Result<String> {
    input_inner_writer(writer, prompt)?;
    input_inner_reader(reader)
}

fn input_inner_writer(mut writer: impl Write, prompt: &str) -> Result<()> {
    write!(&mut writer, "{prompt}")?;
    writer.flush()?;
    Ok(())
}

fn input_inner_reader(mut reader: impl BufRead) -> Result<String> {
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    buf.pop();
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    use crate as rspy;

    #[test]
    fn input_inner_writer_works() {
        let prompt = "プロンプトメッセージ: ";
        let mut output = Vec::new();
        input_inner_writer(&mut output, prompt).unwrap();
        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "プロンプトメッセージ: ");
    }

    #[test]
    fn input_inner_reader_works() {
        // https://stackoverflow.com/a/72187752
        // https://stackoverflow.com/a/28370712
        let user_input = b"I'm George\n";
        let result = input_inner_reader(&user_input[..]);
        assert_eq!(result.unwrap(), "I'm George");
    }

    #[test]
    fn input_inner_works() {
        let prompt = "プロンプトメッセージ: ";
        let mut output = Vec::new();
        let user_input = b"I'm George\n";

        let result = input_inner(prompt, &mut output, &user_input[..]);

        assert_eq!(String::from_utf8(output).unwrap(), "プロンプトメッセージ: ");
        assert_eq!(result.unwrap(), "I'm George");
    }

    #[test]
    #[ignore = "since this is for doctest"]
    #[allow(unused_variables)]
    fn input_doc_test() {
        let user_input = rspy::input("type here: ");
        let user_input = rspy::input("");
    }
}
