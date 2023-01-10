use rspy::string::{ASCII_LOWERCASE, ASCII_UPPERCASE};

#[test]
fn ascii_lowercase_exists() {
    assert_eq!(ASCII_LOWERCASE, "abcdefghijklmnopqrstuvwxyz");
}

#[test]
fn ascii_uppercase_exists() {
    assert_eq!(ASCII_UPPERCASE, "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
}
