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

    fn from_str(identifier: &str) -> Result<CodeStyle, CodeStyleError> {
        let identifier = identifier.trim();
        if identifier == "" {
            return Err(CodeStyleError::EmptyString);
        }

        let first_char = identifier.chars().next().unwrap();
        if !first_char.is_alphabetic() {
            return Err(CodeStyleError::InvalidFirstChar(first_char))
        }

        let mut contains_underscore = false;
        let contains_lower = false;
        let contains_upper = false;
        for (_i, c) in identifier.chars().enumerate() {
            if c == '_' {
                contains_underscore = true;
            }
            else if !c.is_alphanumeric() {
                return Err(CodeStyleError::InvalidChar(c));
            }
        }

        println!("{}{}{}", contains_underscore, contains_lower, contains_upper);

        Ok(CodeStyle::Underscored)
    }
}