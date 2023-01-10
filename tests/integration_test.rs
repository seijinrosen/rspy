mod common;

use pretty_assertions::assert_eq;

#[test]
#[ignore = "since stdin is required"]
fn try_integration_test() {
    common::setup();
    let result = rspy::input("type \"ttt\": ");
    assert_eq!(result, "ttt");
}
