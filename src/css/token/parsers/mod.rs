// Parsers
pub mod at;
pub mod backslash;
pub mod comment;
pub mod hash;
pub mod lt;
pub mod minus;
pub mod period;
pub mod plus;
pub mod single_char;
pub mod string;
pub mod whitespace;

use super::check::is_name_start_code_point;
use super::consume::{ident_like_token, numeric_token};
use super::error::ParseError;
use super::tokens::CSSToken;
use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

type ParseResult = Result<Option<CSSToken>, ParseError>;

// Looks ahead for some chars returns true if they are there and false otherwise
pub fn lookahead(iter: &mut Peekable<Chars>, chars: &[char]) -> bool {
    // Test the first character before we go and copy the whole iterator
    let peek = iter.peek();
    if peek.is_none() {
        return false;
    };

    if peek.is_some() && peek.unwrap().partial_cmp(&chars[0]) != Some(Ordering::Equal) {
        return false;
    }

    // Now that we've matched the first char we need copy the iterator to compare
    let mut cmp = iter.clone();

    for char in chars {
        let next = cmp.next();

        match next {
            None => return false,
            Some(ch) => {
                if ch.partial_cmp(&char) != Some(Ordering::Equal) {
                    return false;
                }
            }
        }
    }

    true
}

/**
 * Goes through all the parsers to parse the iterator at that point
 */
pub fn parse(iter: &mut Peekable<Chars>, position: &mut i32) -> Result<CSSToken, ParseError> {
    // Match Parsers
    // Comment
    match comment::parse(iter, position) {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(token),
            None => (),
        },
    };

    // Whitespace
    match whitespace::parse(iter, position) {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(token),
            None => (),
        },
    };

    // String Token (Double Quotes)
    match string::parse(iter, position, '"') {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(token),
            None => (),
        },
    };

    // NUMBER SIGN (#)
    match hash::parse(iter, position) {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(token),
            None => (),
        },
    };

    // String Token (Single Quotes)
    match string::parse(iter, position, '\'') {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(token),
            None => (),
        },
    };

    // Left Paren
    match single_char::parse(iter, position, '(', CSSToken::LeftParentheses) {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(token),
            None => (),
        },
    };

    // Right Paren
    match single_char::parse(iter, position, ')', CSSToken::RightParentheses) {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(token),
            None => (),
        },
    };

    // Plus symbol
    match plus::parse(iter, position) {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(token),
            None => (),
        },
    };

    // Comma Token
    match single_char::parse(iter, position, ',', CSSToken::Comma) {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(token),
            None => (),
        },
    };

    // Minus symbol
    match minus::parse(iter, position) {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(token),
            None => (),
        },
    };

    // Full Stop
    match period::parse(iter, position) {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(token),
            None => (),
        },
    };

    // Colon
    match single_char::parse(iter, position, ':', CSSToken::Colon) {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(token),
            None => (),
        },
    };

    // Semicolon
    match single_char::parse(iter, position, ';', CSSToken::Semicolon) {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(token),
            None => (),
        },
    };

    // Less than <
    match lt::parse(iter, position) {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(token),
            None => (),
        },
    };

    // Commerical At @
    match at::parse(iter, position) {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(token),
            None => (),
        },
    };

    // Left Square Bracket
    match single_char::parse(iter, position, '[', CSSToken::LeftBracket) {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(token),
            None => (),
        },
    };

    // Backlash \
    match backslash::parse(iter, position) {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(token),
            None => (),
        },
    };

    // Right Square Bracket ]
    match single_char::parse(iter, position, ']', CSSToken::RightBracket) {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(token),
            None => (),
        },
    };

    // Left Curly Brace
    match single_char::parse(iter, position, '{', CSSToken::LeftBrace) {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(token),
            None => (),
        },
    };

    // Right Curly Brace
    match single_char::parse(iter, position, '}', CSSToken::RightBrace) {
        Err(e) => return Err(e),
        Ok(result) => match result {
            Some(token) => return Ok(token),
            None => (),
        },
    };

    // EOF
    if iter.peek().is_none() {
        return Ok(CSSToken::EOF);
    }

    // Simpler matches
    let ch = iter.peek().unwrap();

    // Digit
    if ch.is_ascii_digit() {
        numeric_token(iter, position)
    } else if is_name_start_code_point(*ch) {
        ident_like_token(iter, position)
    } else {
        let ch = iter.next().unwrap();
        Ok(CSSToken::Delim(ch))
    }
}
