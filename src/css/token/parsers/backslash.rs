/**
 * Handles \ in main code stream handling
 *
 * Can be:
 *  - Ident like token
 *  - Parse Error (Delim token)
 */
use super::super::check::{is_valid_escape, next_char_equals};
use super::super::consume;
use super::super::error::ParseError;
use super::ParseResult;
use std::iter::Peekable;
use std::str::Chars;

pub fn parse(points: &mut Peekable<Chars>, position: &mut i32) -> ParseResult {
    if next_char_equals(points, &'\\') {
        *position += 1;
        points.next();

        if is_valid_escape(points) {
            match consume::ident_like_token(points, position) {
                Ok(token) => Ok(Some(token)),
                Err(e) => Err(e),
            }
        } else {
            // Parse Error
            Err(ParseError {
                error_text: "Unexpected token \\",
                at: *position,
                token: None,
            })
        }
    } else {
        Ok(None)
    }
}
