/**
 * Minus tokens
 * This can be a CDC token (like -->)
 * or it can be in a number (like { margin: -5px })
 *
 * Or it can be an identifier
 */
use super::super::check::{is_identifier, is_number, next_char_equals};
use super::super::consume;
use super::super::tokens::CSSToken;
use super::lookahead;
use super::ParseResult;
use std::iter::Peekable;
use std::str::Chars;

const CDC: [char; 3] = ['-', '-', '>'];

pub fn parse(points: &mut Peekable<Chars>, position: &mut i32) -> ParseResult {
    if next_char_equals(points, &'-') {
        // If the input stream starts with a number, reconsume the current input code point, consume a numeric token, and return it.
        if is_number(points) {
            match consume::numeric_token(points, position) {
                Ok(token) => Ok(Some(token)),
                Err(e) => Err(e),
            }

        // If it's a CDC return appropriately
        } else if lookahead(points, &CDC) {
            // Consume the CDO
            *position += 3;
            points.next();
            points.next();
            points.next();

            Ok(Some(CSSToken::CDC))
        } else if is_identifier(points) {
            match consume::ident_like_token(points, position) {
                Ok(token) => Ok(Some(token)),
                Err(e) => Err(e),
            }

        // Otherwise create a delimiter
        } else {
            *position += 1;
            let minus = points.next().unwrap();

            Ok(Some(CSSToken::Delim(minus)))
        }
    } else {
        Ok(None)
    }
}
