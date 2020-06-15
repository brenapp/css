pub mod comment;

use super::error::ParseError;
use super::tokens::CSSToken;
use std::str::Chars;

type ParseResult = Result<Option<CSSToken>, ParseError>;

// Looks ahead for some chars returns true if they are there and false otherwise
pub fn lookahead(iter: &Chars, chars: &[char]) -> bool {
    // Copy it to look ahead
    let mut iter = iter.clone().peekable();

    for ch in chars {
        if let Some(reference) = iter.next() {
            if reference != *ch {
                return true;
            }
        } else {
            return false;
        }
    }

    true
}

/**
 * Goes through all the parsers to parse the iterator at that point
 */
pub fn parse(iter: &mut Chars) -> ParseResult {
    // Comment
    match comment::parse(iter) {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(Some(token)),
            None => (),
        },
    };

    Ok(None)
}
