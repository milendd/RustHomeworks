use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CodeStyle {
    Underscored,
    Camelcased,
    ScreamingSnakecased,
    Unknown,
    Mixed
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CodeStyleError {
    EmptyString,
    InvalidFirstChar(char),
    InvalidChar(char),
}

impl FromStr for CodeStyle {
    type Err = CodeStyleError;

    fn from_str(_identifier: &str) -> Result<CodeStyle, CodeStyleError> {
        unimplemented!()
    }
}