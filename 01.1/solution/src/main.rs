extern crate solution;
use solution::CodeStyle;
use std::str::FromStr;

fn main() {
    let code_style = CodeStyle::from_str("some_var");
    println!("{:?}", code_style.unwrap());
}
