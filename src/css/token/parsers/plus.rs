/**
 * Handles + tokens
 * This can be a delimiter (like a + p {})
 * or it can be in a number (like { margin: +5px })
 */
use super::super::check::{equal, is_number};
use super::super::consume;
use super::super::tokens::CSSToken;
use super::ParseResult;
use std::iter::Peekable;
use std::str::Chars;

pub fn parse(points: &mut Peekable<Chars>, position: &mut i32) -> ParseResult {
    if let Some(ch) = points.peek() {
        if equal(ch, &'+') {
            let numeric = is_number(points);

            if numeric {
                match consume::numeric_token(points, position) {
                    Ok(token) => Ok(Some(token)),
                    Err(e) => Err(e),
                }
            } else {
                // Consume the plus
                *position += 1;
                let plus = points.next().unwrap();

                Ok(Some(CSSToken::Delim(plus)))
            }
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
