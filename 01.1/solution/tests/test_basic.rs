extern crate solution;
use solution::*;
use std::str::FromStr;

#[test]
fn test_basic() {
    let code_style = CodeStyle::from_str(" some_var ");
    assert_eq!(code_style.unwrap(), CodeStyle::Underscored);

    let code_style = CodeStyle::from_str("some_var");
    assert_eq!(code_style.unwrap(), CodeStyle::Underscored);
}

#[test]
fn test_basic_2() {
    let code_style = "some_var".parse::<CodeStyle>();
    assert_eq!(code_style.unwrap(), CodeStyle::Underscored);
}