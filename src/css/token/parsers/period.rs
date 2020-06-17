/**
 * Handles FULL-STOP
 *
 * Can Be:
 * Number
 * Delimiter
 */
use super::super::check::{is_number, next_char_equals};
use super::super::consume;
use super::super::tokens::CSSToken;
use super::ParseResult;
use std::iter::Peekable;
use std::str::Chars;

pub fn parse(points: &mut Peekable<Chars>, position: &mut i32) -> ParseResult {
    if next_char_equals(points, &'.') {
        // If the input stream starts with a number, reconsume the current input code point, consume a numeric token, and return it.
        if is_number(points) {
            match consume::numeric_token(points, position) {
                Ok(token) => Ok(Some(token)),
                Err(e) => Err(e),
            }

        // Otherwise return a delimiter
        } else {
            *position += 1;
            let period = points.next().unwrap();

            Ok(Some(CSSToken::Delim(period)))
        }
    } else {
        Ok(None)
    }
}
