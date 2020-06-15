/**
 * CSS Parse Error Type
 */
use super::tokens;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub token: tokens::CSSToken,
    pub error_text: &'static str,
    pub at: i32,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error_text)
    }
}
