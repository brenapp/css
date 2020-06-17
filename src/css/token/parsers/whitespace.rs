/**
 * Parses CSS Whitespace
 *
 * Returns a Result:
 *  Ok(Some) => If it matched some whitespace
 *  Ok(None) => No whitespace match
 */
use super::super::check::is_whitespace;
use super::super::consume;
use super::super::tokens::CSSToken;
use super::ParseResult;
use std::iter::Peekable;
use std::str::Chars;

pub fn parse(points: &mut Peekable<Chars>, position: &mut i32) -> ParseResult {
    // Check if we're at a whitespace character
    let next = points.peek();

    match next {
        Some(ch) if is_whitespace(&ch) => {
            consume::whitespace(points, position);
            Ok(Some(CSSToken::Whitespace))
        }
        _ => Ok(None),
    }
}
